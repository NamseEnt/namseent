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
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(std::cell::RefCell::new(T::deserialize(buf)?))
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
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(std::cell::UnsafeCell::new(T::deserialize(buf)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell_serde() {
        let original = std::cell::RefCell::new(String::from("hello"));
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::cell::RefCell::<String>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(*original.borrow(), *deserialized.borrow());

        let original = std::cell::RefCell::new(42i32);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::cell::RefCell::<i32>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(*original.borrow(), *deserialized.borrow());
    }

    #[test]
    fn test_unsafecell_serde() {
        let original = std::cell::UnsafeCell::new(String::from("test"));
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::cell::UnsafeCell::<String>::deserialize(&mut buf_slice).unwrap();
        unsafe {
            assert_eq!(*original.get(), *deserialized.get());
        }

        let original = std::cell::UnsafeCell::new(vec![1, 2, 3]);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::cell::UnsafeCell::<Vec<i32>>::deserialize(&mut buf_slice).unwrap();
        unsafe {
            assert_eq!(*original.get(), *deserialized.get());
        }
    }
}
