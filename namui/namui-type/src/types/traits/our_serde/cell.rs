use super::*;

impl<T> Serialize for std::cell::RefCell<T>
where
    T: Serialize,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        self.borrow().serialize(buf);
    }
}

impl<T> Deserialize for std::cell::RefCell<T>
where
    T: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {Ok(std::cell::RefCell::new(T::deserialize(buf)?))
    }
}

impl<T> Serialize for std::cell::UnsafeCell<T>
where
    T: Serialize,
{
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        unsafe { self.get().as_ref() }.unwrap().serialize(buf);
    }
}

impl<T> Deserialize for std::cell::UnsafeCell<T>
where
    T: Deserialize,
{
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {Ok(std::cell::UnsafeCell::new(T::deserialize(buf)?))
    }
}
