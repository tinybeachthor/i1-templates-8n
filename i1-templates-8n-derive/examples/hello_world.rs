#![allow(dead_code)]

use i1_templates_8n_derive::Template;

#[derive(Template)]
struct HelloWorld {
    hello: String,
    world: usize,
    #[template(skip)]
    internal: bool,
}

fn main() {
    println!("{:?}", HelloWorld::FIELDS);
}
