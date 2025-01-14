use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
pub struct Empty{
}
#[derive(Serialize, Deserialize)]
pub struct Resource{
	pub list0: Vec<i32>,
	pub list1: Vec<i32>,
	pub list2: Vec<i32>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub list3: Vec<i32>,
	pub dict0: HashMap<String, i32>,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub dict1: HashMap<String, i32>,
	pub dict2: HashMap<String, i32>,
}

const TEMPLATE0: &'static str = r###"
{%for i in list0%}{{i}}{%endfor%}
{%for i in list1%}{{i}}{%endfor%}
{%for i in list2%}{{i}}{%endfor%}
{%for i in list3%}{{i}}{%endfor%}
"###;

const TEMPLATE1: &'static str = r###"
{%for i in "hello"|enumerate%}{{i}}{%endfor%}
"###;

const TEMPLATE2: &'static str = r###"
{%for i in list0%}{{i}}{%endfor%}
{%for i in list1%}{{i}}{%endfor%}
{%for i in list2%}{{i}}{%endfor%}
{%for i in list3|default(value=[]) %}{{i}}{%endfor%}
"###;

const TEMPLATE3: &'static str = r###"
{%for k,v in dict0%}{{k}}={{v}};{%endfor%}
{%for k,v in dict1|default(value=dict0)%}{{k}}={{v}};{%endfor%}
{%for k,v in dict2|empty%}{{k}}={{v}};{%endfor%}
{%for k,v in dict3|empty%}{{k}}={{v}};{%endfor%}
"###;

fn main (){
	let mut tera = Tera::default();
	let v=Resource{
		list0: vec![],
		list1: vec![1,2,3],
		list2: vec![4],
		list3: vec![],
		dict0: Default::default(),
		dict1: Default::default(),
		dict2: [("a".to_string(), 1)].into_iter().collect(),
	};
	let context=Context::from_serialize(&v).unwrap();
	//https://github.com/Keats/tera/blob/ae13d7ce39d732aae3f68435ed52c60732fe0ee0/src/renderer/processor.rs#L394
	tera.register_filter("empty", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
		println!("{}", value);
		if value.is_null(){
			Ok(tera::to_value(Empty{}).unwrap())
		}else{
			Ok(value.clone())
		}
	});
	tera.register_filter("enumerate", |value: &tera::Value, _: &HashMap<String, tera::Value>| {
		Ok(match value {
			tera::Value::String(v)=>tera::Value::Array(v.chars().map(|v| tera::Value::String(v.to_string())).collect()),
			_ => tera::Value::Null,
		})
	});
	render_print(&mut tera, TEMPLATE0, &context);
	render_print(&mut tera, TEMPLATE1, &context);
	render_print(&mut tera, TEMPLATE2, &context);
	render_print(&mut tera, TEMPLATE3, &context);
}
fn render_print(tera: &mut Tera, template :&str, context :&Context){
	println!("---\n{}", tera.render_str(template, &context).unwrap_or_else(|e| format!("{}", e.source().unwrap().to_string())));
}