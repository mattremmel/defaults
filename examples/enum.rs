use defaults::Defaults;

#[derive(Debug, Defaults)]
#[default(value = "B")]
pub enum Classroom {
    A,
    B,
    C,
}

fn main() {
    println!("{:?}", Classroom::default());
}
