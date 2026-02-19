//! mandolin — OpenAPI仕様からサーバコードを生成するテンプレートエンジン
//!
//! # 設計方針
//! - Rustは「データの準備」のみ行い、「コードの組み立て」は全てテンプレートに任せる
//! - $refは解決せずそのままテンプレートに渡す（2段階生成で対処）
//! - Rustのフィルタはケース変換・正規表現などJinjaでは困難な処理のみ提供する

mod filter;
mod schema_cache;
/// ビルド時に生成されるテンプレート定数
pub mod templates {
    include!(concat!(env!("OUT_DIR"), "/templates.rs"));
}

use openapiv3::OpenAPI;

/// OpenAPI仕様からテンプレート環境を構築する
///
/// 1. デフォルトサーバを補完
/// 2. $ref展開なし、そのままJSON Valueとしてテンプレートに渡す
/// 3. 最小限のフィルタ・関数を登録
///
/// テンプレートは2段階生成（Phase 1: components/schemas → Phase 2: paths）で
/// $refを型名として直接参照する。
pub fn environment(mut spec: OpenAPI) -> Result<minijinja::Environment<'static>, minijinja::Error> {
    // デフォルトサーバを補完
    if spec.servers.is_empty() {
        spec.servers.push(openapiv3::Server {
            url: "/api".to_string(),
            description: Some("Default server added by mandolin".to_string()),
            ..Default::default()
        });
    }

    // $ref展開なし、そのままシリアライズ
    let value = serde_json::to_value(&spec).unwrap();

    let mut env = minijinja::Environment::new();
    for [k, v] in templates::TEMPLATES {
        env.add_template(k, v)?;
    }

    // $ref展開なしのスペックをグローバル変数として設定
    env.add_global("spec", minijinja::Value::from_serialize(&value));

    // フィルタ登録（Jinjaでは困難な言語機能のみ）
    env.add_filter("to_snake_case", filter::to_snake_case);
    env.add_filter("to_pascal_case", filter::to_pascal_case);
    env.add_filter("to_camel_case", filter::to_camel_case);
    env.add_filter("re_replace", filter::re_replace);
    env.add_filter("encode", filter::encode);
    env.add_filter("decode", filter::decode);
    // $refパスから型名を取り出すフィルタ: "#/components/schemas/Foo" → "Foo"
    env.add_filter("ref_name", filter::ref_name);

    // include_pointerフィルタ: JSON Pointerで未解決specから値を取得する
    // SCHEMA_NAMEマクロがインライン匿名スキーマの内容を参照するために必要
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

    // derefフィルタ: $refオブジェクトを実体に解決する
    // parameters/requestBody/responsesなど構造的な$refに使用
    // $refでない値はそのまま返す
    {
        let spec_value = value.clone();
        env.add_filter(
            "deref",
            move |v: minijinja::Value| -> Result<minijinja::Value, minijinja::Error> {
                if let Ok(ref_val) = v.get_item(&minijinja::Value::from("$ref")) {
                    if let Some(ref_path) = ref_val.as_str() {
                        let path = ref_path.strip_prefix("#/").unwrap_or(ref_path);
                        let mut cur = &spec_value;
                        for seg in path.split('/') {
                            let decoded = seg.replace("~1", "/").replace("~0", "~");
                            cur = cur.get(&decoded).ok_or_else(|| {
                                minijinja::Error::new(
                                    minijinja::ErrorKind::InvalidOperation,
                                    format!("deref: not found: {ref_path}"),
                                )
                            })?;
                        }
                        return Ok(minijinja::Value::from_serialize(cur));
                    }
                }
                Ok(v)
            },
        );
    }

    // スキーマキャッシュ（インライン匿名スキーマの重複排除用）
    // named schemasはPhase 1で直接出力するためキャッシュ不要
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

    // anyof_tag: anyOfスキーマの暗黙的discriminatorを検出する
    // 全variantが共通の単一値enumプロパティを持つ場合、そのプロパティ名を返す
    // 例: ShapeNodeの各variantが op: {enum: ["step"]} 等を持つ場合 → "op"
    {
        let spec_value = value.clone();
        env.add_function("anyof_tag", move |schema: minijinja::Value| -> String {
            let any_of = match schema.get_item(&minijinja::Value::from("anyOf")) {
                Ok(v) if !v.is_undefined() && !v.is_none() => v,
                _ => return String::new(),
            };
            let mut disc_prop: Option<String> = None;
            for i in 0.. {
                let item = match any_of.get_item(&minijinja::Value::from(i)) {
                    Ok(v) if !v.is_undefined() => v,
                    _ => break,
                };
                // $refでなければスキップ（inline型はdiscriminatorを持たない）
                let ref_path = match item.get_item(&minijinja::Value::from("$ref")) {
                    Ok(v) => match v.as_str() {
                        Some(s) => s.to_string(),
                        None => return String::new(),
                    },
                    _ => return String::new(),
                };
                // $refを解決
                let path = match ref_path.strip_prefix("#/") {
                    Some(p) => p,
                    None => return String::new(),
                };
                let mut current = &spec_value;
                for segment in path.split('/') {
                    let decoded = segment.replace("~1", "/").replace("~0", "~");
                    current = match current.get(&decoded) {
                        Some(v) => v,
                        None => return String::new(),
                    };
                }
                // 単一値enumプロパティを探す
                let properties = match current.get("properties") {
                    Some(serde_json::Value::Object(m)) => m,
                    _ => return String::new(),
                };
                let mut found = None;
                for (k, v) in properties {
                    if let Some(arr) = v.get("enum").and_then(|e| e.as_array()) {
                        if arr.len() == 1 {
                            found = Some(k.clone());
                            break;
                        }
                    }
                }
                match (&disc_prop, found) {
                    (None, Some(prop)) => disc_prop = Some(prop),
                    (Some(existing), Some(ref prop)) if existing == prop => {}
                    _ => return String::new(),
                }
            }
            disc_prop.unwrap_or_default()
        });
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
                let name = path.file_stem()?.to_str()?.to_string();
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

    /// 指定テンプレートで全OpenAPIファイルをレンダリングし、examples/に出力する
    fn render_target(template: &str, extension: &str) {
        for (name, api) in api_map() {
            println!("render start: {name}");
            let env = environment(api).unwrap();
            let tmpl = env.get_template(template).unwrap();
            let output = tmpl.render(0).unwrap();
            let out_path = format!("examples/{name}.{extension}");
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
