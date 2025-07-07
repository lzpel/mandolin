use std::collections::HashMap;

use crate::{filter, JpList};
use serde::Deserialize;

pub fn jp_list(value: &minijinja::Value, prefix: &str) -> JpList {
	let mut output = Default::default();
	fn recursive(path: String, value: &minijinja::Value, output: &mut JpList) {
		output.push((path.clone(), value.clone())); //注目箇所を追加
		if let Some(v) = value.as_object() {
			//子を検索
			if let Some(v) = v.try_iter_pairs() {
				v.for_each(|(k, v)| {
					recursive(
						format!("{path}/{}", filter::encode(k.as_str().unwrap_or_default())),
						&v,
						output,
					)
				});
			} else if let Some(v) = v.try_iter() {
				v.enumerate()
					.for_each(|(k, v)| recursive(format!("{path}/{k}"), &v, output));
			}
		}
	}
	recursive(prefix.to_string(), value, &mut output);
	output
}
pub enum LsMode<'a> {
	LS((&'a str, bool)),
	SCHEMA,
	OPERATION,
}
pub fn ls(input: &JpList, mode: LsMode) -> minijinja::Value {
	let methods = [
		"get", "put", "post", "delete", "options", "head", "patch", "trace",
	];
	let ret: JpList = input
		.iter()
		.filter(|(k, v)| match mode {
			LsMode::LS((path, recursive)) => k
				.strip_prefix(&path)
				.is_some_and(|v| recursive || !v.contains("/")),
			LsMode::SCHEMA => {
				openapiv3::Schema::deserialize(v).is_ok_and(|v| match v.schema_kind {
					openapiv3::SchemaKind::Type(_) => true,
					_ => false,
				})
			}
			LsMode::OPERATION => k.strip_prefix("#/paths/").is_some_and(|v| {
				let mut w = v.split("/");
				w.next().is_some()
					&& w.next().is_some_and(|v| methods.contains(&v))
					&& w.next().is_none()
			}),
		})
		.cloned()
		.collect();
	minijinja::Value::from_serialize(ret)
}

pub type NestedStruct = HashMap<String, Option<String>>;

pub fn struct_clean(structs: &mut NestedStruct) ->NestedStruct{
	eprintln!("@@@@ clean {:?}", structs);
    let mut drained = HashMap::new();
    for (key, value) in structs.iter_mut() {
        if let Some(v) = value.take() {
            drained.insert(key.clone(), Some(v));
        }
        // valueはtake()されたのでNoneになる
    }
    drained
}

pub fn struct_push(structs: &mut NestedStruct, pointer: &str, content: Option<&str>) ->bool{
	eprintln!("@@@@ push {:?}", pointer);
	let is_first = !structs.contains_key(pointer);
	structs.insert(pointer.to_string(), content.map(|v| v.to_string()));
	return is_first;
}