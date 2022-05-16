use std::thread;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_util::codec;
use tokio_util::codec::{BytesCodec, Decoder};


#[tokio::main]
async fn main2() -> Result<(), Box<dyn std::error::Error>>{

    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (tcp_stream, _) = listener.accept().await?;
        // 別スレッドをたててread writeの処理をしている
        // こうしないと, あるリクエストの処理が終わるまで別リクエストの処理ができない
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let mut framed = BytesCodec::new().framed(tcp_stream);
            while let Some(message) = framed.next().await {
                match message {
                    Ok(bytes) => println!("{:?}", bytes),
                    Err(e) => println!("sss")
                }
            }

            loop {

                // let n = tcp_stream.read(&mut buf).await.expect("faile to read data from socket");
                // println!("{:?}", buf);
                //
                // if n == 0 {
                //     return ;
                // }
                //
                // tcp_stream.write_all(&buf[0..n]).await.expect("");
            }
        });

    }
}
