use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
#[pattern(1)]
enum Test {
    Zero = 0,
    One = 1,
}

fn main() {
    let t = Test::One;
}
