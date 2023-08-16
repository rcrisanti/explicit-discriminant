#![allow(dead_code, unused_variables)]

use explicit_discriminant::ExplicitDiscriminant;

#[derive(ExplicitDiscriminant)]
enum TestNoPattern {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn no_pattern() {
    let t = TestNoPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(1..3)]
enum TestOneRangePattern {
    One = 1,
    Two = 2,
}

#[test]
fn one_range_pattern() {
    let t = TestOneRangePattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(1..=2)]
enum TestOneRangeInclusivePattern {
    One = 1,
    Two = 2,
}

#[test]
fn one_range_inclusive_pattern() {
    let t = TestOneRangeInclusivePattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(1..)]
enum TestOneRangeFromPattern {
    One = 1,
    Two = 2,
}

#[test]
fn one_range_from_pattern() {
    let t = TestOneRangeFromPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(..3)]
enum TestOneRangeToPattern {
    One = 1,
    Two = 2,
}

#[test]
fn one_range_to_pattern() {
    let t = TestOneRangeToPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(..=2)]
enum TestOneInclusiveRangeToPattern {
    One = 1,
    Two = 2,
}

#[test]
fn one_inclusive_range_to_pattern() {
    let t = TestOneInclusiveRangeToPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(1)]
enum TestOneLiteralPattern {
    One = 1,
}

#[test]
fn one_literal_pattern() {
    let t = TestOneLiteralPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(1 | 5)]
enum TestOneOrPattern {
    One = 1,
    Five = 5,
}

#[test]
fn one_or_pattern() {
    let t = TestOneOrPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(_)]
enum TestOneWildcardPattern {
    One = 1,
    Five = 5,
    Million = 1_000_000,
}

#[test]
fn one_wildcard_pattern() {
    let t = TestOneWildcardPattern::One;
}

#[derive(ExplicitDiscriminant)]
#[pattern(..=-1, 4, 7 | 9..=12, 15..17, 19..)]
enum TestManyPatternsOneAttribute {
    VeryNegative = -9999999,
    LittleNegative = -1,
    Four = 4,
    Seven = 7,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Fifteen = 15,
    Sixteen = 16,
    Nineteen = 19,
    VeryPositive = 123921380,
}

#[test]
fn many_patterns_one_attr() {
    let t = TestManyPatternsOneAttribute::VeryNegative;
}

#[derive(ExplicitDiscriminant)]
#[pattern(..=-1)]
#[pattern(4)]
#[pattern(7 | 9..=12)]
#[pattern(15..17)]
#[pattern(19..)]
enum TestManyPatternsManyAttributes {
    VeryNegative = -9999999,
    LittleNegative = -1,
    Four = 4,
    Seven = 7,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Fifteen = 15,
    Sixteen = 16,
    Nineteen = 19,
    VeryPositive = 123921380,
}

#[test]
fn many_patterns_many_attr() {
    let t = TestManyPatternsManyAttributes::VeryNegative;
}

#[derive(ExplicitDiscriminant)]
#[pattern(..=-1, 4)]
#[pattern((7 | 9..=12) | 19..)]
#[pattern(15..17)]
enum TestManyPatternsMixedAttributes {
    VeryNegative = -9999999,
    LittleNegative = -1,
    Four = 4,
    Seven = 7,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Fifteen = 15,
    Sixteen = 16,
    Nineteen = 19,
    VeryPositive = 123921380,
}

#[test]
fn many_patterns_mixed_attr() {
    let t = TestManyPatternsMixedAttributes::VeryNegative;
}

#[test]
fn should_not_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/should_not_compile/*.rs");
}
