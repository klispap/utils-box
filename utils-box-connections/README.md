[![Coverage Status](https://coveralls.io/repos/github/klispap/utils-box/badge.svg?branch=main)](https://coveralls.io/github/klispap/utils-box?branch=main)

# Summary
A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.

# Utilities provided:
 
## SSH Client
Connect via SSH to a server to perform commands, upload & download files

Mininal Example:
```rust
    let ssh = SshClient::local("user".to_string(), "1234".to_string()).unwrap();

    let stdout = ssh.execute_cmd("ls").unwrap();

    println!("{:?}", stdout);

```

## TCP Client
Connect via TCP to a socket to send and receive data

Mininal Example:
```rust
    let mut tcp_client = TcpClient::new("192.168.1.17".to_string(), 36457)?;

    let data: Vec<u8> = vec![8, 30, 15, 30, 5, 19, 0, 7];

    tcp_client.send(&data)?;

    // Block and wait for response
    let resp = tcp_client.receive()?;

     println!("{:?}", resp);

```

## TCP Client
Connect via UDP to a socket to send and receive data

Mininal Example:
```rust
     let mut udp =
            UdpClient::new("0.0.0.0".to_string(), "192.168.1.31".to_string(), 6123).unwrap();

    udp.send(b"\r").unwrap();

    // Block and wait for response
    let data = udp.receive().unwrap();

    println!("{:?} => {}", data, String::from_utf8_lossy(&data));

```

## ZMQ Client
Connect to a ZMQ server to send and receive data

Mininal Example:
```rust
    let zmq_client = ZmqClient::new(("192.168.1.17".to_string(), 36457)?;

    let data: Vec<u8> = vec![8, 30, 15, 30, 5, 19, 0, 7];

    zmq_client.send(&data)?;

    // Block and wait for response
    let resp = zmq_client.receive()?;

    println!("{:?}", resp);

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

