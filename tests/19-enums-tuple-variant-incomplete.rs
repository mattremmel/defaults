use defaults::Defaults;

#[derive(Defaults)]
#[default(value = "A")]
enum Foo {
    A(usize),
    B,
}

fn main() {}
