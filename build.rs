use std::env;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
	let mut file = {
		let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("templates.rs");
		fs::File::create(&dest_path).unwrap()
	};
	let path_dir = Path::new(".").join("templates");
	let path_read_dir = fs::read_dir(&path_dir).unwrap();
	let map_name_content: Vec<(String, String)> = path_read_dir
		.filter_map(Result::ok)
		.map(|entry| {
			let filename = entry.file_name();
			let filename_without_extension = Path::new(&filename).file_stem().unwrap_or_default();
			let name = filename_without_extension.to_str().unwrap().to_uppercase();
			(name, content(entry.path()).unwrap())
		})
		.collect();

	writeln!(file, "// templates templates").unwrap();
	writeln!(file, "#[allow(unused_variables, dead_code)]").unwrap();
	for (name, content) in &map_name_content {
		writeln!(file, "#[allow(unused_variables, dead_code)]").unwrap();
		writeln!(
			file,
			r##########"pub const {name}: &'static str = r######"{content}"######;"##########
		)
		.unwrap();
	}
	writeln!(file, "pub fn templates() -> std::collections::HashMap<String, String> {{").unwrap();
	writeln!(file, "	[").unwrap();
	for (name, _content) in &map_name_content {
		writeln!(file, r#"		["{name}", {name}],"#).unwrap();
	}
	writeln!(file, "	].iter().map(|[a,b]| (a.to_string(), b.to_string())).collect()").unwrap();
	writeln!(file, "}}").unwrap();
}
fn content<P: AsRef<Path>>(path: P) -> io::Result<String> {
	let mut file = io::BufReader::new(fs::File::open(path)?);
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}
pub const BUILTIN: [[&'static str; 2]; 1] = [["", ""]];
