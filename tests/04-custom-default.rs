use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[default(value = "100")]
    x: usize,
    y: usize,
}

fn main() {
    let foo = Foo::default();
    assert_eq!(100, foo.x);
    assert_eq!(0, foo.y);
}
