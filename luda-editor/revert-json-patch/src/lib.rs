#[cfg_attr(test, macro_use)]
extern crate serde_json;

mod diff;
mod revert;
#[cfg(test)]
mod tests;

pub use diff::*;
pub use revert::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::{fmt, mem};

/// Representation of JSON Patch (list of patch operations)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Patch(pub Vec<PatchOperation>);

/// JSON Patch 'add' operation representation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddOperation {
    /// JSON-Pointer value [RFC6901](https://tools.ietf.org/html/rfc6901) that references a location
    /// within the target document where the operation is performed.
    pub path: String,
    /// Value to add to the target location.
    pub value: Value,
}

/// JSON Patch 'remove' operation representation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RemoveOperation {
    /// JSON-Pointer value [RFC6901](https://tools.ietf.org/html/rfc6901) that references a location
    /// within the target document where the operation is performed.
    pub path: String,
}

/// JSON Patch 'replace' operation representation
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReplaceOperation {
    /// JSON-Pointer value [RFC6901](https://tools.ietf.org/html/rfc6901) that references a location
    /// within the target document where the operation is performed.
    pub path: String,
    /// Value to replace with.
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "op")]
#[serde(rename_all = "lowercase")]
pub enum PatchOperation {
    Add(AddOperation),
    Remove(RemoveOperation),
    Replace(ReplaceOperation),
}

/// This type represents all possible errors that can occur when applying JSON patch
#[derive(Debug)]
pub enum PatchError {
    /// One of the pointers in the patch is invalid
    InvalidPointer,

    /// 'test' operation failed
    TestFailed,
}

impl Error for PatchError {}

impl fmt::Display for PatchError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match *self {
                PatchError::InvalidPointer => "invalid pointer",
                PatchError::TestFailed => "test failed",
            }
        )
    }
}

fn parse_index(str: &str, len: usize) -> Result<usize, PatchError> {
    // RFC 6901 prohibits leading zeroes in index
    if str.starts_with('0') && str.len() != 1 {
        return Err(PatchError::InvalidPointer);
    }
    match str.parse::<usize>() {
        Ok(idx) if idx < len => Ok(idx),
        _ => Err(PatchError::InvalidPointer),
    }
}

fn split_pointer(pointer: &str) -> Result<(&str, String), PatchError> {
    pointer
        .rfind('/')
        .ok_or(PatchError::InvalidPointer)
        .map(|idx| {
            (
                &pointer[0..idx],
                pointer[idx + 1..].replace("~1", "/").replace("~0", "~"),
            )
        })
}

fn add(doc: &mut Value, path: &str, value: Value) -> Result<Option<Value>, PatchError> {
    if path.is_empty() {
        return Ok(Some(mem::replace(doc, value)));
    }

    let (parent, last) = split_pointer(path)?;
    let parent = doc.pointer_mut(parent).ok_or(PatchError::InvalidPointer)?;

    match *parent {
        Value::Object(ref mut obj) => Ok(obj.insert(last, value)),
        Value::Array(ref mut arr) if last == "-" => {
            arr.push(value);
            Ok(None)
        }
        Value::Array(ref mut arr) => {
            let idx = parse_index(last.as_str(), arr.len() + 1)?;
            arr.insert(idx, value);
            Ok(None)
        }
        _ => Err(PatchError::InvalidPointer),
    }
}

fn remove(doc: &mut Value, path: &str, allow_last: bool) -> Result<Value, PatchError> {
    let (parent, last) = split_pointer(path)?;
    let parent = doc.pointer_mut(parent).ok_or(PatchError::InvalidPointer)?;

    match *parent {
        Value::Object(ref mut obj) => match obj.remove(last.as_str()) {
            None => Err(PatchError::InvalidPointer),
            Some(val) => Ok(val),
        },
        Value::Array(ref mut arr) if allow_last && last == "-" => Ok(arr.pop().unwrap()),
        Value::Array(ref mut arr) => {
            let idx = parse_index(last.as_str(), arr.len())?;
            Ok(arr.remove(idx))
        }
        _ => Err(PatchError::InvalidPointer),
    }
}

fn replace(doc: &mut Value, path: &str, value: Value) -> Result<Value, PatchError> {
    let target = doc.pointer_mut(path).ok_or(PatchError::InvalidPointer)?;
    Ok(mem::replace(target, value))
}

pub fn from_value(value: Value) -> Result<Patch, serde_json::Error> {
    let patch = serde_json::from_value::<Vec<PatchOperation>>(value)?;
    Ok(Patch(patch))
}

pub fn patch(doc: &mut Value, patch: &Patch) -> Result<(), PatchError> {
    apply_patches(doc, &patch.0)
}

// Apply patches while tracking all the changes being made so they can be reverted back in case
// subsequent patches fail. Uses stack recursion to keep the state.
fn apply_patches(doc: &mut Value, patches: &[PatchOperation]) -> Result<(), PatchError> {
    let (patch, tail) = match patches.split_first() {
        None => return Ok(()),
        Some((patch, tail)) => (patch, tail),
    };

    match *patch {
        PatchOperation::Add(ref op) => {
            let prev = add(doc, op.path.as_str(), op.value.clone())?;
            apply_patches(doc, tail).inspect_err(move |e| {
                match prev {
                    None => remove(doc, op.path.as_str(), true).unwrap(),
                    Some(v) => add(doc, op.path.as_str(), v).unwrap().unwrap(),
                };
            })
        }
        PatchOperation::Remove(ref op) => {
            let prev = remove(doc, op.path.as_str(), false)?;
            apply_patches(doc, tail).inspect_err(move |e| {
                assert!(add(doc, op.path.as_str(), prev).unwrap().is_none());
            })
        }
        PatchOperation::Replace(ref op) => {
            let prev = replace(doc, op.path.as_str(), op.value.clone())?;
            apply_patches(doc, tail).inspect_err(move |e| {
                replace(doc, op.path.as_str(), prev).unwrap();
            })
        }
    }
}
