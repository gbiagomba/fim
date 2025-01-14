// Copyright (C) 2021, Achiefs.

// To get file checksums
use hex::encode;
use sha3::{Sha3_512, Digest};
use std::io::ErrorKind;
// To log the program process
use log::*;

// To calculate file content hash in sha512 format (SHA3 implementation)
pub fn get_checksum(file: String) -> String {
    let mut hasher = Sha3_512::new();
    match std::fs::read_to_string(file) {
        Ok(data) => {
            hasher.update(&data);
            let result = hasher.finalize().to_vec();
            encode(result)
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    debug!("File Not found error ignoring...");
                    String::from("UNKNOWN")
                },
                _ => {
                    debug!("Error not handled: {:?}", e.kind());
                    String::from("UNKNOWN")
                },
            }
        },
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

    fn create_test_file(filename: String) {
        File::create(filename).unwrap().write_all(b"This is a test!").unwrap();
    }

    fn remove_test_file(filename: String) {
        fs::remove_file(filename).unwrap()
    }

    #[test]
    fn test_get_checksum_file() {
        let filename = String::from("test_get_checksum_file");
        create_test_file(filename.clone());
        assert_eq!(get_checksum(filename.clone()), String::from("46512636eeeb22dee0d60f3aba6473b1fb3258dc0c9ed6fbdbf26bed06df796bc70d4c1f6d50ca977b45f35b494e4bd9fb34e55a1576d6d9a3b5e1ab059953ee"));
        remove_test_file(filename.clone());
    }

    #[test]
    fn test_get_checksum_not_exists() {
        assert_ne!(get_checksum(String::from("not_exists")), String::from("This is a test"));
        assert_eq!(get_checksum(String::from("not_exists")), String::from("UNKNOWN"));
    }

    #[test]
    fn test_get_checksum_bad() {
        let filename = String::from("test_get_checksum_bad");
        create_test_file(filename.clone());
        assert_ne!(get_checksum(filename.clone()), String::from("This is a test"));
        remove_test_file(filename.clone());
    }
}
