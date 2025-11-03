[![Coverage Status](https://coveralls.io/repos/github/klispap/utils-box/badge.svg?branch=main)](https://coveralls.io/github/klispap/utils-box?branch=main)

# Summary
A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.

# Utilities provided:
 
## Stopwatch and Timekeper
Keep track of execution times in various points in binaries. Print records.

Minimal Example:
```rust
    let mut s = TimeKeeper::init();
    let mut t = TimeKeeper::init();

    s.totals();

    s.lap("init");

    for _ in 0..5 {
        std::thread::sleep(Duration::from_millis(5));
        s.lap("loop");
        t.lap("loop");
    }
    s.lap_totals("loop");
    std::thread::sleep(Duration::from_millis(1234));
    s.lap("after");

    s.totals();
    t.totals();

    s.merge(t);

    s.totals();

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

