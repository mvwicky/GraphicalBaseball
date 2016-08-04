#[macro_use] extern crate nickel;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};


fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware! { |_, response|
    	let mut data = HashMap::new();
    	data.insert("name", "Michael");
    	data.insert("date", "8-3-2016");
    	return response.render("src/homepage.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}
