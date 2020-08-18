use defaults::Defaults;

struct NonDefault(usize);

#[derive(Defaults)]
struct Foo {
    x: usize,
    #[default(value = "NonDefault(10)")]
    non_default: NonDefault,
}

fn main() {}
