use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

pub fn create_hash_from(mut plan_text: String) -> String {
    // あえてCPU高負荷な処理をさせている
    let a = String::from("");
    for i in 1..100000000 {
        let i_c = std::char::from_digit(i as u32, 10).unwrap_or('1');
        plan_text.push(i_c); // pushメソッドが参照を取るようになっているので、&plan_text.pushとする必要はない
    }
    let mut hasher = Sha256::new();
    let byte_text = plan_text.as_bytes();
    hasher.update(byte_text);
    let output = hasher.finalize();
    Base64::encode_string(&output)
}
