# file-renamer
file-renamer is a command-line tool for renaming files in a directory according to a specified rule.

# Installation
To install file-renamer, you will need to have Rust installed on your machine. You can install Rust using rustup.

Once Rust is installed, use `cargo` to install

```bash
$ cargo install file-renamer
```

## Build from source
clone this repository and build file-renamer using Cargo:

```bash
$ git clone https://github.com/ItzNesbroDev/file-renamer.git
$ cd file-renamer
$ cargo build --release
```

This will create a release binary in the target/release directory, which you can then copy to a location in your PATH for easy access.

# Usage
To use file-renamer, simply specify the directory containing the files you want to rename and the rule you want to apply:

```bash
file-renamer path/to/directory uppercase
```

The first option specifies the path to the directory containing the files you want to rename, and the second option specifies the transformation to apply to the file names. Possible values for second are uppercase and lowercase.

# Examples
To rename all the files in the documents directory to uppercase:

```bash
file-renamer documents uppercase
```

To rename all the files in the downloads directory to lowercase:

```bash
file-renamer downloads lowercase
```

# License
file-renamer is licensed under the MIT License. See LICENSE for more details
