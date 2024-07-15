# i1-templates-8n

Multi-lingual templates done the rust way.

If it compiles, it works.

> [!CAUTION]
> This is very much a pre-alpha stage.
> The API is incomplete and subject to more unexpected changes
> than you can even imagine.

## Example

Check the `example` crate for a full usage demo.

Put templates under `/templates/{lang}/{template}`:
```
Cargo.toml
templates/
    en/
        hello.txt
        ..
    fr/
        hello.txt
    ..
..
```

The placeholder format is the same as for rust `format!` macro.
Use single curly brackets: `Hello {world}`.

```rust
use i1_templates_8n::{Template, typed_langid};

#[derive(Template)]
#[template(name = "hello.txt")]   // define the template name
struct HelloWorld {
    world: String,
    #[template(skip)]   // this field won't be accessible from templates
    _secret: usize,
}

fn main() {
    // initialize rendering context
    let x = HelloWorld {
        world: "Galaxy".to_string(),
        _secret: 42,
    };

    // render template for languages
    println!("en: {:?}", x.render(typed_langid::En));
    println!("fr: {:?}", x.render(typed_langid::Fr));
}
```

The `#[derive(Template)]` macro will implement `Template<L: LangId>` for
all languages that contain the template in their respective templates
directory:
`/templates/en/hello.txt` will match with `typed_langid::En`,
`/templates/fr/hello.txt` will match with `typed_langid::Fr`.
