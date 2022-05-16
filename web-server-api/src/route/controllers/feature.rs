use std::time::Duration;

pub async fn future() -> String {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(5));
        println!("sssssss");
    });
    String::from("com")
}
