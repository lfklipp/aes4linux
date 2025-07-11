use clap::{Parser};
use std::path::PathBuf;
use std::fs;
use std::io::{Write, Read};
use std::fs::File;
use std::env::temp_dir;
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;
use rand::RngCore;
mod lib;
use lib::{encrypt_file, decrypt_file, AesMode as LibAesMode, zip_folder, unzip_to_folder};

fn secure_delete(path: &str) {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            if let Ok(mut file) = File::options().write(true).open(path) {
                let len = metadata.len();
                let mut buf = vec![0u8; 4096];
                let mut written = 0;
                while written < len {
                    rand::thread_rng().fill_bytes(&mut buf);
                    let to_write = std::cmp::min(buf.len() as u64, len - written) as usize;
                    let _ = file.write_all(&buf[..to_write]);
                    written += to_write as u64;
                }
                let _ = file.sync_all();
            }
        }
    }
    let _ = fs::remove_file(path);
}

fn secure_delete_folder(path: &PathBuf) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                secure_delete_folder(&p);
            } else {
                secure_delete(p.to_str().unwrap());
            }
        }
    }
    let _ = fs::remove_dir(path);
}

#[derive(Parser)]
#[command(name = "aes4linux")]
#[command(about = "Encrypt and decrypt files with AES and a password", long_about = None)]
struct Cli {
    /// Encrypt
    #[arg(short = 'e', conflicts_with = "decrypt")]
    encrypt: bool,
    /// Decrypt
    #[arg(short = 'd', conflicts_with = "encrypt")]
    decrypt: bool,
    target_file: PathBuf,
    password: String,
    output_file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let input = cli.target_file.to_str().unwrap();
    let mut password = cli.password.clone();
    let output = match &cli.output_file {
        Some(path) => path.to_str().unwrap().to_string(),
        None => {
            if cli.encrypt {
                format!("{}_encrypted", input)
            } else {
                format!("{}_decrypted.txt", input)
            }
        }
    };
    let input_path = PathBuf::from(input);

    let temp_zip = Arc::new(Mutex::new(None));
    let temp_zip_clone = temp_zip.clone();
    ctrlc::set_handler(move || {
        if let Some(ref path) = temp_zip_clone.lock().unwrap().as_deref() {
            secure_delete(path);
        }
        std::process::exit(1);
    }).expect("Error setting Ctrl-C handler");
    if cli.encrypt {
        if input_path.is_dir() {
            let mut temp = temp_dir();
            temp.push("aes4linux_temp.zip");
            *temp_zip.lock().unwrap() = Some(temp.to_str().unwrap().to_string());
            if let Err(e) = zip_folder(input, temp.to_str().unwrap()) {
                eprintln!("Failed to zip folder: {}", e);
                return;
            }
            let result = encrypt_file(temp.to_str().unwrap(), &output, &password, LibAesMode::Gcm);
            secure_delete(temp.to_str().unwrap());
            match result {
                Ok(_) => {
                    println!("Encryption successful: {}", output);
                    // Only delete original if it's different from output
                    if input != output {
                        secure_delete_folder(&input_path);
                    }
                },
                Err(e) => eprintln!("Encryption failed: {}", e),
            }
        } else {
            if let Err(e) = encrypt_file(input, &output, &password, LibAesMode::Gcm) {
                eprintln!("Encryption failed: {}", e);
            } else {
                println!("Encryption successful: {}", output);
                // Only delete original if it's different from output
                if input != output {
                    secure_delete(input);
                }
            }
        }
    } else if cli.decrypt {
        let mut temp = temp_dir();
        temp.push("aes4linux_temp.zip");
        *temp_zip.lock().unwrap() = Some(temp.to_str().unwrap().to_string());
        let result = decrypt_file(input, temp.to_str().unwrap(), &password, LibAesMode::Gcm);
        match result {
            Ok(_) => {
                if let Ok(_) = unzip_to_folder(temp.to_str().unwrap(), &output) {
                    println!("Decryption and extraction successful: {}", output);
                } else {
                    let _ = fs::rename(temp.to_str().unwrap(), &output);
                    println!("Decryption successful: {}", output);
                }
                secure_delete(temp.to_str().unwrap());
                // Only delete encrypted file if it's different from output
                if input != output {
                    if input_path.is_dir() {
                        secure_delete_folder(&input_path);
                    } else {
                        secure_delete(input);
                    }
                }
            },
            Err(e) => {
                eprintln!("Decryption failed: {}", e);
                secure_delete(temp.to_str().unwrap());
            }
        }
    } else {
        eprintln!("Please specify either -e (encrypt) or -d (decrypt)");
    }
    password.zeroize();
} 