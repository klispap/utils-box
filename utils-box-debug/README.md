[![Coverage Status](https://coveralls.io/repos/github/klispap/utils-box/badge.svg?branch=main)](https://coveralls.io/github/klispap/utils-box?branch=main)

# Summary
A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.

# Utilities provided:
 
## Config
Print in log or stdout debug information from vectors, hashmaps in a human readable way.
Pause execution at specific moments to make debugging easier.

Mininal Example:

```rust
    // Complex data operations before [..]

    let data: Vec<f64> = (0..100).iter().map(|&x| x * f64::PI).collect();
    // Print debug information from data vector
    vector_display(&data[0..10],"Mult_PI", IdxMode::Based1);
    // Pause execution to check values
    pause();

    // Complex data operations after [..]
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

