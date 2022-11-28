use std::{os::unix::net::UnixDatagram, time::Instant};

fn main() -> anyhow::Result<()> {
    std::fs::remove_file("a.socket")?;
    let socket = UnixDatagram::bind("a.socket")?;
    let mut buf = [0; 1024 * 16];
    let mut i: u64 = 0;
    let t0 = Instant::now();
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        assert!(size == buf.len());
        i += 1;
        if i % 1000 == 0 {
            let elapsed = t0.elapsed();
            let data = i * 1024;
            println!("{} {}", i, data as f64 / elapsed.as_secs_f64());
        }
    }
    Ok(())
}
