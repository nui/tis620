fn main() {
    let message = "แมว";
    let encoded = tis620::encode(&message).expect("TIS-620 encoded");
    println!("text:  : {}", message);
    println!("TIS-620: {:X?}", encoded);
    println!("UTF-8  : {:X?}", message.as_bytes());
    assert_eq!(tis620::decode(&encoded).unwrap(), message);
}
