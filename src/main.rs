extern crate data_encoding;
#[macro_use]
extern crate error_chain;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use data_encoding::hex;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};


mod errors {
    error_chain! {
        types {
            CError, CErrorKind, CResultExt;
        }
    }
}

use errors::*;

fn main() {
    main_func()
        .unwrap();
}

fn main_func() -> Result<(), CError> {
    let mut file = File::open("6.1.intro.mp4_download")
        .chain_err(|| "Failed to open file")?;

    let eof_pos = file.seek(SeekFrom::End(0))
        .chain_err(|| "Failed to seek end of file")?;

    println!("eof_pos = {}", eof_pos);

    let last_block_pos = eof_pos - eof_pos % 1024;

    println!("last_block_pos = {}", last_block_pos);

    file.seek(SeekFrom::Start(last_block_pos))
        .chain_err(|| "Failed to seek end of file")?;

    // buf holds a file block of 1024 appended by a hash sum of 32 bytesL
    let mut buf = [0u8; 1056];
    let mut hash = [0u8; 32];
    let mut sha = Sha256::new();

    let read_bytes = file.read(&mut buf)
        .chain_err(|| "Failed to read last block")?;

    println!("Read {} bytes", read_bytes);


    sha.input(&buf[0 .. read_bytes]);
    sha.result(&mut hash);
    sha.reset();

    let mut block_pos = last_block_pos;

    while block_pos >= 1024 {
        block_pos = block_pos - 1024;
        file.seek(SeekFrom::Start(block_pos))
            .chain_err(|| format!("Failed to seek position {}", block_pos))?;

        let read_bytes = file.read(&mut buf[0 .. 1024])
            .chain_err(|| "Failed to read last block")?;

        if read_bytes != 1024 {
            println!("Read only {} bytes from position {}", read_bytes, block_pos);
            break;
        }

        buf[1024 .. 1056].copy_from_slice(&hash);

        sha.input(&buf);
        sha.result(&mut hash);
        sha.reset();
    }

    let hex_encoded = hex::encode(&hash)
        .to_string()
        .to_lowercase();

    println!("hex encoded hash sum {}", &hex_encoded);

    Ok(())
}
