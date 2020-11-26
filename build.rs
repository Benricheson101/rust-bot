use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let in_path_contents = fs::read_to_string("config.json").expect("Unable to find config file!");
	let dest_path = Path::new(&out_dir).join("config.json");

	fs::write(dest_path, in_path_contents).unwrap();
}
