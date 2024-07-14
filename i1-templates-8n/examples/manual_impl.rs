use i1_templates_8n::{Template, typed_langid};

struct HelloWorld {
    hello: String,
    world: String,
}
impl Template<typed_langid::En> for HelloWorld {
    type Output = String;

    fn render(&self, _lang: typed_langid::En) -> Self::Output {
        let HelloWorld {
            hello,
            world,
        } = self;
        std::format!("{hello} {world}!")
    }
}
impl Template<typed_langid::Fr> for HelloWorld {
    type Output = String;

    fn render(&self, _lang: typed_langid::Fr) -> Self::Output {
        let HelloWorld {
            hello,
            world,
        } = self;
        std::format!("{world} {hello}!")
    }
}

fn main() {
    let template = HelloWorld {
        hello: "Howdy".to_string(),
        world: "Galaxy".to_string(),
    };

    render(template)
}

fn render<T>(template: T)
where
    T: Template<typed_langid::En, Output = String>,
    T: Template<typed_langid::Fr, Output = String>,
{
    println!("{:?}", template.render(typed_langid::En));
    println!("{:?}", template.render(typed_langid::Fr));
}
