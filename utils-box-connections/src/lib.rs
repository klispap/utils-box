//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## SSH Client
//! Connect via SSH to a server to perform commands, upload & download files
//!
//! Mininal Example:
//! ```ignore
//!     let ssh = SshClient::local("user".to_string(), "1234".to_string()).unwrap();
//!
//!     let stdout = ssh.execute_cmd("ls").unwrap();
//!
//!     println!("{:?}", stdout);
//!
//! ```
//!
//! ## TCP Client
//! Connect via TCP to a socket to send and receive data
//!
//! Mininal Example:
//! ```ignore
//!     let mut tcp_client = TcpClient::new("192.168.1.17".to_string(), 36457)?;
//!
//!     let data: Vec<u8> = vec![8, 30, 15, 30, 5, 19, 0, 7];
//!
//!     tcp_client.send(&data)?;
//!
//!     // Block and wait for response
//!     let resp = tcp_client.receive()?;
//!
//!      println!("{:?}", resp);
//!
//! ```
//!
//! ## TCP Client
//! Connect via UDP to a socket to send and receive data
//!
//! Mininal Example:
//! ```ignore
//!      let mut udp =
//!             UdpClient::new("0.0.0.0".to_string(), "192.168.1.31".to_string(), 6123).unwrap();
//!
//!     udp.send(b"\r").unwrap();
//!
//!     // Block and wait for response
//!     let data = udp.receive().unwrap();
//!
//!     println!("{:?} => {}", data, String::from_utf8_lossy(&data));
//!
//! ```
//!
//! ## ZMQ Client
//! Connect to a ZMQ server to send and receive data
//!
//! Mininal Example:
//! ```ignore
//!     let zmq_client = ZmqClient::new(("192.168.1.17".to_string(), 36457)?;
//!
//!     let data: Vec<u8> = vec![8, 30, 15, 30, 5, 19, 0, 7];
//!
//!     zmq_client.send(&data)?;
//!
//!     // Block and wait for response
//!     let resp = zmq_client.receive()?;
//!
//!     println!("{:?}", resp);
//!
//! ```
//!

#[cfg(feature = "ssh")]
pub mod ssh_client;
#[cfg(feature = "tcp")]
pub mod tcp_client;
#[cfg(feature = "udp")]
pub mod udp_client;
#[cfg(feature = "zmq")]
pub mod zmq_client;
