use std::{fs, env, process};
extern crate rustc_serialize;
use rustc_serialize::json;
extern crate getopts;
use getopts::Options;


#[derive(RustcDecodable, RustcEncodable)]
struct DirectoryStructure {
    path: String,
    files: Vec<String>
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} TARGET_DIR [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "pretty", "pretty print");

    let matches = opts.parse(&args[1..])
      .unwrap_or_else(|f| panic!(f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }
    let mut pretty_print = false;
    if matches.opt_present("p") {
        pretty_print = true;
    }
    if matches.free.is_empty() {
        print_usage(&program, &opts);
    }
    let ref target_path = matches.free[0];

    let paths: Vec<String> = fs::read_dir(target_path).unwrap().map(|res| res.unwrap().path().display().to_string()).collect();

    let object = DirectoryStructure {
        path: target_path.to_string(),
        files: paths
    };
    let encoded = if pretty_print {
        json::as_pretty_json(&object).to_string()
    } else {
        json::as_json(&object).to_string()
    };
    println!("{}", encoded);
}
