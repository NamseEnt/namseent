use serde_json::Value;

struct PatchDiffer {
    path: String,
    patch: super::RevertablePatch,
    shift: usize,
}

impl PatchDiffer {
    fn new() -> Self {
        Self {
            path: "/".to_string(),
            patch: super::RevertablePatch(vec![]),
            shift: 0,
        }
    }
}

impl<'a> treediff::Delegate<'a, treediff::value::Key, Value> for PatchDiffer {
    fn push(&mut self, key: &treediff::value::Key) {
        use std::fmt::Write;
        if self.path.len() != 1 {
            self.path.push('/');
        }
        match *key {
            treediff::value::Key::Index(idx) => write!(self.path, "{}", idx - self.shift).unwrap(),
            treediff::value::Key::String(ref key) => append_path(&mut self.path, key),
        }
    }

    fn pop(&mut self) {
        let mut pos = self.path.rfind('/').unwrap_or(0);
        if pos == 0 {
            pos = 1;
        }
        self.path.truncate(pos);
        self.shift = 0;
    }

    fn removed<'b>(&mut self, k: &'b treediff::value::Key, v: &'a Value) {
        let len = self.path.len();
        self.push(k);
        self.patch.0.push(super::RevertablePatchOperation::Remove {
            operation: super::RemoveOperation {
                path: self.path.clone(),
            },
            previous_value: v.clone(),
        });
        // Shift indices, we are deleting array elements
        if let treediff::value::Key::Index(_) = k {
            self.shift += 1;
        }
        self.path.truncate(len);
    }

    fn added(&mut self, k: &treediff::value::Key, v: &Value) {
        let len = self.path.len();
        self.push(k);
        self.patch
            .0
            .push(super::RevertablePatchOperation::Add(super::AddOperation {
                path: self.path.clone(),
                value: v.clone(),
            }));
        self.path.truncate(len);
    }

    fn modified(&mut self, old: &'a Value, new: &'a Value) {
        self.patch.0.push(super::RevertablePatchOperation::Replace {
            operation: super::ReplaceOperation {
                path: self.path.clone(),
                value: new.clone(),
            },
            previous_value: old.clone(),
        });
    }
}

fn append_path(path: &mut String, key: &str) {
    path.reserve(key.len());
    for ch in key.chars() {
        if ch == '~' {
            *path += "~0";
        } else if ch == '/' {
            *path += "~1";
        } else {
            path.push(ch);
        }
    }
}

pub fn diff(left: &Value, right: &Value) -> super::Patch {
    diff_revertable(left, right).to_patch()
}

pub fn diff_revertable(left: &Value, right: &Value) -> super::RevertablePatch {
    let mut differ = PatchDiffer::new();
    treediff::diff(left, right, &mut differ);
    differ.patch
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    #[test]
    pub fn replace_all() {
        let left = json!({"title": "Hello!"});
        let p = super::diff(&left, &Value::Null);
        assert_eq!(
            p,
            serde_json::from_value(json!([
                { "op": "replace", "path": "/", "value": null },
            ]))
            .unwrap()
        );
    }

    #[test]
    pub fn add_all() {
        let right = json!({"title": "Hello!"});
        let p = super::diff(&Value::Null, &right);
        assert_eq!(
            p,
            serde_json::from_value(json!([
                { "op": "replace", "path": "/", "value": { "title": "Hello!" } },
            ]))
            .unwrap()
        );
    }

    #[test]
    pub fn remove_all() {
        let left = json!(["hello", "bye"]);
        let right = json!([]);
        let p = super::diff(&left, &right);
        assert_eq!(
            p,
            serde_json::from_value(json!([
                { "op": "remove", "path": "/0" },
                { "op": "remove", "path": "/0" },
            ]))
            .unwrap()
        );
    }

    #[test]
    pub fn remove_tail() {
        let left = json!(["hello", "bye", "hi"]);
        let right = json!(["hello"]);
        let p = super::diff(&left, &right);
        assert_eq!(
            p,
            serde_json::from_value(json!([
                { "op": "remove", "path": "/1" },
                { "op": "remove", "path": "/1" },
            ]))
            .unwrap()
        );
    }
    #[test]
    pub fn replace_object() {
        let left = json!(["hello", "bye"]);
        let right = json!({"hello": "bye"});
        let p = super::diff(&left, &right);
        assert_eq!(
            p,
            serde_json::from_value(json!([
                { "op": "add", "path": "/hello", "value": "bye" },
                { "op": "remove", "path": "/0" },
                { "op": "remove", "path": "/0" },
            ]))
            .unwrap()
        );
    }

    #[test]
    fn escape_json_keys() {
        let mut left = json!({
            "/slashed/path": 1
        });
        let right = json!({
            "/slashed/path": 2,
        });
        let patch = super::diff(&left, &right);

        eprintln!("{patch:?}");

        crate::patch(&mut left, &patch).unwrap();
        assert_eq!(left, right);
    }
}
