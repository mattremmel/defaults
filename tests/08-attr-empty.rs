use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[default]
    x: usize,
    y: usize,
}

fn main() {}
