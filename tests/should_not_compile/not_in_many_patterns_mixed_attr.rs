use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
#[pattern(..=-1, 4)]
#[pattern((7 | 9..=12) | 19..)]
#[pattern(15..17)]
enum Test {
    VeryNegative = -9999999,
    LittleNegative = -1,
    Zero = 0,
    Four = 4,
    Five = 5,
    Seven = 7,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Fifteen = 15,
    Sixteen = 16,
    Nineteen = 19,
    Eighteen = 18,
    VeryPositive = 123921380,
}

fn main() {
    let t = Test::Zero;
}
