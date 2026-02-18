# lambda360 対応: schema-ref / two-phase 比較

`20260219-lambda360.md` の 4 パターンが、
`schema-ref.md`（スキーマ $ref のみスキップ）と
`two-phase.md`（resolve.rs 完全削除）でどう変わるかを整理する。

---

## パターン 1: `anyOf` → untagged enum

```json
"NumberOrExpr": { "anyOf": [{"type":"number"}, {"type":"string"}] }
```

**いずれも同じ**。`NumberOrExpr` はスキーマ定義であり `$ref` の問題ではないため、
`IDENTIFIED_SCHEMA` に `anyOf` 分岐を追加する作業は変わらない。

two-phase では Phase 1 の明示ループで処理されるため、
「`SCHEMA_NAME` が呼ばれて初めて生成される」という間接性がなくなり、
生成タイミングが自明になる。

---

## パターン 2: 単一要素 `allOf` + `$ref`（description 付き）

```json
"deg": { "allOf": [{"$ref":"#/components/schemas/NumberOrExpr"}], "description":"..." }
```

これが最も差が出る。

### 現状

`resolve.rs` が `allOf[0]` の `$ref` をインライン展開するため、
テンプレートが受け取るのは:

```json
"deg": { "allOf": [{"anyOf":[{"type":"number"},{"type":"string"}]}] }
```

`SCHEMA` マクロで単一 `allOf` を unwrap すると `allOf[0]` が `{"anyOf":[...]}` になる。
`anyOf` を処理する分岐が `SCHEMA` にも必要になり、**修正が 2 箇所**になる。

### schema-ref.md

スキーマ `$ref` をスキップするため、テンプレートが受け取るのは:

```json
"deg": { "allOf": [{"$ref":"#/components/schemas/NumberOrExpr"}] }
```

単一 `allOf` を unwrap → `allOf[0]` が `{"$ref":"..."}` → `$ref` 分岐 → `NumberOrExpr`。
**`SCHEMA` への `anyOf` 分岐は不要**（`IDENTIFIED_SCHEMA` 側だけで済む）。

### two-phase.md

同上。さらに `ref_name` フィルタで直接型名を取り出すだけ。

```jinja
{%- if schema.allOf|length == 1 %}{{SCHEMA(pointer+"/allOf/0", schema.allOf[0])}}
{# → allOf[0] が {"$ref":"..."} → ref_name → "NumberOrExpr" #}
```

---

## パターン 3: `discriminator` → tagged enum

```json
"ShapeNode": {
  "type": "object",
  "discriminator": { "propertyName":"op", "mapping":{"step":"#/components/schemas/StepNode",...} }
}
```

### 現状

`$ref` → `ShapeNode` がすべて use site にインライン展開される。
`SCHEMA` マクロが `discriminator` を持つ object スキーマを受け取り、
`type == "object"` より先に `discriminator` チェックが必要になる。
しかしその時点のポインタは `#/paths/foo/.../schema` であり、
**`schema_push` キャッシュは呼び出し元ポインタをキーにするため
`enum ShapeNode` が use site ごとに別名で重複生成される**（100 コピー問題と同根）。

lambda360 ドキュメントの「変更 3」では `SCHEMA` の先頭に `discriminator` チェックを
置くよう指示しているが、これはポインタが `#/components/schemas/ShapeNode` のときだけ
正しく動く。use site ポインタで呼ばれた場合は別名の enum が量産される。

### schema-ref.md

`$ref` → `ShapeNode` がスキップされてテンプレートに届くため、
`SCHEMA` は `$ref` 分岐に入り `SCHEMA_NAME("#/components/schemas/ShapeNode")` を呼ぶ。
**キャッシュキーが `#/components/schemas/ShapeNode` に統一される**ので、
100 箇所から参照されても `enum ShapeNode` は 1 つだけ生成される。

### two-phase.md

Phase 1 の明示ループで `ShapeNode` を処理するため、
`enum ShapeNode` は必ず 1 回だけ生成される。
use site では `ref_name` フィルタが `"ShapeNode"` を返すだけで
構造体生成のロジックを一切踏まない。**最もシンプル**。

`discriminator.mapping` の値もただの文字列（`$ref` パス）なので、
`ref_name` フィルタでそのまま型名に変換できる:

```jinja
{%- for op_value, ref_path in schema.discriminator.mapping|items %}
    #[serde(rename = "{{op_value}}")]
    {{op_value|to_pascal_case}}({{ref_path|ref_name}}),
{%- endfor %}
```

---

## パターン 4: `allOf` による継承マージ

```json
"StepNode": {
  "type": "object",
  "allOf": [{"$ref":"#/components/schemas/ShapeNode"}]
}
```

テンプレート内で `allOf` の各要素を走査して親フィールドをマージする処理。

### 現状

`allOf[0]` はインライン済みの `ShapeNode` オブジェクトなので
`parent.properties` に直接アクセスできる。ただし `ShapeNode` が `discriminator` を
持っている場合、インライン後も `discriminator` が混入するため処理が複雑になる。

### schema-ref.md

`allOf[0]` が `{"$ref":"#/components/schemas/ShapeNode"}` のまま届く。
`parent.properties` を参照する前に `include_pointer` で実体を取得する必要がある:

```jinja
{%- for parent_ref in schema.allOf %}
{%- if parent_ref["$ref"] %}
{%- set parent = parent_ref["$ref"]|include_pointer %}
{%- else %}
{%- set parent = parent_ref %}
{%- endif %}
{%- for property_key, property in parent.properties|default({})|items %}
```

現状より 1 ステップ増えるが、インライン済みオブジェクトの中に `discriminator` 等が
混入するという問題は消える。

### two-phase.md

`deref` フィルタで統一的に書ける:

```jinja
{%- for parent_ref in schema.allOf %}
{%- set parent = parent_ref|deref %}
{%- for property_key, property in parent.properties|default({})|items %}
```

`$ref` でも直接記述でも同じ 1 行で済む。**最もシンプル**。

---

## 循環参照（`ShapeNode` ↔ `RotateNode`）

### 現状

`resolve.rs` の循環検出に依存。2 周目で `$ref` が残り、テンプレートが
`elif schema["$ref"]` で拾う設計。

### schema-ref.md / two-phase.md

スキーマ `$ref` はすべてそのまま届くため、**循環かどうかに関係なく同じ処理**になる。
循環検出ロジックが不要になり、`$ref` → 型名という単純なルールだけで動く。

---

## 総合比較

| パターン | 現状 | schema-ref.md | two-phase.md |
|---------|------|---------------|--------------|
| P1: anyOf enum | `IDENTIFIED_SCHEMA` に分岐追加 | 同左 | 同左（Phase 1 で明示的） |
| P2: allOf + $ref | `SCHEMA` と `IDENTIFIED_SCHEMA` の両方に `anyOf` 分岐が必要 | `IDENTIFIED_SCHEMA` のみ | 同左、`ref_name` で直接 |
| P3: discriminator | use site ポインタで enum が重複生成される（根本的な欠陥） | キャッシュキー統一で 1 回だけ生成 | Phase 1 で確実に 1 回生成 |
| P4: allOf 継承 | `parent.properties` 直接アクセス可 | `include_pointer` が必要 | `deref` フィルタで統一 |
| 循環参照 | resolve.rs の検出ロジックに依存 | 不要（全 $ref が残る） | 不要（同左） |

### 判断

- **P2・P3 で現状のアプローチには根本的な問題がある**。
  P2 は修正箇所が増え、P3 は `schema_push` のキャッシュキーが use site ポインタに
  なるため何をしても enum の重複生成を避けられない。

- **schema-ref.md** は `resolve.rs` への変更が最小限で済む一方、
  P4 の `include_pointer` 呼び出しや、`deref` 相当の処理がテンプレート側に散在する。

- **two-phase.md** は変更量が多いが、すべてのパターンで「`$ref` → 型名のみ」
  「実体が必要なら `deref`」という一貫したルールで処理できる。
  lambda360 の 4 パターン全てにおいて修正箇所が最も少なく、修正内容が最も単純になる。
