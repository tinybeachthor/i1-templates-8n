use i1_templates_8n::{Template, typed_langid};

#[derive(Template)]
#[template(name = "hello_world.txt")]
struct HelloWorld {
    world: String,
    #[template(skip)]
    _secret: usize,
}

fn main() {
    let x = HelloWorld {
        world: "Galaxy".to_string(),
        _secret: 42,
    };
    println!("en: {:?}", x.render(typed_langid::En));
    println!("fr: {:?}", x.render(typed_langid::Fr));
}
