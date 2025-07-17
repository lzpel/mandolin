use crate::JpList;
use regex;
use serde::Deserialize;

pub fn encode(content: &str) -> String {
	content.replace("~", "~0").replace("/", "~1") // RFC6901
}
pub fn decode(content: &str) -> String {
	content.replace("~0", "~").replace("~1", "/") // RFC6901
}
pub fn split(content: &str) -> Vec<String> {
	content.split('/').map(decode).collect()
}
pub fn re_replace(content: &str, re: &str, replace: &str) -> String {
	let re = regex::Regex::new(re).unwrap();
	re.replace_all(content, replace).to_string()
}
fn to_case_words(s: &str) -> Vec<String> {
	let re = regex::Regex::new(r"[A-Za-z0-9]+").unwrap();
	re.find_iter(s)
		.map(|m| m.as_str().to_ascii_lowercase())
		.collect()
}
pub fn to_snake_case(s: &str) -> String {
	to_case_words(s).join("_")
}
pub fn to_pascal_case(s: &str) -> String {
	to_case_words(s)
		.into_iter()
		.map(|word| {
			word.chars()
				.enumerate()
				.map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
				.collect::<String>()
		})
		.collect()
}
pub fn to_camel_case(s: &str) -> String {
	to_case_words(s)
		.into_iter()
		.enumerate()
		.map(|(i, word)| {
			if i > 0 {
				word.chars()
					.enumerate()
					.map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
					.collect::<String>()
			} else {
				word
			}
		})
		.collect()
}

pub fn include_ref(
	jp_list: &JpList,
	value: minijinja::Value,
) -> Result<minijinja::Value, minijinja::Error> {
	match openapiv3::ReferenceOr::<()>::deserialize(&value) {
		Ok(openapiv3::ReferenceOr::Reference { reference }) => {
			include_pointer(jp_list, reference.as_str())
		}
		_ => Ok(value),
	}
	/*
		pub fn include_ref(jp_list: &JpList, value: minijinja::Value) -> Result<minijinja::Value, minijinja::Error> {
			//valueが参照だったら参照を解決するだけの関数
			fn reference(
				jp_list: &JpList,
				value: minijinja::Value,
			) -> Result<minijinja::Value, minijinja::Error> {
				match openapiv3::ReferenceOr::<()>::deserialize(&value) {
					Ok(openapiv3::ReferenceOr::Reference { reference }) => jp_list
						.iter()
						.filter_map(|(a, b)| a.eq(&reference).then_some(b.clone()))
						.next()
						.ok_or(minijinja::Error::from(minijinja::ErrorKind::NonKey)),
					_ => Ok(value),
				}
			}
			//それを適用する関数
			let v = reference(jp_list, value)?;
			if let Ok(v) = BTreeMap::<minijinja::Value, minijinja::Value>::deserialize(&v) {
				return v
					.into_iter()
					.map(|(k, v)| reference(jp_list, v).map(|v| (k, v)))
					.collect();
			} else if let Ok(v) = Vec::<minijinja::Value>::deserialize(&v) {
				return v.into_iter().map(|v| reference(jp_list, v)).collect();
			}
			Ok(v)
		}
	*/
}

pub fn include_pointer(
	jp_list: &JpList,
	value: &str,
) -> Result<minijinja::Value, minijinja::Error> {
	jp_list
		.iter()
		.filter_map(|(a, b)| a.eq(value).then_some(b.clone()))
		.next()
		.ok_or(minijinja::Error::from(minijinja::ErrorKind::NonKey))
}
