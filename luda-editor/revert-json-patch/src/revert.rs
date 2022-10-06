use super::*;

#[derive(Debug, Clone)]
pub struct RevertablePatch(pub Vec<RevertablePatchOperation>);

impl RevertablePatch {
    pub fn to_patch(&self) -> Patch {
        Patch(
            self.0
                .iter()
                .map(|patch| match patch {
                    RevertablePatchOperation::Add(operation) => {
                        PatchOperation::Add(operation.clone())
                    }
                    RevertablePatchOperation::Remove { operation, .. } => {
                        PatchOperation::Remove(operation.clone())
                    }
                    RevertablePatchOperation::Replace { operation, .. } => {
                        PatchOperation::Replace(operation.clone())
                    }
                })
                .collect(),
        )
    }
    pub fn to_reversed_patch(&self) -> Patch {
        self.reverse().to_patch()
    }
    pub fn reverse(&self) -> RevertablePatch {
        RevertablePatch(
            self.0
                .iter()
                .rev()
                .map(|patch| match patch {
                    RevertablePatchOperation::Add(operation) => RevertablePatchOperation::Remove {
                        operation: RemoveOperation {
                            path: operation.path.clone(),
                        },
                        previous_value: operation.value.clone(),
                    },
                    RevertablePatchOperation::Remove {
                        operation,
                        previous_value,
                    } => RevertablePatchOperation::Add(AddOperation {
                        path: operation.path.clone(),
                        value: previous_value.clone(),
                    }),
                    RevertablePatchOperation::Replace {
                        operation,
                        previous_value,
                    } => RevertablePatchOperation::Replace {
                        operation: ReplaceOperation {
                            path: operation.path.clone(),
                            value: previous_value.clone(),
                        },
                        previous_value: operation.value.clone(),
                    },
                })
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum RevertablePatchOperation {
    Add(AddOperation),
    Remove {
        operation: RemoveOperation,
        previous_value: Value,
    },
    Replace {
        operation: ReplaceOperation,
        previous_value: Value,
    },
}

pub fn revert(doc: &mut Value, patch: &RevertablePatch) -> Result<(), PatchError> {
    let patch = patch.to_reversed_patch();
    super::patch(doc, &patch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn replace_all() {
        let mut left = json!({"title": "Hello!"});
        let right = json!({});
        let p = super::diff_revertable(&left, &right);

        patch(&mut left, &p.to_patch()).unwrap();
        assert_eq!(left, right);

        revert(&mut left, &p).unwrap();
        assert_eq!(left, json!({"title": "Hello!"}));
    }

    #[test]
    pub fn add_all() {
        let mut left = json!({});
        let right = json!({"title": "Hello!"});
        let p = super::diff_revertable(&left, &right);

        patch(&mut left, &p.to_patch()).unwrap();
        assert_eq!(left, right);

        revert(&mut left, &p).unwrap();
        assert_eq!(left, json!({}));
    }

    #[test]
    pub fn remove_all() {
        let mut left = json!(["hello", "bye"]);
        let right = json!([]);
        let p = super::diff_revertable(&left, &right);

        patch(&mut left, &p.to_patch()).unwrap();
        assert_eq!(left, right);

        revert(&mut left, &p).unwrap();
        assert_eq!(left, json!(["hello", "bye"]));
    }

    #[test]
    pub fn remove_tail() {
        let mut left = json!(["hello", "bye", "hi"]);
        let right = json!(["hello"]);
        let p = super::diff_revertable(&left, &right);

        patch(&mut left, &p.to_patch()).unwrap();
        assert_eq!(left, right);

        revert(&mut left, &p).unwrap();
        assert_eq!(left, json!(["hello", "bye", "hi"]));
    }

    #[test]
    fn escape_json_keys() {
        let mut left = json!({
            "/slashed/path": 1
        });
        let right = json!({
            "/slashed/path": 2,
        });
        let p = super::diff_revertable(&left, &right);

        patch(&mut left, &p.to_patch()).unwrap();
        assert_eq!(left, right);

        revert(&mut left, &p).unwrap();
        assert_eq!(
            left,
            json!({
                "/slashed/path": 1
            })
        );
    }
}
