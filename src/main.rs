#![windows_subsystem = "windows"]

use std::{env::args, os::windows::process::CommandExt, process::Command};

fn main() {
	let mut uri = args().nth(1).expect("argument not provided");
	if uri.as_str().starts_with("https%") {
		// Since Chatterino/chatterino2@af573484c41d1db37014567b6dde054b11e32e5c:
		// The argument is percent encoded, so decode it first.
		uri = percent_encoding::percent_decode_str(&uri)
			.decode_utf8_lossy()
			.into_owned();
	}
	let parsed_uri = uriparse::URI::try_from(uri.as_str()).expect("invalid URI");
	let path = parsed_uri.path().to_string();

	if path.is_empty() {
		panic!("no user found")
	}

	// parsed_uri.query() doesn't expose any lengths,
	// adding it's capacity would require 2 iterations.
	let mut args: Vec<String> = Vec::with_capacity(2);

	args.push(path);
	for param in parsed_uri.query().iter() {
		let parsed_param = param.to_string();
		if !parsed_param.is_empty() {
			args.push(parsed_param);
		}
	}
	if args.len() == 1 {
		args.push("best".to_string());
	}

	const CREATE_NO_WINDOW: u32 = 0x08000000;

	let mut proc = Command::new("twitch-hls-client.exe")
		.args(args)
		.creation_flags(CREATE_NO_WINDOW)
		.spawn()
		.unwrap();

	proc.wait().unwrap();
}
