[![Coverage Status](https://coveralls.io/repos/github/klispap/utils-box/badge.svg?branch=main)](https://coveralls.io/github/klispap/utils-box?branch=main)

# Summary
A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.

# Utilities provided:
 
## Archives
Extract files from Tar, Gz and Zip Files

Mininal Example:
```rust
let archive: PathBuf = std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap()
    .join("test_archive.tar.gz");

let file: PathBuf = "treasure.hex".into();

let destination: PathBuf = std::env::current_exe()
    .unwrap()
    .parent()
    .unwrap();

archives::extract_file(archive, ArchiveType::Gz, file, destination).unwrap();

```
 
# Tips for resolving Ubuntu 22.04/24.04 build issues:

1) Make sure you have the following system-level dependencies installed:
    ```
    sudo apt install pkg-config build-essential fontconfig libfontconfig1-dev
    ``` 

2) Verify that `pkg-config` can detect `libstdc++` properly:
    ```
    pkg-config --libs libstdc++
    ```

3) If `libstdc++` is not detected, add the symbolic link:
    ```
    sudo ln -s /usr/lib/gcc/x86_64-linux-gnu/11/libstdc++.so /usr/lib/libstdc++.so
    ```

