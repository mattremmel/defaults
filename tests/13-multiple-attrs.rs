use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[default(value = "10")]
    #[default(value = "100")]
    x: usize,
}

fn main() {}
