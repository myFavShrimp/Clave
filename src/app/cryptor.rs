use std::fs::{OpenOptions, File};
use std::path::PathBuf;
use std::io::{BufReader, BufWriter, Write, BufRead, Error};

use chacha20::{XChaCha20, Key, XNonce};
use chacha20::cipher::{NewCipher, StreamCipher};
use sha3::{Digest, Sha3_224, Sha3_256};

/// Gets a hash of a slice using the provided algorithm.
fn hash_slice<H: Digest>(input: &[u8]) -> Vec<u8> {
    let mut hasher = H::new();
    hasher.update(input);
    hasher.finalize().as_slice().to_owned()
}

/// Generates a nonce from bytes.
fn generate_nonce(input: &[u8]) -> XNonce {
    let hashed = hash_slice::<Sha3_224>(input);
    XNonce::clone_from_slice(&hashed[..24])
}

/// Creates a cipher from a key.
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

/// Tries to get a buffered reader for the file at `file_path`.
fn get_file_reader(file_path: &PathBuf) -> Result<BufReader<File>, Error> {
    File::open(file_path)
        .and_then(|file| Ok(BufReader::new(file)))
}

/// Tries to get a buffered writer for the file at `file_path`.
fn get_file_writer(file_path: &PathBuf) -> Result<BufWriter<File>, Error> {
    OpenOptions::new()
        .write(true)
        .open(file_path)
        .and_then(|file| Ok(BufWriter::new(file)))
}

/// Overwrites a file with its processed contents.
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

/// Tries to encrypt the file at `file_path`.
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

/// Iterates over all files and subdirectories starting at `path` and encrypts their contents.
pub fn encrypt_path(cipher: &mut XChaCha20, path: &PathBuf) -> Vec<FinalEncryptionResult> {
    let mut results: Vec<FinalEncryptionResult> = vec![];
    if path.is_dir() {
        match path.read_dir() {
            Ok(dir_content) => {
                for item in dir_content.filter_map(Result::ok) {
                    for res in encrypt_path(cipher, &item.path()) {
                        match res {
                            Ok(path) => { results.push(Ok(PathBuf::from(path))) }
                            Err((path, message)) => { results.push(Err((PathBuf::from(path), message))) }
                        }
                    }
                }
            }
            Err(_) => { results.push(Err((PathBuf::from(path), READ_DIR_ERROR_MESSAGE))); }
        }
    }
    else if path.is_file() {
        results.push(encrypt_file(cipher, path)
            .and(Ok(PathBuf::from(path)))
            .or_else(|message| Err((PathBuf::from(path), message)))
        );
    }
    else {
        results.push(Err((PathBuf::from(path), PATH_ERROR_MESSAGE)));
    }
    results
}
