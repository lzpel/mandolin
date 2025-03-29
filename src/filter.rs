use crate::JpList;
use serde::Deserialize;
use std::collections::BTreeMap;

pub fn encode(content: &str) -> String {
	content.replace("~", "~0").replace("/", "~1") // RFC6901
}
pub fn decode(content: &str) -> String {
	content.replace("~0", "~").replace("~1", "/") // RFC6901
}
pub fn split(content: &str) -> Vec<String> {
	content.split('/').map(decode).collect()
}
pub fn to_snake_case(s: &str) -> String {
	let mut result = String::new();
	let mut flag = false;
	for c in s.chars() {
		if c.is_ascii_alphanumeric() == false {
			flag = true; //前置判定
		} else {
			flag |= c.is_uppercase();
			flag &= !result.is_empty();
			//↑当該判定
			if flag {
				result.push('_')
			}
			result.push(c.to_ascii_lowercase());
			flag = false;
		}
	}
	result
}
pub fn to_pascal_case(s: &str) -> String {
	let mut result = String::new();
	let mut flag = false;
	for c in s.chars() {
		if c.is_ascii_alphanumeric() == false {
			flag = true; //前置判定
		} else {
			flag |= result.is_empty();
			//↑当該判定
			if flag {
				result.push(c.to_ascii_uppercase())
			} else {
				result.push(c)
			}
			flag = false;
		}
	}
	result
}

pub fn r(jp_list: &JpList, value: minijinja::Value) -> Result<minijinja::Value, minijinja::Error> {
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

pub fn point(jp_list: &JpList, value: &str) -> Result<minijinja::Value, minijinja::Error>{
	jp_list
		.iter()
		.filter_map(|(a, b)| a.eq(value).then_some(b.clone()))
		.next()
		.ok_or(minijinja::Error::from(minijinja::ErrorKind::NonKey))
}