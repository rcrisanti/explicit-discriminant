use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
enum Test {
    One = 1,
    Two = 2,
    Three,
    Four = 4,
}

fn main() {
    let t = Test::One;
}
