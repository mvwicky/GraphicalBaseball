#[macro_use] extern crate nickel;
extern crate chrono;

use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};
use chrono::*;


fn now_as_str() -> String {
    let now: DateTime<Local> = Local::now();
    let n_str = now.to_string();
    n_str
}


fn main() {
    let mut server = Nickel::new();

    server.utilize(middleware! { |req|
        println!("logging request: {:?}", req.origin.uri);
        println!("{}", now_as_str());
    });

    let now_str = now_as_str();
    
    server.get("/", middleware! { |_, response| 
    	let mut data = HashMap::new();
    	data.insert("name", "Michael");

    	data.insert("date", &now_as_str());

    	return response.render("views/homepage.tpl", &data);
    });
    server.listen("127.0.0.1:6767");
}
