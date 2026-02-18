# `$ref` 事前解決の廃止による正規スキーマ名の利用

## 問題

現在の `resolve.rs` はすべての `$ref` をインライン展開してからテンプレートに渡す。
その結果、`#/components/schemas/Point` を 100 箇所から参照しても、
テンプレートが見るスキーマはすべて `$ref` キーを持たない `{"type":"object",...}` になる。

`SCHEMA` マクロはこれを `type == "object"` ブランチで処理し、
`SCHEMA_NAME(現在のポインタ)` を呼ぶ。ポインタはそれぞれ異なる
（例: `#/paths/foo/get/.../schema`、`#/paths/bar/post/.../schema`）ため、
`schema_push` キャッシュはすべてを別エントリと見なし、100 個の struct が生成される。
名前は `PathsFooGetResponsesSchema` のようなパス由来のものになり、
`struct Point` という名前は生成されない。

```
openapi.json の Point への $ref × 100
  → resolve.rs がすべてインライン展開
  → テンプレートが 100 種類のポインタで SCHEMA_NAME を呼ぶ
  → 100 個の異なる struct（内容は同じ、名前が違う）
```

## 提案

`#/components/schemas/` 配下を指す `$ref` は解決しない。
テンプレートに `{"$ref":"#/components/schemas/Point"}` がそのまま届くようにする。

`SCHEMA` マクロの `$ref` ブランチが循環参照のフォールバックから**主経路**になり、
`SCHEMA_NAME("#/components/schemas/Point")` が呼ばれる。
これにより `struct Point` が 1 回だけ生成され、
参照箇所では `x: Point` という型名が使われる。

```
openapi.json の Point への $ref × 100
  → resolve.rs がスキーマ $ref をスキップ
  → テンプレートが 100 回とも SCHEMA_NAME("#/components/schemas/Point") を呼ぶ
  → schema_push がキャッシュ済みと判断し 2 回目以降は IDENTIFIED_SCHEMA をスキップ
  → struct Point が 1 個、参照側は x: Point
```

---

## 変更 1: `resolve.rs` — スキーマ $ref のスキップ

`resolve_recursive` に 1 条件を追加する。

```rust
fn resolve_recursive(node: &mut Value, root: &Value, visiting: &mut HashSet<String>) {
    match node {
        Value::Object(map) => {
            if let Some(Value::String(ref_path)) = map.get("$ref") {
                let ref_path = ref_path.clone();
                // ↓ 追加: スキーマ定義への $ref はインライン展開しない
                if ref_path.starts_with("#/components/schemas/") {
                    return;
                }
                if let Some(resolved) = lookup_pointer(root, &ref_path) {
                    if !visiting.contains(&ref_path) {
                        visiting.insert(ref_path.clone());
                        let mut resolved = resolved.clone();
                        resolve_recursive(&mut resolved, root, visiting);
                        visiting.remove(&ref_path);
                        *node = resolved;
                        return;
                    }
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
```

パラメータ・レスポンスオブジェクト・requestBody への `$ref`
（`#/components/parameters/`、`#/components/responses/` 等）は引き続き解決される。
テンプレートが `parameter.name` や `response.content` を直接参照できる状態を保つため。

---

## 変更 2: `rust_axum.template` — `SCHEMA` マクロの分岐順序

現在 `schema["$ref"]` は `elif`（循環参照フォールバック）だが、これを先頭の `if` に昇格させる。

**変更前:**
```jinja
{%- macro SCHEMA(pointer,schema) -%}
{%- if schema.type -%}
  ...
{%- elif schema["$ref"] %}{{SCHEMA_NAME(schema["$ref"])}}
{%- else %}u8
{%- endif %}
{%- endmacro -%}
```

**変更後:**
```jinja
{%- macro SCHEMA(pointer,schema) -%}
{%- if schema["$ref"] %}{{SCHEMA_NAME(schema["$ref"])}}
{%- elif schema.type -%}
  ...
{%- else %}u8
{%- endif %}
{%- endmacro -%}
```

`$ref` が届いた時点で `SCHEMA_NAME(ref_path)` を呼び、
`ref_path` がキャッシュになければ `IDENTIFIED_SCHEMA` でその名前の struct を生成する。
2 回目以降は `schema_push` がキャッシュ済みと判断して生成をスキップし、型名だけ返す。

---

## Before / After

### OpenAPI

```json
"components": {
  "schemas": {
    "Point": {
      "type": "object",
      "properties": {
        "x": {"type": "integer", "format": "int32"},
        "y": {"type": "integer", "format": "int32"},
        "z": {"type": "integer", "format": "int32"}
      }
    }
  }
},
"paths": {
  "/a": {"get": {"responses": {"200": {"content": {"application/json":
    {"schema": {"$ref": "#/components/schemas/Point"}}}}}}},
  "/b": {"post": {"requestBody": {"content": {"application/json":
    {"schema": {"$ref": "#/components/schemas/Point"}}}}}}
}
```

### 変更前（現在）

```rust
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsAGetResponses200ContentApplicationJsonSchema {
    pub r#x: Option<i32>,
    pub r#y: Option<i32>,
    pub r#z: Option<i32>,
}

#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct PathsBPostRequestBodyContentApplicationJsonSchema {
    pub r#x: Option<i32>,
    pub r#y: Option<i32>,
    pub r#z: Option<i32>,
}
// struct Point は生成されない
```

### 変更後（提案）

```rust
#[derive(Default,Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Point {
    pub r#x: Option<i32>,
    pub r#y: Option<i32>,
    pub r#z: Option<i32>,
}
// GET /a のレスポンスも POST /b のボディも Point を参照するだけ
```

---

## 波及範囲

### `resolve.rs` のテスト

`test_resolve_simple_ref` は `$ref` が解決されインライン化されることを確認している。
`#/components/schemas/` への `$ref` はスキップされるようになるため、このテストを修正するか
対象を `#/components/parameters/` など非スキーマ `$ref` に変える。

### `test_circular_ref_preserved` は影響なし

循環参照は `#/components/schemas/` 配下なので、今後は「スキップ」によって
`$ref` が残る（以前は循環検出によって残っていた）。結果は同じ。

### `SCHEMA` マクロの `discriminator` 対応（`20260219-lambda360.md` 変更 3）

lambda360 ドキュメントでは `SCHEMA` の先頭に `discriminator` チェックを追加する案があった。
今回の変更で `$ref` を先頭に置くため、追加順は以下にする:

```jinja
{%- macro SCHEMA(pointer,schema) -%}
{%- if schema["$ref"] %}{{SCHEMA_NAME(schema["$ref"])}}
{%- elif schema.discriminator %}{{SCHEMA_NAME(pointer)}}
{%- elif schema.type -%}
  ...
```

### `allOf` の単一 `$ref` パターン

```json
"deg": {"allOf": [{"$ref": "#/components/schemas/NumberOrExpr"}], "description": "..."}
```

`resolve.rs` がスキップした結果、このフィールドのスキーマは
`{"allOf":[{"$ref":"#/components/schemas/NumberOrExpr"}]}` のまま届く。
`SCHEMA` マクロが `schema.allOf` ブランチ（lambda360 ドキュメント 変更 1）で
`schema.allOf[0]` を再帰処理すると `schema["$ref"]` が見つかり `NumberOrExpr` が返る。
事前解決があった場合と同じ結果になり、追加対応は不要。
