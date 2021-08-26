# Clave

Encrypts your files in place for you to share them securely.

This application uses the XChaCha20 stream cipher to process your files. XChaCha20 does not provide authentication (read [here](http://cryptowiki.net/index.php?title=Authenticated_encryption)).

## Usage

``` bash
$ ./clave
clave 0.1.0

Encrypts your files in place to share them securely.

USAGE:
    clave <files>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <files>...    Files to process
```

## Examples

``` bash
$ clave ./test_files/dir_to_process/
These are the paths you have selected for processing:
  "./test_files/dir_to_process/"
Choose a password to use for processing (leave empty to exit):
Confirm your password:
Finished!
The following files were processed successfully:
  ./test_files/dir_to_process/pic.jpg
Errors occurred during the processing of the following files:
  Could not write to file! : ./test_files/dir_to_process/subdir/pic.jpg
```

``` bash
$ clave ./test_files/dir_to_process/subdir/pic.jpg
These are the paths you have selected for processing:
  "./test_files/dir_to_process/subdir/pic.jpg"
Choose a password to use for processing (leave empty to exit):
Confirm your password:
Finished!
The following files were processed successfully:
  ./test_files/dir_to_process/subdir/pic.jpg
```
