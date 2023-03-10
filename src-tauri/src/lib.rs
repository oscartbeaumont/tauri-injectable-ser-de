// TODO: LE or BE byte structure, idk
// TODO: Replacing `as` casts with into/from impls cause they are safer
// TODO: When `debug_assertions` are enabled validate the `len_hint` is correct
// TODO: Deserialization
// TODO: Zero-copy serialization???

pub trait IntoBytes {
    fn len_hint(&self) -> u64;

    fn bytes(self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut bytes = Vec::with_capacity(self.len_hint() as usize);
        self.bytes_into_buf(&mut bytes);
        bytes
    }

    fn bytes_into_buf(self, buf: &mut Vec<u8>);

    fn decode_impl() -> String {
        format!("let offset = [0]; {}", Self::decode_impl_offset())
    }

    fn decode_impl_offset() -> String;
}

impl IntoBytes for String {
    fn len_hint(&self) -> u64 {
        (self.len() as u64) + 1
    }

    fn bytes_into_buf(self, buf: &mut Vec<u8>) {
        let len = self.len() as u8; // TODO: This is really bad, lmao -> This limits max string size to `255` before it will overflow

        buf.reserve(len as usize + 1);
        buf.push(len);
        buf.extend(self.into_bytes());
    }

    fn decode_impl_offset() -> String {
        format!("window.decodeString(data, offset)")
    }
}

impl IntoBytes for Vec<u8> {
    fn len_hint(&self) -> u64 {
        (self.len() as u64) + 1
    }

    fn bytes_into_buf(self, buf: &mut Vec<u8>) {
        let len = self.len() as u8; // TODO: This is really bad, lmao -> This limits max buffer size to `255` before it will overflow

        buf.reserve(len as usize + 1);
        buf.push(len);
        buf.extend(self);
    }

    fn decode_impl_offset() -> String {
        format!("window.decodeBuf(data, offset)")
    }
}

impl IntoBytes for u128 {
    fn len_hint(&self) -> u64 {
        (self.to_string().as_bytes().len() as u64) + 1
    }

    fn bytes_into_buf(self, buf: &mut Vec<u8>) {
        let bytes = self.to_string();
        let len = bytes.as_bytes().len() as u8; // TODO: This is really bad, lmao -> This limits max string size to `255` before it will overflow

        buf.reserve(len as usize + 1);
        buf.push(len);
        buf.extend(bytes.as_bytes());
    }
    fn decode_impl_offset() -> String {
        format!("window.decodeU128(data, offset)")
    }
}
