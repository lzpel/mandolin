//! テンプレート用の最小限フィルタ
//!
//! Jinjaテンプレートでは困難な処理のみRustで提供する:
//! - ケース変換 (heckクレート)
//! - RFC6901 JSON Pointerのエンコード/デコード
//! - 正規表現による置換

use heck::{ToLowerCamelCase, ToPascalCase, ToSnakeCase};

/// スネークケースに変換する (例: "MyType" → "my_type")
pub fn to_snake_case(s: &str) -> String {
    s.to_snake_case()
}

/// パスカルケースに変換する (例: "my_type" → "MyType")
pub fn to_pascal_case(s: &str) -> String {
    s.to_pascal_case()
}

/// キャメルケースに変換する (例: "my_type" → "myType")
pub fn to_camel_case(s: &str) -> String {
    s.to_lower_camel_case()
}

/// RFC6901 JSON Pointerエンコード: ~ → ~0, / → ~1
pub fn encode(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}

/// RFC6901 JSON Pointerデコード: ~1 → /, ~0 → ~
pub fn decode(s: &str) -> String {
    s.replace("~1", "/").replace("~0", "~")
}

/// 正規表現による文字列置換
/// テンプレートからAxumのワイルドカードパス変換等に使用
pub fn re_replace(content: &str, re: &str, replace: &str) -> String {
    regex::Regex::new(re)
        .unwrap()
        .replace_all(content, replace)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        assert_eq!(encode("application/json"), "application~1json");
        assert_eq!(decode("application~1json"), "application/json");
        assert_eq!(decode(&encode("a/b~c")), "a/b~c");
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_snake_case("MyPascalCase"), "my_pascal_case");
        assert_eq!(to_pascal_case("my_snake_case"), "MySnakeCase");
        assert_eq!(to_camel_case("my_snake_case"), "mySnakeCase");
    }

    #[test]
    fn test_re_replace() {
        assert_eq!(
            re_replace("/files/{path_file}", r"(\{\s*)path", r"$1*path"),
            "/files/{*path_file}"
        );
    }
}
