mod collections;
mod numbers;

use bytes::{Buf, BufMut};

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
