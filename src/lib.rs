use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use rand::rngs::OsRng;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, Output};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(PartialEq)]
pub enum AesMode {
    Gcm,
    Cbc,
}

pub fn zip_folder(folder_path: &str, zip_path: &str) -> Result<(), String> {
    let file = File::create(zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let folder = PathBuf::from(folder_path);
    for entry in WalkDir::new(&folder) {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path.strip_prefix(&folder).unwrap().to_str().unwrap();
        if path.is_file() {
            zip.start_file(name, options).map_err(|e| e.to_string())?;
            let mut f = File::open(path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
        } else if !name.is_empty() {
            zip.add_directory(name, options).map_err(|e| e.to_string())?;
        }
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn unzip_to_folder(zip_path: &str, folder_path: &str) -> Result<(), String> {
    let file = File::open(zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = PathBuf::from(folder_path).join(file.name());
        if file.is_dir() {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn encrypt_file(
    input: &str,
    output: &str,
    password: &str,
    mode: AesMode,
) -> Result<(), &'static str> {
    if mode != AesMode::Gcm {
        return Err("Only AES-GCM is implemented");
    }
    // Read input file
    let plaintext = fs::read(input).map_err(|_| "Failed to read input file")?;
    // Generate salt and nonce
    let mut salt = [0u8; 16];
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);
    // Derive key
    let argon2 = Argon2::default();
    let salt_str = SaltString::encode_b64(&salt).map_err(|_| "Salt encode error")?;
    let password_hash = argon2.hash_password(password.as_bytes(), &salt_str)
        .map_err(|_| "Key derivation failed")?;
    let hash_output: Output = password_hash.hash.ok_or("No hash output")?;
    let key_bytes = hash_output.as_bytes();
    if key_bytes.len() < 32 {
        return Err("Derived key too short");
    }
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes[..32]);
    let cipher = Aes256Gcm::new(key);
    // Encrypt
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| "Encryption failed")?;
    // Write output: salt + nonce + ciphertext
    let mut out = fs::File::create(output).map_err(|_| "Failed to create output file")?;
    out.write_all(&salt).map_err(|_| "Write error")?;
    out.write_all(&nonce).map_err(|_| "Write error")?;
    out.write_all(&ciphertext).map_err(|_| "Write error")?;
    Ok(())
}

pub fn decrypt_file(
    input: &str,
    output: &str,
    password: &str,
    mode: AesMode,
) -> Result<(), &'static str> {
    if mode != AesMode::Gcm {
        return Err("Only AES-GCM is implemented");
    }
    // Read input file
    let data = fs::read(input).map_err(|_| "Failed to read input file")?;
    if data.len() < 16 + 12 {
        return Err("Input file too short");
    }
    let salt = &data[..16];
    let nonce = &data[16..28];
    let ciphertext = &data[28..];
    // Derive key
    let argon2 = Argon2::default();
    let salt_str = SaltString::encode_b64(salt).map_err(|_| "Salt encode error")?;
    let password_hash = argon2.hash_password(password.as_bytes(), &salt_str)
        .map_err(|_| "Key derivation failed")?;
    let hash_output: Output = password_hash.hash.ok_or("No hash output")?;
    let key_bytes = hash_output.as_bytes();
    if key_bytes.len() < 32 {
        return Err("Derived key too short");
    }
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes[..32]);
    let cipher = Aes256Gcm::new(key);
    // Decrypt
    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| "Decryption failed (wrong password or corrupted data)")?;
    // Write output
    let mut out = fs::File::create(output).map_err(|_| "Failed to create output file")?;
    out.write_all(&plaintext).map_err(|_| "Write error")?;
    Ok(())
}