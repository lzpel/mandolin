use minijinja;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Empty {}
#[derive(Serialize, Deserialize)]
pub struct Resource {
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
{%set a="hello world"%}
{%for i in a|enumerate%}{{i}}/{%endfor%}
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

fn main() {
	render_print(TEMPLATE0);
	render_print(TEMPLATE1);
	render_print(TEMPLATE2);
	render_print(TEMPLATE3);
}
fn render_print(template: &str) {
	let r = Resource {
		list0: vec![6],
		list1: vec![1, 2, 3],
		list2: vec![4],
		list3: vec![],
		dict0: Default::default(),
		dict1: Default::default(),
		dict2: [("a".to_string(), 1)].into_iter().collect(),
	};
	let minijinja_result = {
		let mut env = minijinja::Environment::new();
		env.add_template("hello.txt", template).unwrap();
		env.add_filter("enumerate", |v: Cow<'_, str>| {
			Ok(v.chars().map(|v| v.to_string()).collect::<Vec<String>>())
		});
		let template = env.get_template("hello.txt").unwrap();
		template.render(&r).unwrap_or_else(|e| format!("{:?}", e))
	};
	println!("{}", minijinja_result);
}
