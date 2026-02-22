//! OpenAPI 仕様の読み込みモジュール
//!
//! `yaml` feature が有効な場合は YAML としてパースを試みて、
//! 失敗した場合は JSON にフォールバックする。
//! `yaml` feature が無効な場合は常に JSON としてパースする。

use std::io::Read;

use openapiv3::OpenAPI;

/// `Read` 実装から OpenAPI 仕様をパースして返す。
pub fn openapi_load<R: Read>(mut reader: R) -> Result<OpenAPI, Box<dyn std::error::Error>> {
	let mut buf = String::new();
	reader.read_to_string(&mut buf)?;
	openapi_parse_str(&buf)
}

/// 文字列から OpenAPI 仕様をパースして返す。
pub fn openapi_parse_str(s: &str) -> Result<OpenAPI, Box<dyn std::error::Error>> {
	#[cfg(feature = "yaml")]
	if let Ok(api) = serde_yaml::from_str(s) {
		return Ok(api);
	}
	Ok(serde_json::from_str(s)?)
}
