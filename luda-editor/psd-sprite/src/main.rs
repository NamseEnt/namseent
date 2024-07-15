fn main() {
    let psd_bytes = include_bytes!("test.psd");
    let psd = psd::Psd::from_bytes(psd_bytes).unwrap();
    println!("{:#?}", psd);
}
