use super::{ONE_PAIR, evaluate};

#[test]
fn test_one_pair() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 5), (2, 7), (0, 6)], &[]);
    assert_eq!(template.kind, ONE_PAIR);
    assert_eq!(template.rank, Some(12));
}
