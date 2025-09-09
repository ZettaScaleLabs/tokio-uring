use std::time::Duration;

use tokio::time::Instant;
use tokio_uring::net::{TcpStream, UnixStream};

fn main() {
    tokio_uring::start(async {
        let socket = std::os::unix::net::UnixStream::connect("/tmp/echo.sock").unwrap();
        let stream = UnixStream::from_std(socket);
//        let stream = TcpStream::connect("127.0.0.1:10000".parse().unwrap())
//            .await
//            .unwrap();
        const SIZE: usize = 64;

        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let to_write = vec![1u8; SIZE];

            let write_time = Instant::now();

            let (result, buf2) = stream.write_all(to_write).await;
            result.unwrap();

            let (result, _buf2) = stream.read(buf2).await;            
            let ts = write_time.elapsed().as_micros();
            
            assert!(result.unwrap() == SIZE);
            
            println!("RTT: {}, latency: {}", ts, ts / 2);
        }
    });
}
