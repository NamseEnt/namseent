use super::{FOUR_OF_A_KIND, evaluate};

#[test]
fn test_four_of_a_kind() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 12), (2, 12), (0, 5)], &[]);
    assert_eq!(template.kind, FOUR_OF_A_KIND);
    assert_eq!(template.rank, Some(12));
}

#[test]
fn test_five_of_a_kind_is_treated_as_four_of_a_kind() {
    let template = evaluate(&[(0, 12), (1, 12), (3, 12), (2, 12), (0, 12)], &[]);
    assert_eq!(template.kind, FOUR_OF_A_KIND);
    assert_eq!(template.rank, Some(12));
}
