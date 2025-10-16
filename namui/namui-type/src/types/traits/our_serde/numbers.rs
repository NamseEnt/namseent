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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_serde_roundtrip {
        ($type_name:ident, $value:expr) => {{
            let original = $value;
            let mut buf = Vec::new();
            original.serialize(&mut buf);
            let mut buf_slice = buf.as_slice();
            let deserialized = $type_name::deserialize(&mut buf_slice).unwrap();
            assert_eq!(original, deserialized);
        }};
    }

    #[test]
    fn test_bool_serde() {
        test_serde_roundtrip!(bool, true);
        test_serde_roundtrip!(bool, false);
    }

    #[test]
    fn test_i8_serde() {
        test_serde_roundtrip!(i8, 0i8);
        test_serde_roundtrip!(i8, 127i8);
        test_serde_roundtrip!(i8, -128i8);
    }

    #[test]
    fn test_i16_serde() {
        test_serde_roundtrip!(i16, 0i16);
        test_serde_roundtrip!(i16, 32767i16);
        test_serde_roundtrip!(i16, -32768i16);
    }

    #[test]
    fn test_i32_serde() {
        test_serde_roundtrip!(i32, 0i32);
        test_serde_roundtrip!(i32, 2147483647i32);
        test_serde_roundtrip!(i32, -2147483648i32);
    }

    #[test]
    fn test_i64_serde() {
        test_serde_roundtrip!(i64, 0i64);
        test_serde_roundtrip!(i64, 9223372036854775807i64);
        test_serde_roundtrip!(i64, -9223372036854775808i64);
    }

    #[test]
    fn test_i128_serde() {
        test_serde_roundtrip!(i128, 0i128);
        test_serde_roundtrip!(i128, 170141183460469231731687303715884105727i128);
        test_serde_roundtrip!(i128, -170141183460469231731687303715884105728i128);
    }

    #[test]
    fn test_isize_serde() {
        test_serde_roundtrip!(isize, 0isize);
        test_serde_roundtrip!(isize, 1000isize);
        test_serde_roundtrip!(isize, -1000isize);
    }

    #[test]
    fn test_u8_serde() {
        test_serde_roundtrip!(u8, 0u8);
        test_serde_roundtrip!(u8, 255u8);
    }

    #[test]
    fn test_u16_serde() {
        test_serde_roundtrip!(u16, 0u16);
        test_serde_roundtrip!(u16, 65535u16);
    }

    #[test]
    fn test_u32_serde() {
        test_serde_roundtrip!(u32, 0u32);
        test_serde_roundtrip!(u32, 4294967295u32);
    }

    #[test]
    fn test_u64_serde() {
        test_serde_roundtrip!(u64, 0u64);
        test_serde_roundtrip!(u64, 18446744073709551615u64);
    }

    #[test]
    fn test_u128_serde() {
        test_serde_roundtrip!(u128, 0u128);
        test_serde_roundtrip!(u128, 340282366920938463463374607431768211455u128);
    }

    #[test]
    fn test_usize_serde() {
        test_serde_roundtrip!(usize, 0usize);
        test_serde_roundtrip!(usize, 1000usize);
    }

    #[test]
    fn test_f32_serde() {
        test_serde_roundtrip!(f32, 0.0f32);
        test_serde_roundtrip!(f32, std::f32::consts::PI);
        test_serde_roundtrip!(f32, -42.5f32);
    }

    #[test]
    fn test_f64_serde() {
        test_serde_roundtrip!(f64, 0.0f64);
        test_serde_roundtrip!(f64, std::f64::consts::PI);
        test_serde_roundtrip!(f64, -42.5f64);
    }

    #[test]
    fn test_atomic_bool_serde() {
        let original = std::sync::atomic::AtomicBool::new(true);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::sync::atomic::AtomicBool::deserialize(&mut buf_slice).unwrap();
        assert_eq!(
            original.load(std::sync::atomic::Ordering::Acquire),
            deserialized.load(std::sync::atomic::Ordering::Acquire)
        );

        let original = std::sync::atomic::AtomicBool::new(false);
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::sync::atomic::AtomicBool::deserialize(&mut buf_slice).unwrap();
        assert_eq!(
            original.load(std::sync::atomic::Ordering::Acquire),
            deserialized.load(std::sync::atomic::Ordering::Acquire)
        );
    }

    #[test]
    fn test_nonzero_usize_serde() {
        let original = std::num::NonZero::new(42usize).unwrap();
        let mut buf = Vec::new();
        original.serialize(&mut buf);
        let mut buf_slice = buf.as_slice();
        let deserialized = std::num::NonZero::<usize>::deserialize(&mut buf_slice).unwrap();
        assert_eq!(original.get(), deserialized.get());
    }
}
