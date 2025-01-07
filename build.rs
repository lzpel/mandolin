use std::fs;
use std::env;
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::File;
use std::io::Read;

fn main() {
	let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("templates.rs");
	let mut file = fs::File::create(&dest_path).unwrap();

	let path_dir=Path::new(".").join("templates");
	let paths = fs::read_dir(&path_dir).unwrap();

	writeln!(file, "// templates templates").unwrap();
	for entry in paths {
		let filename = entry.unwrap().file_name();
		writeln!(file, "#[allow(unused_variables, dead_code)]").unwrap();
		writeln!(
			file,
			r##########"pub const {}: &'static str = r######"{}"######;"##########,
			Path::new(&filename).file_stem().unwrap_or(&filename).to_str().unwrap().to_uppercase(),
			content(path_dir.join(filename)).unwrap()
		).unwrap();
	}
}
fn content<P: AsRef<Path>>(path: P) -> io::Result<String>{
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}
