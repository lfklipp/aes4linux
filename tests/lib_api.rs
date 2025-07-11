use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use aes4linux::{encrypt_file, decrypt_file, AesMode};
use aes4linux::{zip_folder, unzip_to_folder};

#[test]
fn test_file_encrypt_decrypt() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("test.txt");
    let enc_path = dir.path().join("test.txt_encrypted");
    let dec_path = dir.path().join("test.txt_decrypted.txt");
    let password = "testpassword";
    let content = b"Hello, AES4Linux!";
    {
        let mut f = File::create(&input_path).unwrap();
        f.write_all(content).unwrap();
    }
    encrypt_file(input_path.to_str().unwrap(), enc_path.to_str().unwrap(), password, AesMode::Gcm).unwrap();
    decrypt_file(enc_path.to_str().unwrap(), dec_path.to_str().unwrap(), password, AesMode::Gcm).unwrap();
    let result = fs::read(&dec_path).unwrap();
    assert_eq!(result, content);
}

#[test]
fn test_folder_encrypt_decrypt() {
    let dir = tempdir().unwrap();
    let folder = dir.path().join("folder");
    let _ = fs::create_dir(&folder);
    let file1 = folder.join("a.txt");
    let file2 = folder.join("b.txt");
    fs::write(&file1, b"File A").unwrap();
    fs::write(&file2, b"File B").unwrap();
    let enc_path = dir.path().join("folder_encrypted");
    let dec_folder = dir.path().join("folder_decrypted");
    let password = "testpassword";
    let zip_path = dir.path().join("folder.zip");
    zip_folder(folder.to_str().unwrap(), zip_path.to_str().unwrap()).unwrap();
    encrypt_file(zip_path.to_str().unwrap(), enc_path.to_str().unwrap(), password, AesMode::Gcm).unwrap();
    let zip_out = dir.path().join("out.zip");
    decrypt_file(enc_path.to_str().unwrap(), zip_out.to_str().unwrap(), password, AesMode::Gcm).unwrap();
    unzip_to_folder(zip_out.to_str().unwrap(), dec_folder.to_str().unwrap()).unwrap();
    assert_eq!(fs::read(dec_folder.join("a.txt")).unwrap(), b"File A");
    assert_eq!(fs::read(dec_folder.join("b.txt")).unwrap(), b"File B");
} 