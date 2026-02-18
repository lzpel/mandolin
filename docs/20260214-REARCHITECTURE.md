CONSULT.md(下記)を元に位置からこのプロジェクトを再構築してください

make testと入力するとopenapiディレクトリの~.yamlまたは~.jsonを読み込み、同名の単一ソース.rsを生成し、それがコンパイルできることを確認してください

すべての関数やテンプレートのマクロに日本語コメントで意味を教えてください

# Mandolin リアーキテクチャ提案

## 現状の課題

### 1. `jp_list` (フラットリスト) アプローチの複雑さ

現在の設計では、OpenAPI仕様全体を `Vec<(String, Value)>` に展開し、JSON Pointer をキーとするフラットリストを構築している。
この設計に起因して以下の補助機構が必要になっている:

- `include_ref` / `include_pointer` フィルタ ($ref 解決)
- `ls` / `ls_recursive` / `ls_operation` / `ls_schema` 関数 (フラットリストの検索)
- `jp_list()` 自体の再帰構築ロジック

**本来、テンプレートエンジンに渡すデータが構造化されていれば、これらは全て不要。**

### 2. Rust側に OpenAPI の構造知識が漏れている

`function.rs` の `ls()` が HTTP メソッド一覧を持っていたり、`openapiv3::Schema` で型判定していたりと、
本来テンプレート側で行うべきドメインロジックが Rust に入り込んでいる。

### 3. 文字列操作の手動実装

`filter.rs` の `to_snake_case` / `to_pascal_case` / `to_camel_case` が手動実装されている。
`heck` クレートで 1 行ずつに置き換え可能。

### 4. 共有可変状態 (`Arc<Mutex<>>`)

`schema_push` / `schema_drain` のために `Arc<Mutex<HashMap>>` を使用している。
テンプレートレンダリングは単一スレッドだが、`Environment` が `Send + Sync` を要求するためこうなっている。
ロジック自体は必要だが、もっと整理できる。

---

## 設計方針

> **Rust は「データの準備」だけ行い、「コードの組み立て」は全てテンプレートに任せる**

具体的には:

1. **`openapiv3::OpenAPI` を公開 API として維持** → 型安全性・バリデーションを保つ
2. **$ref を事前解決** → テンプレートは解決済みのオブジェクトを直接操作する
3. **openapiv3 を拡張する形で解決済みデータを持つ** → `#[serde(flatten)]` パターン
4. **テンプレートが OpenAPI 構造を直接走査** → `ls_*` 関数群を廃止
5. **Rust のフィルタは言語機能のみ** → ケース変換・正規表現など、Jinja では困難なものだけ

---

## 提案するファイル構成

```
src/
  lib.rs           公開API: environment() のみ (~50行)
  main.rs          CLI (現状とほぼ同じ)
  resolve.rs       openapiv3 拡張 + $ref 事前解決 (~80行)  ← 新規
  filter.rs        最小限のフィルタ (~15行)
  schema_cache.rs  スキーマ重複排除 (~40行)  ← function.rs から分離・簡素化
  templates.rs     ビルド時生成 (変更なし)
```

**削除されるもの:**
- `function.rs` (全体)
- `JpUnit` / `JpList` 型
- `jp_list()` 関数
- `include_ref` / `include_pointer` フィルタ
- `ls` / `ls_recursive` / `ls_operation` / `ls_schema` 関数
- `LsMode` enum

---

## Rust 側の詳細

### `resolve.rs` — openapiv3 の拡張 + $ref 事前解決

`openapiv3::OpenAPI` を `#[serde(flatten)]` で拡張し、$ref 解決済みデータを並走させる:

```rust
use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

/// openapiv3::OpenAPI を拡張した型
/// 型付き API は base 経由で使い、テンプレートには resolved を渡す
#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedOpenAPI {
    #[serde(flatten)]
    pub base: OpenAPI,
}

impl ResolvedOpenAPI {
    pub fn new(mut spec: OpenAPI) -> Self {
        // 型付き API でデフォルトサーバ追加
        if spec.servers.is_empty() {
            spec.servers.push(openapiv3::Server {
                url: "/api".to_string(),
                description: Some("Default server added by mandolin".to_string()),
                ..Default::default()
            });
        }
        Self { base: spec }
    }

    /// テンプレート用に $ref を全て事前解決した JSON Value を返す
    pub fn resolved_value(&self) -> Value {
        let mut value = serde_json::to_value(&self.base).unwrap();
        resolve_recursive(&mut value, &value.clone(), &mut HashSet::new());
        value
    }
}

impl From<OpenAPI> for ResolvedOpenAPI {
    fn from(spec: OpenAPI) -> Self {
        Self::new(spec)
    }
}

// --- $ref 解決 (serde_json::Value レベル) ---

fn resolve_recursive(node: &mut Value, root: &Value, visiting: &mut HashSet<String>) {
    match node {
        Value::Object(map) => {
            if let Some(Value::String(ref_path)) = map.get("$ref") {
                if let Some(resolved) = lookup_pointer(root, ref_path) {
                    if !visiting.contains(ref_path) {
                        visiting.insert(ref_path.clone());
                        let mut resolved = resolved.clone();
                        resolve_recursive(&mut resolved, root, visiting);
                        visiting.remove(ref_path);
                        *node = resolved;
                        return;
                    }
                    // 循環参照はそのまま残す（テンプレート側で対処）
                }
            }
            for value in map.values_mut() {
                resolve_recursive(value, root, visiting);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                resolve_recursive(item, root, visiting);
            }
        }
        _ => {}
    }
}

fn lookup_pointer(root: &Value, ref_path: &str) -> Option<&Value> {
    let path = ref_path.strip_prefix("#/")?;
    let mut current = root;
    for segment in path.split('/') {
        let decoded = segment.replace("~1", "/").replace("~0", "~");
        current = current.get(&decoded)?;
    }
    Some(current)
}
```

**設計のポイント:**
- `ResolvedOpenAPI` は `openapiv3::OpenAPI` を `flatten` で内包する
- `base` フィールド経由で型付き API がそのまま使える (`resolved.base.servers` 等)
- 将来フィールドを追加したくなったとき、`ResolvedOpenAPI` に足すだけ
- `From<OpenAPI>` で既存コードからの移行が容易
- $ref 解決は `serde_json::Value` レベルで行う (openapiv3 の型ツリーを歩くより圧倒的にシンプル)

### `lib.rs` — 公開 API

```rust
mod resolve;
mod filter;
mod schema_cache;
pub mod templates;

use openapiv3::OpenAPI;

pub fn environment(spec: OpenAPI) -> Result<minijinja::Environment<'static>, minijinja::Error> {
    // 1. openapiv3 を拡張して $ref を事前解決
    let resolved = resolve::ResolvedOpenAPI::new(spec);
    let value = resolved.resolved_value();

    // 2. テンプレート環境構築
    let mut env = minijinja::Environment::new();
    for [k, v] in templates::TEMPLATES {
        env.add_template(k, v)?;
    }

    // 3. 解決済みスペックをグローバル変数として設定
    env.add_global("spec", minijinja::Value::from_serialize(&value));

    // 4. フィルタ登録 (言語機能のみ)
    env.add_filter("to_snake_case", filter::to_snake_case);
    env.add_filter("to_pascal_case", filter::to_pascal_case);
    env.add_filter("to_camel_case", filter::to_camel_case);
    env.add_filter("re_replace", filter::re_replace);
    env.add_filter("encode", filter::encode);
    env.add_filter("decode", filter::decode);

    // 5. スキーマキャッシュ (インラインスキーマの重複排除用)
    let cache = schema_cache::SchemaCache::new();
    {
        let c = cache.clone();
        env.add_function("schema_push", move |pointer: &str, content: Option<&str>| {
            c.push(pointer, content)
        });
        let c = cache.clone();
        env.add_function("schema_drain", move || c.drain());
    }

    Ok(env)
}
```

**ポイント:**
- **公開 API のシグネチャは `openapiv3::OpenAPI` のまま** (破壊的変更なし)
- `jp_list` 構築が消え、代わりに `ResolvedOpenAPI::new()` + `resolved_value()` + `add_global` だけ
- フィルタ/関数の登録がシンプルに
- `include_ref`, `include_pointer`, `ls_*` の登録が全て不要に

### `filter.rs` — 最小限のフィルタ

```rust
use heck::{ToSnakeCase, ToPascalCase, ToLowerCamelCase};

pub fn to_snake_case(s: &str) -> String { s.to_snake_case() }
pub fn to_pascal_case(s: &str) -> String { s.to_pascal_case() }
pub fn to_camel_case(s: &str) -> String { s.to_lower_camel_case() }

pub fn encode(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}

pub fn decode(s: &str) -> String {
    s.replace("~1", "/").replace("~0", "~")
}

pub fn re_replace(content: &str, re: &str, replace: &str) -> String {
    regex::Regex::new(re).unwrap().replace_all(content, replace).to_string()
}
```

**現在の filter.rs (116行) → 約15行。**
`include_ref`, `include_pointer`, `split`, `to_case_words` が全て消える。

### `schema_cache.rs` — スキーマ重複排除

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SchemaCache(Arc<Mutex<HashMap<String, Option<String>>>>);

impl SchemaCache {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn push(&self, pointer: &str, content: Option<&str>) -> bool {
        let mut map = self.0.lock().unwrap();
        if let Some(v) = map.get(pointer) {
            if v.is_none() {
                map.insert(pointer.to_string(), content.map(|c| c.to_string()));
            }
            false
        } else {
            map.insert(pointer.to_string(), content.map(|c| c.to_string()));
            true
        }
    }

    pub fn drain(&self) -> HashMap<String, Option<String>> {
        let mut map = self.0.lock().unwrap();
        map.iter_mut()
            .filter_map(|(k, v)| v.take().map(|val| (k.clone(), Some(val))))
            .collect()
    }
}
```

**ロジックは同じだが、型として分離・カプセル化される。**
`Arc<Mutex<>>` は `Environment` が `Send + Sync` を要求するため残る。

### `main.rs` — CLI (変更なし)

公開 API が `openapiv3::OpenAPI` のままなので、CLI 側のコードは変更不要:

```rust
// 変更なし
let input_api: openapiv3::OpenAPI = serde_yaml::from_str(&input)?;
let env = mandolin::environment(input_api)?;
```

---

## テンプレート側の変更

### 最大の変更: `ls_*` の廃止 → 直接走査

**現在 (`rust_axum.template`):**
```jinja
{%- for pointer, operation in ls_operation() %}
    fn {{ OPERATION_NAME(pointer, operation) }}(...)
{%- endfor %}
```

**提案:**
```jinja
{%- set methods = ["get","put","post","delete","options","head","patch","trace"] %}
{%- for path_key, path_item in spec.paths|items %}
{%- for method in methods %}
{%- if path_item[method] is defined %}
{%- set pointer = "#/paths/" + path_key|encode + "/" + method %}
{%- set operation = path_item[method] %}
    fn {{ OPERATION_NAME(pointer, operation) }}(...)
{%- endif %}
{%- endfor %}
{%- endfor %}
```

これは `ls_operation()` の Rust 実装を、テンプレートの 3 行に置き換えたもの。
`methods` 配列は各テンプレートで定義する（言語ごとに必要なメソッドが異なる可能性があるため）。

### `include_ref` の廃止

**現在:**
```jinja
{%- for parameter in operation.parameters|include_ref %}
{%- with parameter=parameter|include_ref %}
```

**提案:**
```jinja
{%- for parameter in operation.parameters %}
{# $ref は事前解決済み。そのまま使える #}
```

### `include_pointer` の廃止

**現在:**
```jinja
{{ pointer|include_pointer }}
```

**提案:**
テンプレートが既に解決済みオブジェクトを持っているので、ほとんどの場合不要。
唯一の用途だった `"#"|include_pointer` (OpenAPI 全体の取得) は `spec` グローバル変数で置き換え:
```jinja
r###"{{ spec|tojson }}"###
```

### `split` フィルタの廃止

**現在:**
```jinja
{{(pointer|split)[-1]}}   {# メソッド名 #}
{{(pointer|split)[-2]}}   {# パス #}
```

**提案:**
テンプレートが `path_key` と `method` を直接持っているので不要:
```jinja
{{ method }}       {# "get", "post", etc. #}
{{ path_key }}     {# "/pets/{petId}", etc. #}
```

### スキーマ関連 — 変更なし

`SCHEMA`, `SCHEMA_NAME`, `IDENTIFIED_SCHEMA`, `IDENTIFIED_SCHEMA_DRAIN` マクロは
現在のままで動作する。`schema_push` / `schema_drain` の Rust 側インターフェースは同じ。

`SCHEMA` マクロ内の `schema["$ref"]` 判定は、循環参照で解決されなかった場合のフォールバックとして残す:
```jinja
{%- elif schema["$ref"] %}{{ SCHEMA_NAME(schema["$ref"]) }}
```

---

## テンプレートの統合: 1 テンプレート = 1 ファイル

### 仮説

> テンプレートの分割 (schema / operation / extension) は現状で利益を生んでおらず、
> LLM がテンプレートを管理する前提では、**1 テンプレート = 1 ファイル** の方が
> 正確性・保守性・可読性の全てにおいて優れる。

### 検証 1: 分割が正当化される条件を満たしているか

テンプレートの分割 (include) が正当化されるのは以下の **いずれか** が成立する場合:

| 分割が正当化される条件 | 現状の mandolin |
|---|---|
| **(A) 言語間で共有される** | **No** — Rust/TS/Go 各自が独自の schema/operation を持つ。クロスリファレンスはゼロ |
| **(B) 単体で意味を持つ** | **No** — サブテンプレートはマクロ定義のみで、include されないと動作しない |
| **(C) 組み合わせが変わりうる** | **No** — `rust_axum` は常に同じ 3 ファイルを include する。差し替えの想定がない |

**3 条件のいずれも満たしていない。**
現在の分割は「同じ一つのテンプレートを 3-4 ファイルに分けている」だけで、
モジュール分割の利点を得ていない。

### 検証 2: 分割が実害を生んでいるか

**実害 1 — コピペ事故 (`go_schema.template`):**

```jinja
{# go_schema.template L38-45 より — Go テンプレートに Rust 構文が混入: #}
type {{SCHEMA_NAME(pointer)}} struct {
{%- for property_key, property in schema.properties|items %}
    pub r#{{property_key}}:{%- if ... %}{{inner}}{%- else %}Option<{{inner}}>{%- endif %},
                                         {# ↑ Go に pub, r#, Option<> は存在しない #}
{%- endfor %}
}
{%- else %}
pub type {{SCHEMA_NAME(pointer)}}={{SCHEMA(pointer, schema)}};
{# ↑ Go に pub type は存在しない #}
```

`rust_schema.template` からコピーして Go 用に修正し忘れた箇所が **複数** 残っている。
schema ファイルが分離しているため、メインテンプレート (`go_server.template`) を読んだだけでは
この問題に気づけない。

**実害 2 — 暗黙の依存関係:**

`rust_operation.template` は `SCHEMA()` マクロを呼ぶが、その定義は `rust_schema.template` にある。
ファイル単体では依存が見えず、include 順序に暗黙に依存する。

**実害 3 — LLM にとっての認知コスト:**

| | 分割 (現状) | 統合 (提案) |
|---|---|---|
| テンプレート修正に必要なファイル読み込み | 4 ファイル | 1 ファイル |
| マクロ定義箇所の特定 | grep / 推測が必要 | 同一ファイル内で自明 |
| include 順序の把握 | 必要 | 不要 |
| 全体像の把握 | 4 ファイルを合成して理解 | 1 ファイルをそのまま理解 |

### 検証 3: 統合による悪影響はあるか

| 懸念 | 評価 |
|---|---|
| **ファイルが大きくなる** | 最大の `rust_axum` で ~460行。LLM のコンテキストにとって問題なし |
| **構造が見えにくくなる** | Jinja マクロ + セクションコメントで十分に構造化できる |
| **将来の言語間共有が困難になる** | 現在の macros は同名でも中身が完全に異なり、共有の見込みがない |

### 結論: 1 テンプレート = 1 ファイルにすべき

分割の 3 条件 (共有・単体意味・組み合わせ) をいずれも満たさず、
実害 (コピペ事故・暗黙依存・認知コスト) が確認された。
統合による悪影響は実質的にない。

**提案するファイル構成:**

```
templates/
  rust_axum.template        (~460行: schema + operation + extension + main)
  typescript_hono.template  (~200行: schema + operation + main)
  go_server.template        (~200行: schema + operation + main)
```

**統合後の各ファイル内部構造:**

```jinja
{# ===== Schema Macros ===== #}
{%- macro SCHEMA_NAME(pointer) %}...{%- endmacro %}
{%- macro SCHEMA(pointer, schema) %}...{%- endmacro %}
{%- macro IDENTIFIED_SCHEMA(pointer, schema) %}...{%- endmacro %}
{%- macro IDENTIFIED_SCHEMA_DRAIN() %}...{%- endmacro %}

{# ===== Operation Macros ===== #}
{%- macro OPERATION_NAME(pointer, operation) %}...{%- endmacro %}
{%- macro OPERATION_REQUEST(...) %}...{%- endmacro %}
{%- macro OPERATION_RESPONSE(...) %}...{%- endmacro %}

{# ===== Framework-specific Helpers (Rust/Axum のみ) ===== #}
{%- macro EXPRESSION_MULTIPART(...) %}...{%- endmacro %}
{%- macro UTILITY() %}...{%- endmacro %}

{# ===== Generated Code ===== #}
// This is generated by mandolin ...
pub trait ApiInterface { ... }
```

Jinja マクロがモジュールの役割を果たし、セクションコメントで視覚的に区切る。
`{% include %}` が消え、`build.rs` の `TEMPLATES` 配列も 10 → 3 エントリに簡素化される。

---

## 依存クレートの変更

```toml
[dependencies]
# 追加
heck = "0.5"

# 維持 (全て)
openapiv3 = "2.0"     # ← 維持: 公開APIの型安全性 + バリデーション
minijinja = "2.9"
regex = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
clap = { version = "4", features = ["derive"] }
```

`openapiv3` を維持する理由:
- 公開 API が `openapiv3::OpenAPI` のまま → **破壊的変更なし**
- 型付きの構造操作 (デフォルトサーバ追加等) が引き続き可能
- 呼び出し側はパース時にバリデーションの恩恵を受ける
- `#[serde(flatten)]` で拡張可能 → 将来のフィールド追加も容易

$ref 解決だけは内部で `serde_json::to_value()` → JSON ツリー操作で行う。
openapiv3 の型ツリーを歩いて `ReferenceOr<T>` を一つずつ解決するより圧倒的にシンプル。

---

## 公開 API — 変更なし

```rust
// 変更なし
pub fn environment(value: openapiv3::OpenAPI) -> Result<Environment<'static>, Error>
```

ライブラリユーザーの使い方:

```rust
// 変更なし
let api: openapiv3::OpenAPI = serde_yaml::from_str(&yaml)?;
let env = mandolin::environment(api)?;
```

**既存の全てのユーザーコード・WASM バインディングがそのまま動く。**

内部的には `ResolvedOpenAPI` でラップして $ref 解決するが、これは完全に内部実装の詳細。

---

## ファイル数・行数の比較 (概算)

**Rust ソース:**

| ファイル | 現在 | 提案 | 変化 |
|---|---|---|---|
| `lib.rs` (テスト除く) | 80行 | 40行 | -50% |
| `filter.rs` | 116行 | 15行 | -87% |
| `function.rs` | 92行 | 0行 (削除) | -100% |
| `schema_cache.rs` | — | 40行 | 新規 |
| `resolve.rs` | — | 80行 | 新規 |
| **合計** | **288行** | **175行** | **-39%** |

**テンプレート:**

| | 現在 | 提案 | 変化 |
|---|---|---|---|
| ファイル数 | 10 | 3 | -70% |
| 総行数 | ほぼ同等 | ほぼ同等 | — |
| `include_ref` 等の間接参照 | 多数 | 0 | 除去 |
| `ls_operation()` 等の独自関数呼出 | 多数 | 0 | テンプレート内で直接走査 |

テンプレートの総行数は変わらないが、`include` の消去 + 間接参照の除去で可読性が大幅に向上する。

---

## メリットまとめ

1. **Rust コードが大幅削減** — 本当に必要な処理 (データ準備・文字列変換) のみ残る
2. **公開 API は非破壊** — `openapiv3::OpenAPI` をそのまま受け取る。既存ユーザーに影響なし
3. **テンプレートが自己完結** — OpenAPI の構造知識がテンプレート側に集約される
4. **$ref 事前解決** — テンプレート内で参照解決を意識する必要がなくなる
5. **openapiv3 の拡張性** — `ResolvedOpenAPI` に将来フィールドを追加可能
   (`#[serde(flatten)]` + 追加フィールド)
6. **カスタムテンプレートの作成が容易** — `ls_operation` 等の独自関数を覚える必要がない。
   OpenAPI の構造さえ知っていれば `spec.paths` を直接走査できる
7. **デバッグが容易** — `spec` をそのまま `tojson` でダンプできる

## リスク・注意点

1. **循環参照**: `$ref` の循環参照は `resolve.rs` で検出し、解決せずに残す。
   テンプレート側の `SCHEMA` マクロに `schema["$ref"]` フォールバックを残すことで対処。

2. **テンプレート互換性**: 既存テンプレート (Rust/TypeScript/Go) は全て書き換えが必要。
   `ls_operation()`, `include_ref` 等を使うカスタムテンプレートも影響を受ける。
   ただし公開 Rust API は非破壊なので、影響範囲はテンプレートのみ。

3. **`heck` クレートのケース変換**: 現在の手動実装と微妙に挙動が異なる可能性がある。
   特に数字を含む識別子 (`int32` → `Int32` vs `Int_32`) でテストが必要。

4. **`serde_json::to_value` のコスト**: `OpenAPI` → `Value` 変換 + $ref 解決のために
   全体を1回クローンする。ただし一度だけの処理なので実用上は問題ない。
