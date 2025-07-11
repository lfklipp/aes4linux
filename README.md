# aes4linux

A simple command-line tool to securely encrypt and decrypt files or folders using a password. Uses AES-256-GCM encryption and Argon2 password-based key derivation for strong security.

My priority in this project is to use build features and environment testing. Feel free to copy, paste, or fork this project; I reserve no rights whatsoever.

Don't put your trust into my hobby projects, for production environments consider this AI slop you need to fix first.

## Features
- Encrypt or decrypt any file or folder
- Password-based encryption (no key files needed)
- Secure: AES-256-GCM, Argon2, random salt and nonce
- No sensitive data is logged or stored

# Privacy

This Program is specifically build with privacy in mind it:
- Implements memory zeroization for sensitive data
- Adds signal handling for temp file cleanup

It is not all mighty however and very simple. Only trust (especially in dangerous environments) the program you made yourself and use Tails. 

## Build

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo build --release
```

## Usage

### Encrypt a file
```sh
./aes4linux -e <file> <password> [output_file]
```
- If `output_file` is left empty, output will be `<file>_encrypted`

### Decrypt a file
```sh
./aes4linux -d <encrypted_file> <password> [output_file]
```
- If `output_file` is left empty, output will be `<encrypted_file>_decrypted.txt`

### Encrypt a folder
```sh
./aes4linux -e <folder> <password> [output_file]
```
- The folder is zipped and then encrypted. If `output_file` is left empty, output will be `<folder>_encrypted`

### Decrypt a folder
```sh
./aes4linux -d <encrypted_file> <password> [output_folder]
```
- The encrypted file is decrypted and extracted to the output folder. If `output_folder` is left empty, output will be `<encrypted_file>_decrypted.txt`

## Example
Encrypt a folder:
```sh
./aes4linux -e myfolder mypassword
```
Decrypt it:
```sh
./aes4linux -d myfolder_encrypted mypassword
```

---

**Note:** Always use strong passwords for best security. 

**Warn:** files without an extension are not being encrypted therefor deleted. 