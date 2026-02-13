//! OpenAPI仕様の$ref参照を事前解決するモジュール
//!
//! openapiv3::OpenAPIを拡張し、$refを全てインライン展開した
//! serde_json::Valueを生成する。テンプレートはこの解決済みデータを直接操作する。

use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

/// openapiv3::OpenAPIを拡張した型
/// 型付きAPIはbase経由で使い、テンプレートにはresolved_value()を渡す
#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedOpenAPI {
    #[serde(flatten)]
    pub base: OpenAPI,
}

impl ResolvedOpenAPI {
    /// OpenAPI仕様を受け取り、デフォルトサーバを補完する
    pub fn new(mut spec: OpenAPI) -> Self {
        if spec.servers.is_empty() {
            spec.servers.push(openapiv3::Server {
                url: "/api".to_string(),
                description: Some("Default server added by mandolin".to_string()),
                ..Default::default()
            });
        }
        Self { base: spec }
    }

    /// $refを全て事前解決したJSON Valueを返す
    /// テンプレートエンジンにはこの値を渡す
    pub fn resolved_value(&self) -> Value {
        let mut value = serde_json::to_value(&self.base).unwrap();
        let source = value.clone();
        resolve_recursive(&mut value, &source, &mut HashSet::new());
        value
    }
}

impl From<OpenAPI> for ResolvedOpenAPI {
    fn from(spec: OpenAPI) -> Self {
        Self::new(spec)
    }
}

/// JSON Valueツリーを再帰的に走査し、$refを解決する
/// visitingセットで循環参照を検出し、無限ループを防止する
fn resolve_recursive(node: &mut Value, root: &Value, visiting: &mut HashSet<String>) {
    match node {
        Value::Object(map) => {
            // $refオブジェクトを検出したら参照先の値で置換する
            if let Some(Value::String(ref_path)) = map.get("$ref") {
                let ref_path = ref_path.clone();
                if let Some(resolved) = lookup_pointer(root, &ref_path) {
                    if !visiting.contains(&ref_path) {
                        // 循環参照ガード: 訪問中としてマークしてから再帰
                        visiting.insert(ref_path.clone());
                        let mut resolved = resolved.clone();
                        resolve_recursive(&mut resolved, root, visiting);
                        visiting.remove(&ref_path);
                        *node = resolved;
                        return;
                    }
                    // 循環参照の場合は$refをそのまま残す（テンプレート側でフォールバック）
                }
            }
            // 子要素を再帰的に解決
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

/// JSON Pointer文字列からルートオブジェクト内の値を探索する
/// 例: "#/components/schemas/Pet" → root["components"]["schemas"]["Pet"]
fn lookup_pointer<'a>(root: &'a Value, ref_path: &str) -> Option<&'a Value> {
    let path = ref_path.strip_prefix("#/")?;
    let mut current = root;
    for segment in path.split('/') {
        // RFC6901デコード: ~1→/, ~0→~
        let decoded = segment.replace("~1", "/").replace("~0", "~");
        current = current.get(&decoded)?;
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_simple_ref() {
        let json: Value = serde_json::json!({
            "components": {
                "schemas": {
                    "Pet": { "type": "object", "properties": { "name": { "type": "string" } } }
                }
            },
            "paths": {
                "/pets": {
                    "get": {
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/Pet" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        let mut resolved = json.clone();
        let source = resolved.clone();
        resolve_recursive(&mut resolved, &source, &mut HashSet::new());
        // $refが解決されてインライン化されていることを確認
        let schema = &resolved["paths"]["/pets"]["get"]["responses"]["200"]["content"]["application/json"]["schema"];
        assert_eq!(schema["type"], "object");
        assert!(schema.get("$ref").is_none());
    }

    #[test]
    fn test_circular_ref_preserved() {
        let json: Value = serde_json::json!({
            "components": {
                "schemas": {
                    "Node": {
                        "type": "object",
                        "properties": {
                            "child": { "$ref": "#/components/schemas/Node" }
                        }
                    }
                }
            }
        });
        let mut resolved = json.clone();
        let source = resolved.clone();
        resolve_recursive(&mut resolved, &source, &mut HashSet::new());
        // 1段目: $refは解決されてNodeオブジェクトに展開される
        let child = &resolved["components"]["schemas"]["Node"]["properties"]["child"];
        assert_eq!(child["type"], "object");
        // 2段目: 循環部分は$refのまま残る（無限展開を防止）
        let grandchild = &child["properties"]["child"];
        assert!(grandchild.get("$ref").is_some());
    }
}
