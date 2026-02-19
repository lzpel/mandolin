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

```

