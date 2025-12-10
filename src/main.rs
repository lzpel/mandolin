use clap::Parser;
use mandolin;
use serde_yaml;
use std::io::Read;
#[derive(Parser, Debug)]
#[clap(author, version, about = "Input openapi.json/yaml, output server source code in rust.", long_about = None)]
struct Args {
	#[clap(
		short = 'i',
		long = "input",
		value_name = "FILE",
		help = "Sets the input JSON/YAML file. If '-' specified, stdin will be used."
	)]
	input: String,
	#[clap(
		short = 'o',
		long = "output",
		value_name = "FILE",
		help = "Sets the output source file. If omitted, stdout will be used."
	)]
	output: Option<String>,
	#[clap(
		short = 't',
		long = "template",
		value_name = "TEMPLATE",
		default_value = "RUST_AXUM",
		help = "Sets the template name"
	)]
	template: String,
}
pub fn main() {
	// コマンドライン引数をパース
	let args = Args::parse();
	let input = match args.input.as_str() {
		"-" => {
			// Noneの場合: 標準入力から読み込む
			let mut buffer = String::new();
			if let Err(e) = std::io::stdin().read_to_string(&mut buffer) {
				eprintln!("Cannot read from stdin\n{:?}", e);
				std::process::exit(1);
			}
			buffer
		}
		filename => {
			// Someの場合: ファイルから読み込む
			std::fs::read_to_string(filename).unwrap_or_else(|e| {
				eprintln!("Cannot read {}\n{:?}", filename, e);
				std::process::exit(1);
			})
		}
	};

	let input_api = match [
		serde_yaml::from_str(input.as_str()).map_err(|_| ()),
		serde_json::from_str(input.as_str()).map_err(|_| ()),
	]
	.into_iter()
	.find_map(Result::ok)
	{
		Some(v) => v,
		None => {
			eprintln!("Cannot parse input as json/yaml");
			std::process::exit(1);
		}
	};
	let env = mandolin::environment(input_api).unwrap();
	let template = env.get_template(args.template.as_str()).unwrap();
	let output = template.render(0).unwrap();
	// write the rendered output
	match &args.output {
		Some(v) => {
			std::fs::write(v.as_str(), output).unwrap();
		}
		None => {
			print!("{}", output);
		}
	}
}
