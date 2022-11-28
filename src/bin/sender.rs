use std::{os::unix::net::UnixDatagram, time::Duration};

fn main() -> anyhow::Result<()> {
    std::fs::remove_file("b.socket")?;
    let socket = UnixDatagram::bind("b.socket")?;
    let buf = [0; 1024 * 2];
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
