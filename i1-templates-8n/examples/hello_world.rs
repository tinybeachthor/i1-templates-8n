#![allow(dead_code)]

use i1_templates_8n::Template;

#[derive(Template)]
#[template(path = "hello_world.txt")]
struct HelloWorld {
    hello: String,
    world: String,
    #[template(skip)]
    secret: usize,
}

fn main() {
    println!("{:?}", HelloWorld::TEMPLATE);

    let x = HelloWorld {
        hello: "Howdy".to_string(),
        world: "Galaxy".to_string(),
        secret: 42,
    };
    println!("{:?}", x.render());
}