use defaults::Defaults;

#[derive(Defaults)]
#[default(value = "A(10)")]
enum Foo {
    A(usize),
    B,
}

fn main() {
    if let Foo::A(a) = Foo::default() {
        assert_eq!(a, 10);
    } else {
        panic!("Expected A");
    }
}
