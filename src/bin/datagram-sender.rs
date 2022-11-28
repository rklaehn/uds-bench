use std::{os::unix::net::UnixDatagram, time::Duration, env};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>(); 
    let n = args.get(1).context("missing buffer size")?.parse::<usize>()?;
    std::fs::remove_file("b.socket")?;
    let socket = UnixDatagram::bind("b.socket")?;
    let buf = vec![0; n];
    let mut i: u64 = 0;
    loop {
        match socket.send_to(&buf, "a.socket") {
            Err(e) => {
                println!("send_to error: {} {} {}", i, e.kind(), e);
                std::thread::sleep(Duration::from_millis(1));
            }
            Ok(n) => {
                i += 1;
                assert!(n == buf.len());
            }
        }
    }
    Ok(())
}
