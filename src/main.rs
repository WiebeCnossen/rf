use std::{env, fs::File, io::prelude::*};

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    let me = args.next().unwrap();
    let size = args.next().and_then(|a| a.parse().ok()).unwrap_or(35);
    let to = args.next().unwrap_or_else(|| "rf.pdf".to_owned());
    let bits = args.next().and_then(|a| a.parse().ok()).unwrap_or(20);

    println!("{}: {}<<{} -> {}", me, size, bits, to);

    let mut rf = File::create(to)?;
    let mut buf: Vec<u8> = Vec::with_capacity(1 << bits);
    for _ in 0..size {
        for _ in 0..1 << bits {
            buf.push(rand::random());
        }

        rf.write_all(&buf)?;
        buf.truncate(0);
    }

    Ok(())
}
