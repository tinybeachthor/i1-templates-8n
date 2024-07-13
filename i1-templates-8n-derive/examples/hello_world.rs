#![allow(dead_code)]

use i1_templates_8n_derive::Template;

#[derive(Template)]
#[template(path = "hello_world.txt")]
struct HelloWorld {
    hello: String,
    world: usize,
    #[template(skip)]
    internal: bool,
}

fn main() {
    println!("{:?}", HelloWorld::FIELDS);
    println!("{:?}", HelloWorld::TEMPLATE);
}
