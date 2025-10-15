use bytes::{Buf, BufMut};

pub trait State:
    Send + 'static + Serialize + Deserialize + bincode::Encode + bincode::Decode<()>
{
}

impl<T: Send + 'static + Serialize + Deserialize + bincode::Encode + bincode::Decode<()>> State
    for T
{
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize: Sized {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError>;
}

pub enum DeserializeError {
    InvalidName { expected: String, actual: String },
    InvalidEnumVariant { expected: String, actual: String },
}

pub trait BufMutExt {
    fn write_string(&mut self, name: &str);
}

impl BufMutExt for Vec<u8> {
    fn write_string(&mut self, name: &str) {
        self.put_u16(name.len() as u16);
        self.put_slice(name.as_bytes());
    }
}

pub trait BufExt {
    fn read_name(&mut self, expected: &'static str) -> Result<String, DeserializeError>;
    fn read_string(&mut self) -> String;
}

impl<T> BufExt for T
where
    T: Buf + ?Sized,
{
    fn read_name(&mut self, expected: &'static str) -> Result<String, DeserializeError> {
        let name = self.read_string();
        if name != expected {
            return Err(DeserializeError::InvalidName {
                expected: expected.to_string(),
                actual: name.to_string(),
            });
        }
        Ok(name)
    }

    fn read_string(&mut self) -> String {
        let name_len = self.get_u16();
        let name =
            std::string::String::from_utf8(self.chunk()[..name_len as usize].to_vec()).unwrap();
        self.advance(name_len as usize);
        name
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

impl Serialize for bool {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u8(*self as u8);
        buffer
    }
}

impl Deserialize for bool {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u8() != 0)
    }
}

impl Serialize for i8 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_i8(*self);
        buffer
    }
}

impl Deserialize for i8 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_i8())
    }
}

impl Serialize for i16 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_i16(*self);
        buffer
    }
}

impl Deserialize for i16 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_i16())
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_i32(*self);
        buffer
    }
}

impl Deserialize for i32 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_i32())
    }
}

impl Serialize for i64 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_i64(*self);
        buffer
    }
}

impl Deserialize for i64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_i64())
    }
}

impl Serialize for i128 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_i128(*self);
        buffer
    }
}

impl Deserialize for i128 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_i128())
    }
}

impl Serialize for isize {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_i64(*self as i64);
        buffer
    }
}

impl Deserialize for isize {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_i64() as isize)
    }
}

impl Serialize for u8 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u8(*self);
        buffer
    }
}

impl Deserialize for u8 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u8())
    }
}

impl Serialize for u16 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u16(*self);
        buffer
    }
}

impl Deserialize for u16 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u16())
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u32(*self);
        buffer
    }
}

impl Deserialize for u32 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u32())
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(*self);
        buffer
    }
}

impl Deserialize for u64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u64())
    }
}

impl Serialize for u128 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u128(*self);
        buffer
    }
}

impl Deserialize for u128 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u128())
    }
}

impl Serialize for usize {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(*self as u64);
        buffer
    }
}

impl Deserialize for usize {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_u64() as usize)
    }
}

impl Serialize for f32 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_f32(*self);
        buffer
    }
}

impl Deserialize for f32 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_f32())
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_f64(*self);
        buffer
    }
}

impl Deserialize for f64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(buf.get_f64())
    }
}

impl Serialize for char {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u32(*self as u32);
        buffer
    }
}

impl Deserialize for char {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let value = buf.get_u32();
        Ok(char::from_u32(value).unwrap())
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.len() as u64);
        buffer.put_slice(self.as_bytes());
        buffer
    }
}

impl Deserialize for String {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let len = buf.get_u64() as usize;
        let string_bytes = &buf[..len];
        Ok(String::from_utf8(string_bytes.to_vec()).unwrap())
    }
}

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

impl Serialize for std::time::Duration {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.as_nanos() as u64);
        buffer
    }
}

impl Deserialize for std::time::Duration {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::time::Duration::from_nanos(buf.get_u64()))
    }
}

impl Serialize for std::time::SystemTime {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(self.elapsed().unwrap().as_nanos() as u64);
        buffer
    }
}

impl Deserialize for std::time::SystemTime {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_nanos(buf.get_u64()))
    }
}

impl<T> Serialize for Option<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u8(self.is_some() as u8);
        if let Some(value) = self {
            let value_bytes = value.serialize();
            buffer.put_slice(&value_bytes);
        }
        buffer
    }
}

impl<T> Deserialize for Option<T>
where
    T: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        let is_some = buf.get_u8() != 0;
        if is_some {
            Ok(Some(T::deserialize(buf)?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Serialize for Box<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u64(std::ptr::addr_of!(*self) as u64);
        buffer
    }
}

impl<T> Deserialize for Box<T>
where
    T: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(Box::new(T::deserialize(buf)?))
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

impl<T> Serialize for std::marker::PhantomData<T> {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer
    }
}

impl<T> Deserialize for std::marker::PhantomData<T> {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::marker::PhantomData)
    }
}

impl<T> Serialize for std::cell::RefCell<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        let value_bytes = self.borrow().serialize();
        buffer.put_slice(&value_bytes);
        buffer
    }
}

impl<T> Deserialize for std::cell::RefCell<T>
where
    T: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::cell::RefCell::new(T::deserialize(buf)?))
    }
}

impl<T> Serialize for std::sync::Arc<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        let value_bytes = self.as_ref().serialize();
        buffer.put_slice(&value_bytes);
        buffer
    }
}

impl<T> Deserialize for std::sync::Arc<T>
where
    T: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::sync::Arc::new(T::deserialize(buf)?))
    }
}

impl Serialize for std::sync::atomic::AtomicBool {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.put_u8(self.load(std::sync::atomic::Ordering::Acquire) as u8);
        buffer
    }
}

impl Deserialize for std::sync::atomic::AtomicBool {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::sync::atomic::AtomicBool::new(buf.get_u8() != 0))
    }
}

impl Serialize for std::path::PathBuf {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.write_string(std::any::type_name::<Self>());
        buffer.write_string(&self.as_os_str().to_string_lossy());
        buffer
    }
}

impl Deserialize for std::path::PathBuf {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;
        Ok(std::path::PathBuf::from(buf.read_string()))
    }
}
