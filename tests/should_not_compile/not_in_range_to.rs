use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
#[pattern(..3)]
enum Test {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

fn main() {
    let t = Test::One;
}
