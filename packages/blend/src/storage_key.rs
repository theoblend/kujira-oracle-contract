pub const fn as_str<T>(key: &T) -> &str {
    let array_ref = unsafe { std::mem::transmute::<_, &[u8; 1]>(key) };
    match core::str::from_utf8(array_ref) {
        Ok(a) => a,
        Err(_) => panic!("Non-utf8 enum value found. Use a-z, A-Z and 0-9."),
    }
}
