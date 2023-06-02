use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use chacha20::cipher::{NewCipher, StreamCipher};
use chacha20::{Key, XChaCha20, XNonce};
use sha3::{Sha3_224, Sha3_256};

use crate::hash::hash_slice;

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

pub enum EncryptionResult {
    Ok(PathBuf),
    Error(EncryptionError),
}

#[derive(Default, Debug)]
pub struct FinalEncryptionResult {
    pub oks: Vec<PathBuf>,
    pub errs: Vec<EncryptionError>,
}

impl FinalEncryptionResult {
    pub fn extend(&mut self, result: FinalEncryptionResult) {
        self.oks.extend(result.oks);
        self.errs.extend(result.errs);
    }

    pub fn push_and_log(&mut self, item: EncryptionResult) {
        match item {
            EncryptionResult::Ok(value) => {
                log::info!("Processed '{}'", value.display());
                self.oks.push(value);
            }
            EncryptionResult::Error(value) => {
                log::error!("{}", value);
                self.errs.push(value);
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("Could not write to file '{path}' [{bytes_written} bytes written]: {source}")]
    FileWriteError {
        source: std::io::Error,
        bytes_written: usize,
        path: PathBuf,
    },
    #[error("Could not read from file '{path}' [{bytes_written} bytes written]: {source}")]
    FileReadError {
        source: std::io::Error,
        bytes_written: usize,
        path: PathBuf,
    },
    #[error("Could not read from dir '{path}': {source}")]
    DirReadError {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("Path is not a file/directory '{path}'")]
    PathError { path: PathBuf },
    #[error("Could not process file '{path}': {source}")]
    ProcessingError {
        source: ProcessingError,
        path: PathBuf,
    },
}

fn get_file_reader(file_path: &PathBuf) -> Result<BufReader<File>, EncryptionError> {
    File::open(file_path)
        .map(BufReader::new)
        .map_err(|e| EncryptionError::FileReadError {
            source: e,
            bytes_written: 0,
            path: file_path.clone(),
        })
}

fn get_file_writer(file_path: &PathBuf) -> Result<BufWriter<File>, EncryptionError> {
    OpenOptions::new()
        .write(true)
        .open(file_path)
        .map(BufWriter::new)
        .map_err(|e| EncryptionError::FileWriteError {
            source: e,
            bytes_written: 0,
            path: file_path.clone(),
        })
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Could not write to file [{bytes_written} bytes written]: {source}")]
    WriteError {
        source: std::io::Error,
        bytes_written: usize,
    },
    #[error("Could not read from file [{bytes_written} bytes written]: {source}")]
    ReadError {
        source: std::io::Error,
        bytes_written: usize,
    },
}

fn process_file(
    cipher: &mut XChaCha20,
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
) -> Result<usize, ProcessingError> {
    let mut bytes_written = 0;
    loop {
        let buffer = reader.fill_buf().map_err(|e| ProcessingError::ReadError {
            source: e,
            bytes_written,
        })?;

        let mut data: Vec<u8> = Vec::new();

        data.extend_from_slice(buffer);
        cipher.apply_keystream(&mut data);

        let length = buffer.len();
        if length == 0 {
            return Ok(bytes_written);
        }
        reader.consume(length);

        bytes_written += writer
            .write(&data)
            .map_err(|e| ProcessingError::WriteError {
                source: e,
                bytes_written,
            })?;
    }
}

fn encrypt_file(cipher: &mut XChaCha20, file_path: &PathBuf) -> Result<usize, EncryptionError> {
    let mut reader = get_file_reader(file_path)?;
    let mut writer = get_file_writer(file_path)?;

    process_file(cipher, &mut reader, &mut writer).map_err(|e| EncryptionError::ProcessingError {
        source: e,
        path: file_path.clone(),
    })
}

pub fn encrypt_path(cipher: &mut XChaCha20, path: &PathBuf) -> FinalEncryptionResult {
    let mut result = FinalEncryptionResult::default();

    if path.is_symlink() || !path.exists() {
        result.push_and_log(EncryptionResult::Error(EncryptionError::PathError {
            path: path.clone(),
        }));
    } else if path.is_file() {
        match encrypt_file(cipher, path) {
            Ok(_) => result.push_and_log(EncryptionResult::Ok(path.clone())),
            Err(err) => result.push_and_log(EncryptionResult::Error(err)),
        };
    } else if path.is_dir() {
        match path.read_dir() {
            Ok(dir_content) => {
                for item in dir_content {
                    match item {
                        Ok(dir_entry) => result.extend(encrypt_path(cipher, &dir_entry.path())),
                        Err(e) => result.push_and_log(EncryptionResult::Error(
                            EncryptionError::DirReadError {
                                source: e,
                                path: path.clone(),
                            },
                        )),
                    }
                }
            }
            Err(e) => result.errs.push(EncryptionError::DirReadError {
                source: e,
                path: path.clone(),
            }),
        }
    }

    result
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
