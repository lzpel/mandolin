//! mandolin CLI — OpenAPI仕様からサーバコードを生成する
//!
//! 使用例:
//!   mandolin -i openapi.yaml -t RUST_AXUM -o server.rs
//!   cat openapi.json | mandolin -i - -t RUST_AXUM > server.rs

use clap::Parser;
use std::io::Read;

/// OpenAPI仕様からサーバコードを生成するコマンドラインツール
#[derive(Parser)]
#[command(name = "mandolin", about = "Generate server code from OpenAPI specification")]
struct Args {
    /// 入力ファイルパス（"-" で標準入力から読み込み）
    #[arg(short, long)]
    input: String,

    /// 出力ファイルパス（省略時は標準出力）
    #[arg(short, long)]
    output: Option<String>,

    /// テンプレート名（デフォルト: RUST_AXUM）
    #[arg(short, long, default_value = "RUST_AXUM")]
    template: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 入力ファイルまたは標準入力を読み込む
    let input = if args.input == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(&args.input)?
    };

    // YAMLまたはJSONとしてパースする
    let api: openapiv3::OpenAPI = if args.input.ends_with(".json") {
        serde_json::from_str(&input)?
    } else {
        serde_yaml::from_str(&input)?
    };

    // テンプレート環境を構築してレンダリング
    let env = mandolin::environment(api)?;
    let template = env.get_template(&args.template)?;
    let output = template.render(0)?;

    // 出力先に書き込む
    if let Some(path) = args.output {
        std::fs::write(&path, &output)?;
    } else {
        print!("{}", output);
    }

    Ok(())
}
