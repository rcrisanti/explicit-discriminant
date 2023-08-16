# explicit-discriminant

![CI](https://github.com/rcrisanti/explicit-discriminant/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/rcrisanti/explicit-discriminant/actions/workflows/cd.yml/badge.svg)

This crate enforces explicit discriminants of an enum.

```rust
use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
enum MyEnum {
    One = 1,
    Two = 2,
    Three, // will not compile
    Four = 4
}
```

You can also optionally add patterns that will enforce which discriminant values are allowed.

```rust
use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
#[pattern(2..=3 | 12..)]
#[pattern(9 | ..-3)]
enum MyEnum {
    MinusFour = -4,
    One = 1, // error here, since not in any of the patterns
    Two = 2,
    Three = 3, 
    Nine = 9,
    Ten = 10, // error here, since not in any of the patterns
    Twelve = 12,
    OneHundred = 100,
}
```
