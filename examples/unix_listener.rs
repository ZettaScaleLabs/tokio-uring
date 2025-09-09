use std::env;

use tokio_uring::net::{TcpListener, UnixListener};

fn main() {
    tokio_uring::start(async {

        std::fs::remove_file("/tmp/echo.sock");

        let listener = UnixListener::bind("/tmp/echo.sock").unwrap();
        //let listener = TcpListener::bind("127.0.0.1:10000".parse().unwrap()).unwrap();
        const SIZE: usize = 64;

        loop {
            let (stream) = listener.accept().await.unwrap();
            tokio_uring::spawn(async move {
                let mut buf = vec![1u8; SIZE];

                loop {
                    let (result, buf2) = stream.read(buf).await;
                    assert!(result.unwrap() == SIZE);

                    let (result, buf3) = stream.write(buf2).submit().await;
                    assert!(result.unwrap() == SIZE);

                    buf = buf3;

                    //println!("read from {}: {:?}", &socket_addr, &buf[..read]);
                }
            });
        }
    });
}
