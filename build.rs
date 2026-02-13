//! ビルドスクリプト — テンプレートファイルをバイナリに埋め込む
//!
//! templates/*.templateを読み込み、定数としてtemplates.rsに書き出す。
//! ファイル名がそのままテンプレート名になる（大文字変換）。
//! 例: rust_axum.template → RUST_AXUM

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("templates.rs");
    let mut file = fs::File::create(&dest_path).unwrap();

    // テンプレートディレクトリから全.templateファイルを収集
    let templates = collect_templates();

    // 各テンプレートを定数として出力
    for (name, content) in &templates {
        writeln!(file, "#[allow(dead_code)]").unwrap();
        writeln!(
            file,
            r##########"pub const {name}: &str = r######"{content}"######;"##########
        )
        .unwrap();
    }

    // テンプレート一覧の配列を出力
    writeln!(
        file,
        "pub const TEMPLATES: [[&str; 2]; {}] = [",
        templates.len()
    )
    .unwrap();
    for (name, _) in &templates {
        writeln!(file, r#"    ["{name}", {name}],"#).unwrap();
    }
    writeln!(file, "];").unwrap();
}

/// templates/ディレクトリの.templateファイルを(名前, 内容)のリストとして返す
fn collect_templates() -> Vec<(String, String)> {
    let template_dir = Path::new(".").join("templates");
    let mut templates: Vec<_> = fs::read_dir(&template_dir)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "template")
        })
        .map(|entry| {
            let stem = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_uppercase();
            let content = fs::read_to_string(entry.path()).unwrap();
            (stem, content)
        })
        .collect();
    templates.sort_by(|a, b| a.0.cmp(&b.0));
    templates
}
