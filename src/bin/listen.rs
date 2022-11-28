use std::os::unix::net::UnixDatagram;

fn main() -> anyhow::Result<()> {
    std::fs::remove_file("a.socket")?;
    let socket = UnixDatagram::bind("a.socket")?;
    let mut buf = [0; 1024 * 2];
    let mut i: u64 = 0;
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        assert!(size == buf.len());
        i += 1;
        if i % 1000 == 0 {
            println!("{} {}", i, i * 1024);
        }
    }
    Ok(())
}
