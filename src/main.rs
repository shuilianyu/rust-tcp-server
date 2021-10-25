use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;
use std::panic;


// TCP流处理函数
fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512];
    for _ in 0..1000 {
        // 从流中读取数据
        let bytes_read = stream.read(&mut buf)?;
        // 如果本次读到数据长度为0，直接return
        if bytes_read == 0 {
            return Ok(());
        }

        // 模式匹配练习，如果读取到的数据长度大于10，出现异常; 小于10，打印内容
        if (bytes_read > 10) {
            panic!("bytes_read too large");
        } else {
            println!("{}", str::from_utf8(&buf[..bytes_read]).expect("Could not write buffer as string"));
        }

        // 将数据返回给client端
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // 建立连接
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        // 获取数据流
        let stream = stream.expect("failed!");

        let handle = thread::spawn(move || {
            // 数据流内容处理
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        });

        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
