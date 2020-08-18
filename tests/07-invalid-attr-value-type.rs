use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[default(value = 10)]
    x: usize,
    y: usize,
}

fn main() {}
