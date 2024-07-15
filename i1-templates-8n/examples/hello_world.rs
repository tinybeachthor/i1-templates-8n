use i1_templates_8n::{Template, typed_langid};

#[derive(Template)]
#[template(name = "hello_world.txt")]
struct HelloWorld {
    hello: String,
    world: String,
    #[template(skip)]
    _secret: usize,
}

fn main() {
    let x = HelloWorld {
        hello: "Howdy".to_string(),
        world: "Galaxy".to_string(),
        _secret: 42,
    };
    println!("{:?}", x.render(typed_langid::En));
}
