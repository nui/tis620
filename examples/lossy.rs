use tis620::ReplacementChar;

fn main() {
    let message = "ใช้เวลา 42 µs";
    let replacement = ReplacementChar::from_char('m');
    let encoded = tis620::encode_lossy(&message, |_| replacement);
    println!("Message      : {}", message);
    println!("Lossy message: {}", tis620::decode(&encoded).unwrap());
    println!("UTF-8  : {:X?}", message.as_bytes());
    println!("TIS-620: {:X?}", encoded);
}
