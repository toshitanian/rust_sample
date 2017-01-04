extern crate hyper;
use hyper::Client;
use std::io::Read;
use std::sync::Arc;
use std::thread;

fn main() {

    // use http
    let client = Client::new();

    let url = "http://example.com";
    let mut res = client.get(url).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Result:: url:{}, body_lenght:{}, status_code:{}", url, body.len(), res.status);

    // use https
    let url = "https://example.com";
    let mut res = client.get(url).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Result:: url:{}, body_lenght:{}, status_code:{}", url, body.len(), res.status);

    // use thread
	let client = Arc::new(Client::new());
	let clone1 = client.clone();
	let child = thread::spawn(move || {
	    let url = "http://example.com";
	    let mut res = clone1.get(url).send().unwrap();
		let mut body = String::new();
	    res.read_to_string(&mut body).unwrap();
		println!("Result(thread):: url:{}, body_lenght:{}, status_code:{}", url, body.len(), res.status);
	});
	println!("after thread");
	child.join();
	    println!("after join");
}
