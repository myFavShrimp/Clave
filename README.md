# Clave

Encrypts files in place for you to share them securely.

This application uses the XChaCha20 stream cipher to process your files. XChaCha20 does not provide authentication (read [here](https://en.wikipedia.org/wiki/Authenticated_encryption)).

## Usage

``` bash
$ ./clave
Encrypts files in place for you to share them securely

Usage: clave [PATHS]...

Arguments:
  [PATHS]...

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

``` bash
$ clave ./test_files/
Choose a password to use for processing:
Confirm your password:
22:32:28 [INFO] Processed './test_files/dir_to_process/pic.jpg'
22:32:28 [INFO] Processed './test_files/dir_to_process/subdir/pic.jpg'
22:32:28 [ERROR] Could not write to file './test_files/test_pic.jpg' [0 bytes written]: Permission denied (os error 13)
```
