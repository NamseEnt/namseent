use super::{HIGH, evaluate};

#[test]
fn test_high_card() {
    let template = evaluate(&[(0, 12), (1, 8), (3, 5), (2, 7), (0, 6)], &[]);
    assert_eq!(template.kind, HIGH);
    assert_eq!(template.rank, Some(12));
}
