use enum_iter::EnumIter;

#[derive(EnumIter, PartialEq, Eq, Debug)]
enum Test {
    Zero,
    One,
}

#[test]
fn enum_iter() {
    let mut iter = Test::iter();
    assert_eq!(iter.next(), Some(Test::Zero));
    assert_eq!(iter.next(), Some(Test::One));
    assert_eq!(iter.next(), None);
}
