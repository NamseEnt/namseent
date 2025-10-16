use super::*;

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.len() as u64);
        for item in self {
            item.serialize_without_name(buf);
        }
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::deserialize_without_name(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize, const N: usize> Serialize for [T; N] {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(N as u64);
        for item in self {
            item.serialize_without_name(buf);
        }
    }
}

impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        if len != N {
            return Err(DeserializeError::InvalidName {
                expected: format!("array of length {}", N),
                actual: format!("array of length {}", len),
            });
        }
        let mut result: [std::mem::MaybeUninit<T>; N] =
            unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for result_item in result.iter_mut().take(N) {
            *result_item = std::mem::MaybeUninit::new(T::deserialize_without_name(buf)?);
        }
        Ok(unsafe { std::mem::transmute_copy::<[std::mem::MaybeUninit<T>; N], [T; N]>(&result) })
    }
}

impl<Key, Value> Serialize for std::collections::BTreeMap<Key, Value>
where
    Key: Serialize,
    Value: Serialize,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.len() as u64);
        for (key, value) in self {
            key.serialize_without_name(buf);
            value.serialize_without_name(buf);
        }
    }
}

impl<Key, Value> Deserialize for std::collections::BTreeMap<Key, Value>
where
    Key: Deserialize + Ord,
    Value: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        let mut result = std::collections::BTreeMap::new();
        for _ in 0..len {
            result.insert(
                Key::deserialize_without_name(buf)?,
                Value::deserialize_without_name(buf)?,
            );
        }
        Ok(result)
    }
}

impl<Key, Value> Serialize for std::collections::HashMap<Key, Value>
where
    Key: Serialize,
    Value: Serialize,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.len() as u64);
        for (key, value) in self {
            key.serialize_without_name(buf);
            value.serialize_without_name(buf);
        }
    }
}

impl<Key, Value> Deserialize for std::collections::HashMap<Key, Value>
where
    Key: Deserialize + std::cmp::Eq + std::hash::Hash,
    Value: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        let mut result = std::collections::HashMap::new();
        for _ in 0..len {
            result.insert(
                Key::deserialize_without_name(buf)?,
                Value::deserialize_without_name(buf)?,
            );
        }
        Ok(result)
    }
}

impl<T: Serialize> Serialize for std::collections::HashSet<T> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.len() as u64);
        for item in self {
            item.serialize_without_name(buf);
        }
    }
}

impl<T> Deserialize for std::collections::HashSet<T>
where
    T: Deserialize + std::cmp::Eq + std::hash::Hash,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        let mut result = std::collections::HashSet::new();
        for _ in 0..len {
            result.insert(T::deserialize_without_name(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize> Serialize for std::collections::BTreeSet<T> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.len() as u64);
        for item in self {
            item.serialize_without_name(buf);
        }
    }
}

impl<T> Deserialize for std::collections::BTreeSet<T>
where
    T: Deserialize + Ord,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        let mut result = std::collections::BTreeSet::new();
        for _ in 0..len {
            result.insert(T::deserialize_without_name(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize> Serialize for std::collections::VecDeque<T> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.len() as u64);
        for item in self {
            item.serialize_without_name(buf);
        }
    }
}

impl<T: Deserialize> Deserialize for std::collections::VecDeque<T> {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        let len = buf.get_u64() as usize;
        let mut result = std::collections::VecDeque::with_capacity(len);
        for _ in 0..len {
            result.push_back(T::deserialize_without_name(buf)?);
        }
        Ok(result)
    }
}

impl Serialize for () {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, _buf: &mut Vec<u8>) {}
}

impl Deserialize for () {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(_buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(())
    }
}

macro_rules! impl_tuple_serialize {
    ($($T:ident),* ; $($idx:tt),*) => {
        impl<$($T: Serialize),*> Serialize for ($($T,)*) {
            fn serialize(&self, buf: &mut Vec<u8>) {
                buf.write_string(std::any::type_name::<Self>());
                self.serialize_without_name(buf);
            }
            fn serialize_without_name(&self, buf: &mut Vec<u8>) {
                $(self.$idx.serialize(buf);)*
            }
        }
    };
}

macro_rules! impl_tuple_deserialize {
    ($($T:ident),* ; $($idx:tt),*) => {
        impl<$($T: Deserialize),*> Deserialize for ($($T,)*) {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                buf.read_name(std::any::type_name::<Self>())?;
                Self::deserialize_without_name(buf)
            }
            fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                Ok(($($T::deserialize(buf)?,)*))
            }
        }
    };
}

impl_tuple_serialize!(T0; 0);
impl_tuple_deserialize!(T0; 0);

impl_tuple_serialize!(T0, T1; 0, 1);
impl_tuple_deserialize!(T0, T1; 0, 1);

impl_tuple_serialize!(T0, T1, T2; 0, 1, 2);
impl_tuple_deserialize!(T0, T1, T2; 0, 1, 2);

impl_tuple_serialize!(T0, T1, T2, T3; 0, 1, 2, 3);
impl_tuple_deserialize!(T0, T1, T2, T3; 0, 1, 2, 3);

impl_tuple_serialize!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
impl_tuple_deserialize!(T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);

impl_tuple_serialize!(T0, T1, T2, T3, T4, T5; 0, 1, 2, 3, 4, 5);
impl_tuple_deserialize!(T0, T1, T2, T3, T4, T5; 0, 1, 2, 3, 4, 5);

impl_tuple_serialize!(T0, T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5, 6);
impl_tuple_deserialize!(T0, T1, T2, T3, T4, T5, T6; 0, 1, 2, 3, 4, 5, 6);

impl_tuple_serialize!(T0, T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple_deserialize!(T0, T1, T2, T3, T4, T5, T6, T7; 0, 1, 2, 3, 4, 5, 6, 7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_serde() {
        let original: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = Vec::<i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);

        let original: Vec<String> = vec![
            String::from("hello"),
            String::from("world"),
            String::from(""),
        ];
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = Vec::<String>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);

        let original: Vec<i32> = vec![];
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = Vec::<i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_array_serde() {
        let original: [i32; 5] = [1, 2, 3, 4, 5];
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = <[i32; 5]>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);

        let original: [u64; 3] = [100, 200, 300];
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = <[u64; 3]>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_btreemap_serde() {
        let mut original = std::collections::BTreeMap::new();
        original.insert(String::from("a"), 1);
        original.insert(String::from("b"), 2);
        original.insert(String::from("c"), 3);

        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized =
            std::collections::BTreeMap::<String, i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_hashmap_serde() {
        let mut original = std::collections::HashMap::new();
        original.insert(String::from("key1"), 100);
        original.insert(String::from("key2"), 200);

        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized =
            std::collections::HashMap::<String, i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_hashset_serde() {
        let mut original = std::collections::HashSet::new();
        original.insert(String::from("hello"));
        original.insert(String::from("world"));

        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized =
            std::collections::HashSet::<String>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_btreeset_serde() {
        let mut original = std::collections::BTreeSet::new();
        original.insert(1);
        original.insert(2);
        original.insert(3);

        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::collections::BTreeSet::<i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_vecdeque_serde() {
        let mut original = std::collections::VecDeque::new();
        original.push_back(10);
        original.push_back(20);
        original.push_back(30);

        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::collections::VecDeque::<i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_unit_serde() {
        let original = ();
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        <()>::deserialize(&mut buf_slice).unwrap();
    }

    #[test]
    fn test_tuple_1_serde() {
        let original: (String,) = (String::from("hello"),);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = <(String,)>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_tuple_2_serde() {
        let original: (i32, String) = (42, String::from("test"));
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = <(i32, String)>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_tuple_3_serde() {
        let original: (i32, String, f64) = (42, String::from("test"), std::f64::consts::PI);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = <(i32, String, f64)>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_tuple_4_serde() {
        let original: (i32, String, f64, bool) =
            (42, String::from("test"), std::f64::consts::PI, true);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = <(i32, String, f64, bool)>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_nested_collections_serde() {
        let mut inner_map = std::collections::BTreeMap::new();
        inner_map.insert(1, String::from("a"));
        inner_map.insert(2, String::from("b"));

        let original: Vec<std::collections::BTreeMap<i32, String>> = vec![inner_map];
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized =
            Vec::<std::collections::BTreeMap<i32, String>>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original, deserialized);
    }
}
