use testing::{sploosh, splish};

#[test]
pub fn verify() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}