use i1_templates_8n::{Template, Result};

struct HelloWorld {
    hello: String,
    world: String,
}
impl Template for HelloWorld {
    fn render_into(&self, writer: &mut (impl std::fmt::Write + ?Sized)) -> Result<()> {
        let HelloWorld {
            hello,
            world,
        } = self;
        std::write!(writer, "{hello} {world}!")?;
        Ok(())
    }

    const SIZE_HINT: usize = 256;
}

fn main() {
    let x = HelloWorld {
        hello: "Howdy".to_string(),
        world: "Galaxy".to_string(),
    };
    println!("{:?}", x.render());
}

