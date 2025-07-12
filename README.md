# aes4linux

A simple command-line tool to securely encrypt and decrypt files or folders using a password. Uses AES-256-GCM encryption and Argon2 password-based key derivation for strong security.

## Features
- Encrypt or decrypt any file or folder
- Password-based encryption (no key files needed)
- Secure: AES-256-GCM, Argon2, random salt and nonce
- **Secure deletion**: Original files are securely overwritten and deleted after encryption/decryption
- **No sensitive data logging or storage**
- **Signal handling**: Ctrl+C cleanup ensures temp files are securely deleted
- **Memory protection**: Passwords are zeroized after use

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
- If `output_file` is omitted, output will be `<file>_encrypted`
- **Original file is securely deleted after encryption**

### Decrypt a file
```sh
./aes4linux -d <encrypted_file> <password> [output_file]
```
- If `output_file` is omitted, output will be `<encrypted_file>_decrypted.txt`
- **Encrypted file is securely deleted after decryption**

### Encrypt a folder
```sh
./aes4linux -e <folder> <password> [output_file]
```
- The folder is zipped and then encrypted
- If `output_file` is omitted, output will be `<folder>_encrypted`
- **Original folder is securely deleted after encryption**

### Decrypt a folder
```sh
./aes4linux -d <encrypted_file> <password> [output_folder]
```
- The encrypted file is decrypted and extracted to the output folder
- If `output_folder` is omitted, output will be `<encrypted_file>_decrypted.txt`
- **Encrypted file is securely deleted after decryption**

## Examples

### Basic file encryption
```sh
# Encrypt a file
./aes4linux -e secret.txt mypassword
# Creates: secret.txt_encrypted
# Deletes: secret.txt (securely)

# Decrypt the file
./aes4linux -d secret.txt_encrypted mypassword
# Creates: secret.txt_encrypted_decrypted.txt
# Deletes: secret.txt_encrypted (securely)
```

### Folder encryption
```sh
# Encrypt a folder
./aes4linux -e myfolder mypassword
# Creates: myfolder_encrypted
# Deletes: myfolder (securely)

# Decrypt the folder
./aes4linux -d myfolder_encrypted mypassword
# Creates: myfolder_encrypted_decrypted.txt (extracted folder)
# Deletes: myfolder_encrypted (securely)
```

### Custom output names
```sh
# Encrypt with custom output name
./aes4linux -e document.pdf mypassword backup.enc

# Decrypt with custom output name
./aes4linux -d backup.enc mypassword original.pdf
```

## Security Features

### Encryption
- **AES-256-GCM**: Industry-standard encryption with authentication
- **Argon2**: Memory-hard password-based key derivation
- **Random salt and nonce**: Each encryption is unique
- **No metadata**: File format reveals nothing about content

### Secure Deletion
- **Overwrite before delete**: Files are overwritten with random data before deletion
- **Signal handling**: Ctrl+C or process termination triggers secure cleanup
- **Memory zeroization**: Passwords are cleared from memory after use
- **Temp file cleanup**: Temporary files are securely deleted

### Privacy
- **No logging**: No sensitive data is logged
- **No configuration files**: No persistent settings that could leak information
- **No network access**: Works completely offline

## File Format

Encrypted files contain:
1. **Salt** (16 bytes): For Argon2 key derivation
2. **Nonce** (12 bytes): For AES-GCM encryption
3. **Ciphertext**: The encrypted data

## Error Handling

- **Wrong password**: Clear error message without revealing information
- **Corrupted files**: Detection and reporting of file corruption
- **Missing files**: Helpful error messages for missing input files
- **Permission errors**: Clear reporting of file system issues

---

**Note:** Always use strong passwords for best security. The tool is designed for privacy-focused use and leaves minimal traces of operation. 