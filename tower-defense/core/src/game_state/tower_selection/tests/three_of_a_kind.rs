use super::{THREE_OF_A_KIND, evaluate};

#[test]
fn test_three_of_a_kind() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 12), (2, 8), (0, 5)], &[]);
    assert_eq!(template.kind, THREE_OF_A_KIND);
    assert_eq!(template.rank, Some(12));
}
