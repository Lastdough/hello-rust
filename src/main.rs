use base64::{engine::general_purpose, Engine as _};
use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
    // println!("Hello, world!");
    
    let stdout: std::io::Stdout = stdout();
    let message: String = String::from("Hello Friend!");
    let width: usize = message.chars().count();
    println!("message : {}", &message);

    let b64 = general_purpose::STANDARD.encode(message);
    println!("base64: {}", b64);

    let decoded_vec = general_purpose::STANDARD.decode(b64.clone()).unwrap();
    println!("decoded vector utf-8: {:?}", &decoded_vec);

    let decoded_message = String::from_utf8(decoded_vec).unwrap();
    println!("decoded message: {}", &decoded_message);

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(&decoded_message, width, &mut writer).unwrap();
}
