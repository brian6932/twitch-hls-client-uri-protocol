#![windows_subsystem = "windows"]

use std::{env::args, os::windows::process::CommandExt, process::Command};

fn main() {
	let uri = args().nth(1).expect("argument not provided");
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
		args.push(param.to_string());
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
