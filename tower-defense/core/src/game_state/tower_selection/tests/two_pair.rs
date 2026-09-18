use super::{TWO_PAIR, evaluate};

#[test]
fn test_two_pair() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 8), (2, 8), (0, 5)], &[]);
    assert_eq!(template.kind, TWO_PAIR);
    assert_eq!(template.rank, Some(12));
}
