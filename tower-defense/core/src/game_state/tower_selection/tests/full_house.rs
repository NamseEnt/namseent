use super::{FULL_HOUSE, evaluate};

#[test]
fn test_full_house() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 12), (2, 8), (0, 8)], &[]);
    assert_eq!(template.kind, FULL_HOUSE);
    assert_eq!(template.rank, Some(12));
}
