use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
#[pattern(1..=3)]
enum Test {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

fn main() {
    let t = Test::One;
}
