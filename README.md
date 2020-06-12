This crate is a tool for multi-threaded brute forcing passwords of a file encrypted using vim "blowsfish2" encryption type.

Requires rust nightly.

# Usage

```bash
vimbrute -f encrypted_file -t4 < passwords_dictionary | tee promising_passwords
```

Reads standard input for passwords to try, assuming one password per line. A
password is deemed valid if decryption of the first 256 bytes of FILE
produces a valid utf8 string (ignoring possible trailing split character).
Such passwords are printed on the stdout. The program quits when stdin stream
ends, NOT after finding first match.

## Options

- `-f`/`--file` - required, path to the encrypted FILE
- `-t`/`--threads` (default: 1) - number of threads

## Exit code

- `0` - a match was found
- `1` - no match was found
- `2` - other errors occured

# Performance

On my 4 core Intel Core i5-3570 it achieves:

- 4140 passwords/second with `-t4`
- 1118 passwords/second with `-t1`

# LICENSE

MIT.

Parts of the crypto-related code were adapted from [SirVer's vimdecrypt](https://github.com/SirVer/vimdecrypt-rs/).
