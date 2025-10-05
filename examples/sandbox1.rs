fn main() {
	fn x(y: i32) -> Result<String, String> {
		for i in 0..3 {
			if i == y {
				return Err("a".to_string());
			}
		}
		Ok("ok".to_string())
	}
	match x(2) {
		Ok(v) => println!("{v}"),
		Err(v) => println!("{v}"),
	}
}
