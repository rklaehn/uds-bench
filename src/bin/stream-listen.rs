use std::{os::unix::net::UnixStream, time::Instant, io::Read, env};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>(); 
    let n = args.get(1).context("missing buffer size")?.parse::<usize>()?;
    let mut socket = UnixStream::connect("/tmp/c.socket")?;
    let mut buf = vec![0; n];
    let mut i: u64 = 0;
    let t0 = Instant::now();
    loop {
        socket.read_exact(&mut buf)?;
        i += 1;
        if i % 1000 == 0 {
            let elapsed = t0.elapsed();
            let data = i * (buf.len() as u64);
            println!("{} {}", i, data as f64 / elapsed.as_secs_f64());
        }
    }
    Ok(())
}
