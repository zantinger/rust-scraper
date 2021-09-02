use std::io;
use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    println!("Type URL");

    let mut url = String::new();

    io::stdin()
        .read_line(&mut url)
        .expect("cannot read url");

    let mut buffer = File::create("foobar.txt")?;
    buffer.write_all(&url.as_bytes())?;

    Ok(())
}
