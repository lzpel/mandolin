# 

## openapi/openapi_lambda360.json は正しい

## examples/openapi_lambda360.rs は正しいか確証がない

### chatgptにopenapi/openapi_lambda360.jsonがリクエストのボディで要求しているshapenodeのデータを3種類用意してもらう。

以下は **`ShapeNode` が取りうる JSON のダミーデータ例 3 種**です。
（目的：`anyOf` で分岐する **パース（deserialization）が正しく動く**ことの確認）

---

1) StepNode（末端ノード）

```json
{
  "op": "step",
  "path": "s3://lambda360/parts/PA-001-DF7.STEP",
  "content_hash": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
}
```

---

2) TranslateNode（単項：shape を 1 つ持つ）

```json
{
  "op": "translate",
  "shape": {
    "op": "step",
    "path": "s3://lambda360/parts/PA-001-DF7.STEP",
    "content_hash": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  },
  "xyz": [10, 0, -5]
}
```

---

3) SubtractNode（2項：a,b を持つ + 式を混ぜる）

```json
{
  "op": "subtract",
  "a": {
    "op": "union",
    "a": {
      "op": "step",
      "path": "s3://lambda360/parts/base.STEP",
      "content_hash": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    },
    "b": {
      "op": "translate",
      "shape": {
        "op": "step",
        "path": "s3://lambda360/parts/boss.STEP",
        "content_hash": "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
      },
      "xyz": ["$dx", 0, 0]
    }
  },
  "b": {
    "op": "rotate",
    "shape": {
      "op": "step",
      "path": "s3://lambda360/parts/hole.STEP",
      "content_hash": "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
    },
    "axis": [0, 0, 1],
    "deg": "$angleDeg"
  }
}
```

---

### これで「パースが正しい」をどう担保するか（最小チェック観点）

* `op` がそれぞれ **"step" / "translate" / "subtract"** で分岐できる
* 単項（shape）・二項（a,b）の **再帰構造**を通る
* `NumberOrExpr` に **number と string（式）**の両方が現れる（例3）

必要なら、あなたの実装言語（Rust serde / TS zod など）に合わせて「この3 JSON を食わせたときに *必ず* 期待する型に落ちる」テストコードの雛形もそのまま出します。

### 3種類のダミーデータがパースされることをclaude codeに要求する。

それによりmandolinが他のshapenodeを正しく受け取れることを保証する。

fn mainをパースチェック専用に入れ替えたうえで cargo run --example openapi_lambda360 を実行しcode 0が返るようにテンプレートを修正することをclaude codeに要求してみよう。それで成功すると今は考えている

以下chatgptの回答

---

了解。**HTTP サーバを立てずに**、`ShapeNode` の **serde パースが通るか**だけを確かめる `fn main` に丸ごと差し替えるなら、こういう形になります。

ポイントはこれだけです：

* `ShapeNode` は `#[serde(untagged)]` なので **`op` を見て自動で Variant が決まるわけではなく**、各 Variant の構造に「合う」ものから順に試されます（Variant0→1→2…）。
  → なので、テスト JSON は **互いに曖昧にマッチしない**形にしておくのが重要。
* “成功したか”は `serde_json::from_str::<ShapeNode>()` が `Ok` になるかで判定。

下の `fn main` は、あなたが前に作った **3種類の JSON（step / translate / subtract）** を埋め込み、`ShapeNode` にパースして、最終的に **どの Variant に落ちたか**まで表示します。

```rust
fn main() {
	// 1) StepNode
	let j1 = r#"
	{
	  "op": "step",
	  "path": "s3://lambda360/parts/PA-001-DF7.STEP",
	  "content_hash": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
	}
	"#;

	// 2) TranslateNode
	let j2 = r#"
	{
	  "op": "translate",
	  "shape": {
	    "op": "step",
	    "path": "s3://lambda360/parts/PA-001-DF7.STEP",
	    "content_hash": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
	  },
	  "xyz": [10, 0, -5]
	}
	"#;

	// 3) SubtractNode（式も混ぜる）
	let j3 = r#"
	{
	  "op": "subtract",
	  "a": {
	    "op": "union",
	    "a": {
	      "op": "step",
	      "path": "s3://lambda360/parts/base.STEP",
	      "content_hash": "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
	    },
	    "b": {
	      "op": "translate",
	      "shape": {
	        "op": "step",
	        "path": "s3://lambda360/parts/boss.STEP",
	        "content_hash": "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
	      },
	      "xyz": ["$dx", 0, 0]
	    }
	  },
	  "b": {
	    "op": "rotate",
	    "shape": {
	      "op": "step",
	      "path": "s3://lambda360/parts/hole.STEP",
	      "content_hash": "sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
	    },
	    "axis": [0, 0, 1],
	    "deg": "$angleDeg"
	  }
	}
	"#;

	let cases = [("case1_step", j1), ("case2_translate", j2), ("case3_subtract", j3)];

	for (name, json) in cases {
		println!("=== {name} ===");
		match serde_json::from_str::<ShapeNode>(json) {
			Ok(node) => {
				// どの Variant に落ちたかを表示（serde(untagged) の挙動確認用）
				let variant = match &node {
					ShapeNode::Variant0(_) => "Variant0(StepNode)",
					ShapeNode::Variant1(_) => "Variant1(UnionShapeNode)",
					ShapeNode::Variant2(_) => "Variant2(IntersectNode)",
					ShapeNode::Variant3(_) => "Variant3(SubtractNode)",
					ShapeNode::Variant4(_) => "Variant4(ScaleNode)",
					ShapeNode::Variant5(_) => "Variant5(TranslateNode)",
					ShapeNode::Variant6(_) => "Variant6(RotateNode)",
					ShapeNode::Variant7(_) => "Variant7(StretchNode)",
				};
				println!("OK: parsed as {variant}");

				// ついでに整形して再出力（Serialize/Deserialize の往復チェック）
				let pretty = serde_json::to_string_pretty(&node).expect("serialize should succeed");
				println!("{pretty}");
			}
			Err(e) => {
				eprintln!("NG: parse failed: {e}");
				std::process::exit(1);
			}
		}
	}

	println!("All cases parsed successfully.");
}
```

### これで「パースの正しさ」をどこまで保証できる？

この `main` は少なくとも次を保証します。

* `ShapeNode` の **再帰構造**（shape/a/b のネスト）が serde で破綻してない
* `NumberOrExpr` の **number/string 両対応**が効いてる
* 3ケース全部が **Variant のどれかに確定**し、例外で落ちない

---

### 注意（本質）

あなたの現状コードは `ShapeNode` が `untagged enum` なので、「`op` が一致しない Variant にも構造が合えばマッチしてしまう」リスクがあります。
（例：全 struct が `op:String` で enum 制約が Rust 側に無いので、**`{"op":"rotate","shape":..., "axis":..., "deg":...}` が別 Variant に誤マッチ**する可能性が理論上あります。たまたま必須フィールドの違いで防げてるだけ。）

もし「op による discriminator を Rust 側でも厳密にしたい」なら、`op` を `String` じゃなく `#[serde(rename="rotate")] enum Op { Rotate }` みたいにするか、`#[serde(tag="op")]` の tagged enum に寄せる（OpenAPI の discriminator と整合）方が堅いです。

必要なら、**mandolin 側の生成方針**として「`op` を enum で固定」するパターン（破壊的変更少なめ）も提案できます。


## 自動生成されるコードはどのような書き方になるだろうか

serdeでパース出来れば何でもよいのでClaude codeに自動生成にふさわしい自然な書き方以外を要求しないが

おおよそこんなかんじになるのではと人間は予想している

```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ShapeNode {
    #[serde(rename = "step")]
    Step(StepNode),

    #[serde(rename = "union")]
    Union(UnionNode),

    #[serde(rename = "intersect")]
    Intersect(IntersectNode),

    #[serde(rename = "subtract")]
    Subtract(SubtractNode),

    #[serde(rename = "scale")]
    Scale(ScaleNode),

    #[serde(rename = "translate")]
    Translate(TranslateNode),

    #[serde(rename = "rotate")]
    Rotate(RotateNode),

    #[serde(rename = "stretch")]
    Stretch(StretchNode),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepNode {
    pub path: String,
    #[serde(default)]
    pub content_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnionNode {
    pub a: Box<ShapeNode>,
    pub b: Box<ShapeNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntersectNode {
    pub a: Box<ShapeNode>,
    pub b: Box<ShapeNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtractNode {
    pub a: Box<ShapeNode>,
    pub b: Box<ShapeNode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScaleNode {
    pub shape: Box<ShapeNode>,
    pub factor: NumberOrExpr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranslateNode {
    pub shape: Box<ShapeNode>,
    pub xyz: Vec<NumberOrExpr>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotateNode {
    pub shape: Box<ShapeNode>,
    pub axis: Vec<NumberOrExpr>,
    pub deg: NumberOrExpr,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StretchNode {
    pub shape: Box<ShapeNode>,
    pub cut: Vec<NumberOrExpr>,
    pub delta: Vec<NumberOrExpr>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberOrExpr {
    Num(f64),
    Expr(String),
}
```

---

## Claude Code によるリファクタリング実施報告（2026-02-20）

> 以下は Claude Code (claude-opus-4-6) が上記の要件に基づいて実施したリファクタリングの記録です。

### リファクタリング前の問題

1. **再帰型が無限サイズ** — `ShapeNode` を含む struct（`UnionShapeNode` 等）に `Box<>` がなく、`cargo test` でコンパイルエラー（`recursive types have infinite size`）
2. **Variant の誤マッチ** — `#[serde(untagged)]` では `op` の値を見ずに構造の一致だけで判定するため、`UnionShapeNode`・`IntersectNode`・`SubtractNode`（全て `a`, `b` を持つ）が区別できず、常に最初の Variant にマッチしてしまう
3. **Variant 名が不明瞭** — `Variant0`, `Variant1` ... では何の型か読めない

### 変更内容

#### `src/lib.rs` — `anyof_tag` 関数 + `tag_skip_push`/`tag_skip_get` を追加

- **`anyof_tag`**: `anyOf` スキーマの暗黙的 discriminator を検出。全 variant が `$ref` で、かつ共通の単一値 `enum` プロパティ（例：`op: {enum: ["step"]}`）を持つかを判定し、持っていればそのプロパティ名（`"op"`）を返す。
- **`tag_skip_push` / `tag_skip_get`**: tagged anyOf の variant `$ref` パスと discriminator プロパティ名を記録し、struct 生成時に**対象の struct の対象プロパティだけ**を除外する。tagged anyOf と無関係な standalone struct のプロパティは影響を受けない。

#### `templates/rust_axum.template` — 3 パス構成に変更

| パス / 箇所 | 変更内容 |
|------|----------|
| Phase 1（既存） | 名前付きスキーマの先行登録 |
| Phase 2（新規） | `anyof_tag` で tagged anyOf を検出し、`tag_skip_push` で variant の discriminator プロパティを記録 |
| Phase 3（既存を改修） | struct 生成時に `tag_skip_get` で確認し、記録されたプロパティだけを除外。SCHEMA マクロで anyOf 参照を `Box<>` 化。anyOf enum で tagged / untagged を分岐 |

### 生成コードの Before / After

```rust
// Before
#[serde(untagged)]
pub enum ShapeNode {
    Variant0(StepNode),
    Variant1(UnionShapeNode),
    ...
}
pub struct StepNode {
    pub op: String,          // 冗長
    pub path: String,
    pub content_hash: Option<String>,
}
pub struct UnionShapeNode {
    pub a: ShapeNode,        // Box なし → コンパイルエラー
    pub b: ShapeNode,
    pub op: String,
}
```

```rust
// After
#[serde(tag="op")]
pub enum ShapeNode {
    #[serde(rename="step")]       Step(StepNode),
    #[serde(rename="union")]      Union(UnionShapeNode),
    #[serde(rename="intersect")]  Intersect(IntersectNode),
    #[serde(rename="subtract")]   Subtract(SubtractNode),
    #[serde(rename="scale")]      Scale(ScaleNode),
    #[serde(rename="translate")]  Translate(TranslateNode),
    #[serde(rename="rotate")]     Rotate(RotateNode),
    #[serde(rename="stretch")]    Stretch(StretchNode),
}
pub struct StepNode {
    pub path: String,
    pub content_hash: Option<String>,
}
pub struct UnionShapeNode {
    pub a: Box<ShapeNode>,
    pub b: Box<ShapeNode>,
}
```

### テスト結果

`make test-shapenode` が正常終了（exit code 0）。3 つの JSON テストケースが全てパースされ、JCS (RFC 8785) 正規化による往復一致（`JCS(before) == JCS(after)`）も確認済み。

```
=== case1_step ===
OK: JCS(before)==JCS(after)
=== case2_translate ===
OK: JCS(before)==JCS(after)
=== case3_subtract ===
OK: JCS(before)==JCS(after)
All cases passed.
```

### 汎用性

| anyOf パターン | 検出結果 | 生成 |
|---|---|---|
| 全 variant が `$ref`、共通の単一値 enum プロパティあり | `anyof_tag` → `"op"` 等 | `#[serde(tag="op")]` tagged enum、variant struct から discriminator 除外 |
| inline 型の anyOf（NumberOrExpr 等） | `anyof_tag` → `""` | `#[serde(untagged)]` enum（従来通り） |
| `$ref` だが共通 discriminator なし | `anyof_tag` → `""` | `#[serde(untagged)]` enum（従来通り） |
| 混在（一部 `$ref`、一部 inline） | `anyof_tag` → `""` | `#[serde(untagged)]` enum（従来通り） |
| tagged anyOf と無関係な standalone struct に単一値 enum | `tag_skip_get` → `false` | プロパティは消えない |

- 未対応のケースが来ても `#[serde(untagged)]` にフォールバックするので壊れることはない。
- `anyof_tag` や `tag_skip_push`/`tag_skip_get` を拡張する形で自然に対応できる。