fn main() {
    let u: &[u8] = [49, 50, 51];

    unsafe {
        assert!(u == std::cast::transmute("123"));
    }
}
