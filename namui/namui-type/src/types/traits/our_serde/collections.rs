use super::*;

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        for item in self {
            let item_bytes = item.serialize();
            buffer.put_slice(&item_bytes);
        }
        buffer
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::deserialize(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize, const N: usize> Serialize for [T; N] {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(N as u64);
        for item in self {
            let item_bytes = item.serialize();
            buffer.put_slice(&item_bytes);
        }
        buffer
    }
}

impl<T: Deserialize, const N: usize> Deserialize for [T; N] {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
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
            *result_item = std::mem::MaybeUninit::new(T::deserialize(buf)?);
        }
        Ok(unsafe { std::mem::transmute_copy::<[std::mem::MaybeUninit<T>; N], [T; N]>(&result) })
    }
}

impl<Key, Value> Serialize for std::collections::BTreeMap<Key, Value>
where
    Key: Serialize,
    Value: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        for (key, value) in self {
            let key_bytes = key.serialize();
            let value_bytes = value.serialize();
            buffer.put_slice(&key_bytes);
            buffer.put_slice(&value_bytes);
        }
        buffer
    }
}

impl<Key, Value> Deserialize for std::collections::BTreeMap<Key, Value>
where
    Key: Deserialize + Ord,
    Value: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let mut result = std::collections::BTreeMap::new();
        for _ in 0..len {
            result.insert(Key::deserialize(buf)?, Value::deserialize(buf)?);
        }
        Ok(result)
    }
}

impl<Key, Value> Serialize for std::collections::HashMap<Key, Value>
where
    Key: Serialize,
    Value: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        for (key, value) in self {
            let key_bytes = key.serialize();
            let value_bytes = value.serialize();
            buffer.put_slice(&key_bytes);
            buffer.put_slice(&value_bytes);
        }
        buffer
    }
}

impl<Key, Value> Deserialize for std::collections::HashMap<Key, Value>
where
    Key: Deserialize + std::cmp::Eq + std::hash::Hash,
    Value: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let mut result = std::collections::HashMap::new();
        for _ in 0..len {
            result.insert(Key::deserialize(buf)?, Value::deserialize(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize> Serialize for std::collections::HashSet<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        for item in self {
            let item_bytes = item.serialize();
            buffer.put_slice(&item_bytes);
        }
        buffer
    }
}

impl<T> Deserialize for std::collections::HashSet<T>
where
    T: Deserialize + std::cmp::Eq + std::hash::Hash,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let mut result = std::collections::HashSet::new();
        for _ in 0..len {
            result.insert(T::deserialize(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize> Serialize for std::collections::BTreeSet<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        for item in self {
            let item_bytes = item.serialize();
            buffer.put_slice(&item_bytes);
        }
        buffer
    }
}

impl<T> Deserialize for std::collections::BTreeSet<T>
where
    T: Deserialize + Ord,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let mut result = std::collections::BTreeSet::new();
        for _ in 0..len {
            result.insert(T::deserialize(buf)?);
        }
        Ok(result)
    }
}

impl<T: Serialize> Serialize for std::collections::VecDeque<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        for item in self {
            let item_bytes = item.serialize();
            buffer.put_slice(&item_bytes);
        }
        buffer
    }
}

impl<T: Deserialize> Deserialize for std::collections::VecDeque<T> {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let mut result = std::collections::VecDeque::with_capacity(len);
        for _ in 0..len {
            result.push_back(T::deserialize(buf)?);
        }
        Ok(result)
    }
}

impl Serialize for () {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer
    }
}

impl Deserialize for () {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(())
    }
}

macro_rules! impl_tuple_serialize {
    ($($T:ident),* ; $($idx:tt),*) => {
        impl<$($T: Serialize),*> Serialize for ($($T,)*) {
            fn serialize(&self) -> Vec<u8> {
                let mut buffer = vec![];
                buffer.write_string(std::any::type_name::<Self>());
                $(buffer.put_slice(&self.$idx.serialize());)*
                buffer
            }
        }
    };
}

macro_rules! impl_tuple_deserialize {
    ($($T:ident),* ; $($idx:tt),*) => {
        impl<$($T: Deserialize),*> Deserialize for ($($T,)*) {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                buf.read_name(std::any::type_name::<Self>())?;
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
