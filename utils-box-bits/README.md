[![Coverage Status](https://coveralls.io/repos/github/klispap/utils-box/badge.svg?branch=main)](https://coveralls.io/github/klispap/utils-box?branch=main)

# Summary
A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.

# Utilities provided:
 
## Bits
Convertions between different representations of raw bit streams

Mininal Example:
```rust
let received_bit_stream: u64 = 0b110101000100111010110;

let bytes = bits::bits_to_vec(received_bit_stream,21);

println!("Received bit stream: {} ", bits::bit_vec_to_hex_string(&bytes));

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

