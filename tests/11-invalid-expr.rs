use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[default(value = "1/")]
    x: usize,
}

fn main() {}
