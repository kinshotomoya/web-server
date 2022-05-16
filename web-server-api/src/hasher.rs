use serde::__private::de::Content::U8;
use sha2::{Sha256, Digest};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::Output;
use base64ct::{Base64, Encoding};

pub fn create_hash_from(mut plan_text: String) -> String {
    // あえてCPU高負荷な処理をさせている
    for i in 1..100000000 {
        let i_c = std::char::from_digit(i as u32, 10).unwrap_or('1');
        &plan_text.push(i_c);
    }
    let mut hasher = Sha256::new();
    let byte_text =  plan_text.as_bytes();
    hasher.update(byte_text);
    let output = hasher.finalize();
    let hash = Base64::encode_string(&output);
    println!("{}", hash);
    hash
}
