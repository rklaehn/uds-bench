use std::{os::unix::net::UnixListener, io::Write, env};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>(); 
    let n = args.get(1).context("missing buffer size")?.parse::<usize>()?;
    std::fs::remove_file("/tmp/c.socket").ok();
    let socket = UnixListener::bind("/tmp/c.socket")?;
    let buf = vec![0; n];
    let mut i: u64 = 0;
    loop {
        let (mut stream, addr) = socket.accept()?;
        println!("connected to {:?}", addr);
        loop {
            match stream.write_all(&buf) {
                Err(e) => {
                    println!("write error: {} {} {}", i, e.kind(), e);
                    break;
                }
                Ok(n) => {
                    i += 1;
                }
            }
        }
    }
    Ok(())
}
