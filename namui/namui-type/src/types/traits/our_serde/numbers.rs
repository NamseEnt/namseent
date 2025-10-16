use super::*;

impl Serialize for bool {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u8(*self as u8);
    }
}

impl Deserialize for bool {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u8() != 0)
    }
}

impl Serialize for i8 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_i8(*self);
    }
}

impl Deserialize for i8 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_i8())
    }
}

impl Serialize for i16 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_i16(*self);
    }
}

impl Deserialize for i16 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_i16())
    }
}

impl Serialize for i32 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_i32(*self);
    }
}

impl Deserialize for i32 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_i32())
    }
}

impl Serialize for i64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_i64(*self);
    }
}

impl Deserialize for i64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_i64())
    }
}

impl Serialize for i128 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_i128(*self);
    }
}

impl Deserialize for i128 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_i128())
    }
}

impl Serialize for isize {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_i64(*self as i64);
    }
}

impl Deserialize for isize {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_i64() as isize)
    }
}

impl Serialize for u8 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u8(*self);
    }
}

impl Deserialize for u8 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u8())
    }
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u16(*self);
    }
}

impl Deserialize for u16 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u16())
    }
}

impl Serialize for u32 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u32(*self);
    }
}

impl Deserialize for u32 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u32())
    }
}

impl Serialize for u64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(*self);
    }
}

impl Deserialize for u64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u64())
    }
}

impl Serialize for u128 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u128(*self);
    }
}

impl Deserialize for u128 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u128())
    }
}

impl Serialize for usize {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(*self as u64);
    }
}

impl Deserialize for usize {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_u64() as usize)
    }
}

impl Serialize for f32 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_f32(*self);
    }
}

impl Deserialize for f32 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_f32())
    }
}

impl Serialize for f64 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_f64(*self);
    }
}

impl Deserialize for f64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(buf.get_f64())
    }
}

impl Serialize for std::sync::atomic::AtomicBool {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u8(self.load(std::sync::atomic::Ordering::Acquire) as u8);
    }
}

impl Deserialize for std::sync::atomic::AtomicBool {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(std::sync::atomic::AtomicBool::new(buf.get_u8() != 0))
    }
}

impl Serialize for std::num::NonZero<usize> {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.write_string(std::any::type_name::<Self>());
        self.serialize_without_name(buf);
    }
    fn serialize_without_name(&self, buf: &mut Vec<u8>) {
        buf.put_u64(self.get() as u64);
    }
}

impl Deserialize for std::num::NonZero<usize> {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
        Ok(std::num::NonZero::new(buf.get_u64() as usize).unwrap())
    }
}
