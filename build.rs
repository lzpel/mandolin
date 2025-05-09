use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
fn main() {
	let mut file = {
		let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("templates.rs");
		fs::File::create(&dest_path).unwrap()
	};
	let templates = get_templates();
	writeln!(file, "// templates templates").unwrap();
	writeln!(file, "#[allow(unused_variables, dead_code)]").unwrap();
	for (_path, name, content) in &templates {
		writeln!(file, "#[allow(unused_variables, dead_code)]").unwrap();
		writeln!(
			file,
			r##########"pub const {name}: &'static str = r######"{content}"######;"##########
		)
		.unwrap();
	}
	writeln!(
		file,
		"pub const TEMPLATES: [[&'static str; 2]; {}] = [",
		templates.len()
	)
	.unwrap();
	for (_path, name, _content) in &templates {
		writeln!(file, r#"	["{name}", {name}],"#).unwrap();
	}
	writeln!(file, "];").unwrap();
	// memo
	writeln!(file, "/*").unwrap();
	for (path, name, _content) in &templates {
		let path = path.replace(std::path::MAIN_SEPARATOR_STR, "/");
		writeln!(
			file,
			r#"let content = fs::read_to_string("{path}").unwrap();"#
		)
		.unwrap();
		writeln!(
			file,
			r#"env.add_template("{name}", content.as_str()).unwrap();"#
		)
		.unwrap();
	}
	writeln!(file, "*/").unwrap();
}

fn get_templates() -> Vec<(String, String, String)> {
	// return Vec<(template name: String, template text: String)> from ./templates directory
	let path_read_dir = fs::read_dir(&Path::new(".").join("templates")).unwrap();
	path_read_dir
		.filter_map(Result::ok)
		.map(|entry| {
			let filename = entry.file_name();
			let filename_without_extension = Path::new(&filename).file_stem().unwrap_or_default();
			let filename_with_upper = filename_without_extension.to_str().unwrap().to_uppercase();
			(
				entry.path().to_str().unwrap().to_string(),
				filename_with_upper,
				fs::read_to_string(entry.path()).unwrap(),
			)
		})
		.collect()
}
