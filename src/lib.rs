//! mandolin — OpenAPI仕様からサーバコードを生成するテンプレートエンジン
//!
//! # 設計方針
//! - Rustは「データの準備」のみ行い、「コードの組み立て」は全てテンプレートに任せる
//! - $refは事前解決し、テンプレートは解決済みオブジェクトを直接操作する
//! - Rustのフィルタはケース変換・正規表現などJinjaでは困難な処理のみ提供する

mod filter;
mod resolve;
mod schema_cache;
/// ビルド時に生成されるテンプレート定数
pub mod templates {
    include!(concat!(env!("OUT_DIR"), "/templates.rs"));
}

use openapiv3::OpenAPI;

/// OpenAPI仕様からテンプレート環境を構築する
///
/// 1. openapiv3を拡張して$refを事前解決
/// 2. 解決済みスペックをグローバル変数`spec`としてテンプレートに渡す
/// 3. 最小限のフィルタ・関数を登録
///
/// テンプレートは`spec.paths`等を直接走査してコードを生成する。
pub fn environment(spec: OpenAPI) -> Result<minijinja::Environment<'static>, minijinja::Error> {
    // $refを事前解決し、テンプレート用のJSON Valueを得る
    let resolved = resolve::ResolvedOpenAPI::new(spec);
    let value = resolved.resolved_value();

    // テンプレート環境を構築
    let mut env = minijinja::Environment::new();
    for [k, v] in templates::TEMPLATES {
        env.add_template(k, v)?;
    }

    // 解決済みスペックをグローバル変数として設定
    // テンプレートからは`spec.paths`, `spec.components`等でアクセスできる
    env.add_global("spec", minijinja::Value::from_serialize(&value));

    // フィルタ登録（Jinjaでは困難な言語機能のみ）
    env.add_filter("to_snake_case", filter::to_snake_case);
    env.add_filter("to_pascal_case", filter::to_pascal_case);
    env.add_filter("to_camel_case", filter::to_camel_case);
    env.add_filter("re_replace", filter::re_replace);
    env.add_filter("encode", filter::encode);
    env.add_filter("decode", filter::decode);

    // include_pointerフィルタ: JSON Pointerで解決済みspecから値を取得する
    // SCHEMA_NAMEマクロがポインタからスキーマオブジェクトを参照するために必要
    {
        let spec_value = value.clone();
        env.add_filter(
            "include_pointer",
            move |pointer: &str| -> Result<minijinja::Value, minijinja::Error> {
                let path = pointer.strip_prefix("#/").ok_or_else(|| {
                    minijinja::Error::new(
                        minijinja::ErrorKind::InvalidOperation,
                        format!("invalid pointer: {pointer}"),
                    )
                })?;
                let mut current = &spec_value;
                for segment in path.split('/') {
                    let decoded = segment.replace("~1", "/").replace("~0", "~");
                    current = current.get(&decoded).ok_or_else(|| {
                        minijinja::Error::new(
                            minijinja::ErrorKind::InvalidOperation,
                            format!("pointer not found: {pointer}"),
                        )
                    })?;
                }
                Ok(minijinja::Value::from_serialize(current))
            },
        );
    }

    // スキーマキャッシュ（インラインスキーマの重複排除用）
    let cache = schema_cache::SchemaCache::new();
    {
        let c = cache.clone();
        env.add_function(
            "schema_push",
            move |pointer: &str, content: Option<&str>| c.push(pointer, content),
        );
        let c = cache.clone();
        env.add_function("schema_drain", move || c.drain());
    }

    Ok(env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    /// openapiディレクトリの全ファイルをパースしてHashMapに格納する
    fn api_map() -> std::collections::HashMap<String, OpenAPI> {
        fs::read_dir(Path::new(".").join("openapi"))
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();
                let ext = path.extension()?.to_str()?;
                let name = entry.file_name().to_str()?.to_string();
                match ext {
                    "yaml" | "yml" => {
                        let reader = std::io::BufReader::new(fs::File::open(&path).ok()?);
                        serde_yaml::from_reader(reader).ok().map(|api| (name, api))
                    }
                    "json" => {
                        let reader = std::io::BufReader::new(fs::File::open(&path).ok()?);
                        serde_json::from_reader(reader).ok().map(|api| (name, api))
                    }
                    _ => None,
                }
            })
            .collect()
    }

    /// 指定テンプレートで全OpenAPIファイルをレンダリングし、out/に出力する
    fn render_target(template: &str, extension: &str) {
        for (name, api) in api_map() {
            println!("render start: {name}");
            let env = environment(api).unwrap();
            let tmpl = env.get_template(template).unwrap();
            let output = tmpl.render(0).unwrap();
            let out_path = format!("out/{name}.{extension}");
            if let Some(parent) = Path::new(&out_path).parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let mut writer = std::io::BufWriter::new(fs::File::create(&out_path).unwrap());
            writeln!(writer, "{}", output).unwrap();
            println!("render complete: {name}");
        }
    }

    #[test]
    fn render() {
        render_target("RUST_AXUM", "rs");
        render_target("TYPESCRIPT_HONO", "ts");
    }
}
