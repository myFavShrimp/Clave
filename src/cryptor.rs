use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::PathBuf;

use chacha20::cipher::{NewCipher, StreamCipher};
use chacha20::{Key, XChaCha20, XNonce};
use sha3::{Digest, Sha3_224, Sha3_256};

fn hash_slice<H: Digest>(input: &[u8]) -> Vec<u8> {
    let mut hasher = H::new();
    hasher.update(input);
    hasher.finalize().as_slice().to_owned()
}

fn generate_nonce(input: &[u8]) -> XNonce {
    let hashed = hash_slice::<Sha3_224>(input);
    XNonce::clone_from_slice(&hashed[..24])
}

pub fn create_cipher(key: &[u8]) -> XChaCha20 {
    XChaCha20::new(
        Key::from_slice(hash_slice::<Sha3_256>(key).as_slice()),
        &generate_nonce(key),
    )
}

type EncryptionResult = Result<(), &'static str>;
type FinalEncryptionResult = Result<PathBuf, (PathBuf, &'static str)>;

const WRITE_FILE_ERROR_MESSAGE: &'static str = "Could not write to file!";
const READ_FILE_ERROR_MESSAGE: &'static str = "Could not read from file!";
const READ_DIR_ERROR_MESSAGE: &'static str = "Could not read from file!";
const PATH_ERROR_MESSAGE: &'static str = "Could not determine file path target!";

fn get_file_reader(file_path: &PathBuf) -> Result<BufReader<File>, Error> {
    File::open(file_path).and_then(|file| Ok(BufReader::new(file)))
}

fn get_file_writer(file_path: &PathBuf) -> Result<BufWriter<File>, Error> {
    OpenOptions::new()
        .write(true)
        .open(file_path)
        .and_then(|file| Ok(BufWriter::new(file)))
}

fn process_file(
    cipher: &mut XChaCha20,
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
) -> EncryptionResult {
    let mut length = 1;
    while length > 0 {
        if let Ok(buffer) = reader.fill_buf() {
            let mut data: Vec<u8> = Vec::new();

            data.extend_from_slice(buffer);
            cipher.apply_keystream(&mut data);

            length = buffer.len();
            reader.consume(length);

            if let Err(_) = writer.write(&data) {
                return Err(WRITE_FILE_ERROR_MESSAGE);
            }
        } else {
            return Err(READ_FILE_ERROR_MESSAGE);
        }
    }
    Ok(())
}

fn encrypt_file(cipher: &mut XChaCha20, file_path: &PathBuf) -> EncryptionResult {
    return if let Ok(mut reader) = get_file_reader(file_path) {
        if let Ok(mut writer) = get_file_writer(file_path) {
            // encryption stuff
            // Ok(())
            return process_file(cipher, &mut reader, &mut writer);
        } else {
            Err(WRITE_FILE_ERROR_MESSAGE)
        }
    } else {
        Err(READ_FILE_ERROR_MESSAGE)
    };
}

pub fn encrypt_path(cipher: &mut XChaCha20, path: &PathBuf) -> Vec<FinalEncryptionResult> {
    let mut results: Vec<FinalEncryptionResult> = vec![];
    if path.is_dir() {
        match path.read_dir() {
            Ok(dir_content) => {
                for item in dir_content.filter_map(Result::ok) {
                    for res in encrypt_path(cipher, &item.path()) {
                        match res {
                            Ok(path) => results.push(Ok(PathBuf::from(path))),
                            Err((path, message)) => {
                                results.push(Err((PathBuf::from(path), message)))
                            }
                        }
                    }
                }
            }
            Err(_) => {
                results.push(Err((PathBuf::from(path), READ_DIR_ERROR_MESSAGE)));
            }
        }
    } else if path.is_file() {
        results.push(
            encrypt_file(cipher, path)
                .and(Ok(PathBuf::from(path)))
                .or_else(|message| Err((PathBuf::from(path), message))),
        );
    } else {
        results.push(Err((PathBuf::from(path), PATH_ERROR_MESSAGE)));
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;
    use file_diff::diff_files;

    const TEST_PASSWORD: &'static str = "This is a super secret password no one's able to guess.";
    const TEST_DIR: &'static str =
        &concat!(env!("CARGO_MANIFEST_DIR"), "/test_files/dir_to_process");
    const TEST_PIC_1: &'static str = &concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/test_files/dir_to_process/pic.jpg"
    );
    const TEST_PIC_2: &'static str = &concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/test_files/dir_to_process/subdir/pic.jpg"
    );

    /// Opens a file. Panics if opening fails.
    fn open_file_or_panic(path: &PathBuf) -> File {
        match File::open(path) {
            Ok(f) => return f,
            Err(e) => panic!("Couldn't open test file {:?}! : {}", path, e),
        };
    }

    /// Compares two files. Returns true if they equal.
    fn comp_files(path1: &PathBuf, path2: &PathBuf) -> bool {
        let mut pic1 = open_file_or_panic(&path1);
        let mut pic2 = open_file_or_panic(&path2);

        diff_files(&mut pic1, &mut pic2)
    }

    #[test]
    fn test_hash_str() {
        let hash32 = hash_slice::<Sha3_256>(TEST_DIR.as_bytes());
        let hash28 = hash_slice::<Sha3_224>(TEST_DIR.as_bytes());

        assert_eq!(hash32.len(), 32);
        assert_eq!(hash28.len(), 28);
    }

    #[test]
    fn test_generate_nonce() {
        generate_nonce(TEST_PASSWORD.as_bytes());
    }

    #[test]
    fn test_create_cipher() {
        create_cipher(TEST_PASSWORD.as_bytes());
    }

    #[test]
    fn test_test_files_eq() {
        assert!(comp_files(
            &PathBuf::from(TEST_PIC_1),
            &PathBuf::from(TEST_PIC_2)
        ));
    }

    #[test]
    fn test_encrypt_file() {
        let mut cipher = create_cipher(TEST_PASSWORD.as_bytes());
        let _anything = encrypt_file(&mut cipher, &PathBuf::from(TEST_PIC_1));
        assert!(!comp_files(
            &PathBuf::from(TEST_PIC_1),
            &PathBuf::from(TEST_PIC_2)
        ));

        let mut cipher = create_cipher(TEST_PASSWORD.as_bytes());
        let _anything = encrypt_file(&mut cipher, &PathBuf::from(TEST_PIC_1));
        assert!(comp_files(
            &PathBuf::from(TEST_PIC_1),
            &PathBuf::from(TEST_PIC_2)
        ));
    }
}
