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

	writeln!(file, "// templates templates").unwrap();
	for entry in path_read_dir {
		let filename = entry.unwrap().file_name();
		writeln!(file, "#[allow(unused_variables, dead_code)]").unwrap();
		writeln!(
			file,
			r##########"pub const {}: &'static str = r######"{}"######;"##########,
			Path::new(&filename)
				.file_stem()
				.unwrap_or(&filename)
				.to_str()
				.unwrap()
				.to_uppercase(),
			content(path_dir.join(filename)).unwrap()
		)
		.unwrap();
	}
}
fn content<P: AsRef<Path>>(path: P) -> io::Result<String> {
	let mut file = io::BufReader::new(fs::File::open(path)?);
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}
