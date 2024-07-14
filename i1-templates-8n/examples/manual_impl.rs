use i1_templates_8n::Template;

struct HelloWorld {
    hello: String,
    world: String,
}
impl Template for HelloWorld {
    type Output = String;

    fn render(&self) -> Self::Output {
        let HelloWorld {
            hello,
            world,
        } = self;
        std::format!("{hello} {world}!")
    }
}

fn main() {
    let x = HelloWorld {
        hello: "Howdy".to_string(),
        world: "Galaxy".to_string(),
    };
    println!("{:?}", x.render());
}

