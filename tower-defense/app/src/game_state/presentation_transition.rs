use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct PresentationTransition<Id> {
    pub added: Vec<Id>,
    pub removed: Vec<Id>,
    pub retained: Vec<Id>,
    pub updated: Vec<Id>,
    pub ordered: Vec<Id>,
}

pub(crate) fn diff_projection<Id, Value>(
    before: &[(Id, Value)],
    after: &[(Id, Value)],
) -> PresentationTransition<Id>
where
    Id: Clone + Eq + Hash,
    Value: PartialEq,
{
    assert_unique_ids(before);
    assert_unique_ids(after);
    let before_by_id = before
        .iter()
        .map(|(id, value)| (id, value))
        .collect::<HashMap<_, _>>();
    let after_by_id = after
        .iter()
        .map(|(id, value)| (id, value))
        .collect::<HashMap<_, _>>();

    let added = after
        .iter()
        .filter(|(id, _)| !before_by_id.contains_key(id))
        .map(|(id, _)| id.clone())
        .collect();
    let removed = before
        .iter()
        .filter(|(id, _)| !after_by_id.contains_key(id))
        .map(|(id, _)| id.clone())
        .collect();
    let retained = after
        .iter()
        .filter(|(id, _)| before_by_id.contains_key(id))
        .map(|(id, _)| id.clone())
        .collect();
    let updated = after
        .iter()
        .filter(|(id, value)| {
            before_by_id
                .get(id)
                .is_some_and(|previous| *previous != value)
        })
        .map(|(id, _)| id.clone())
        .collect();

    PresentationTransition {
        added,
        removed,
        retained,
        updated,
        ordered: after.iter().map(|(id, _)| id.clone()).collect(),
    }
}

fn assert_unique_ids<Id, Value>(projection: &[(Id, Value)])
where
    Id: Eq + Hash,
{
    let mut ids = HashSet::with_capacity(projection.len());
    assert!(
        projection.iter().all(|(id, _)| ids.insert(id)),
        "duplicate presentation identity"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_diff_preserves_order_and_classifies_payload_changes() {
        let transition = diff_projection(
            &[(1_u8, "old"), (2, "same"), (3, "removed")],
            &[(2, "same"), (4, "added"), (1, "new")],
        );

        assert_eq!(transition.added, vec![4]);
        assert_eq!(transition.removed, vec![3]);
        assert_eq!(transition.retained, vec![2, 1]);
        assert_eq!(transition.updated, vec![1]);
        assert_eq!(transition.ordered, vec![2, 4, 1]);
    }

    #[test]
    #[should_panic(expected = "duplicate presentation identity")]
    fn duplicate_projection_ids_are_rejected() {
        let _ = diff_projection(&[(1_u8, 1)], &[(1, 2), (1, 3)]);
    }
}
