//! # TCP Client utility
//! A small TCP utility to connect and manipulate TCP connections to a server.
//! Useful for sending and receiving data via a TCP socket.

use anyhow::{bail, Result};
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use crate::{log_error, log_info, log_trace};

pub static BUFFER_SIZE: usize = 500;

pub struct TcpClient {
    server_ip: String,
    server_port: u16,
    tcp_stream: TcpStream,
}

impl TcpClient {
    /// Create a new TCP connection to the specified server and connect to it
    pub fn new(server_ip: String, server_port: u16) -> Result<Self> {
        let tcp_stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;

        Ok(Self {
            server_ip,
            server_port,
            tcp_stream,
        })
    }

    /// Get the server IP and port information
    pub fn server_info(&self) -> (String, u16) {
        (self.server_ip.clone(), self.server_port)
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        match self.tcp_stream.write(data) {
            Ok(_) => {
                log_trace!(
                    "[TcpClient][{}:{}] Send [{} Bytes] SUCCESSFULLY!",
                    self.server_ip,
                    self.server_port,
                    data.len()
                );
                Ok(())
            }
            Err(e) => {
                log_error!(
                    "[TcpClient][{}:{}] Send FAILED with [{}]",
                    self.server_ip,
                    self.server_port,
                    e
                );
                bail!(e)
            }
        }
    }

    pub fn receive(&mut self) -> Result<Vec<u8>> {
        let mut data: Vec<u8> = vec![0; BUFFER_SIZE];

        match self.tcp_stream.read(&mut data) {
            Ok(data_len) => {
                log_trace!(
                    "[TcpClient][{}:{}] Received [{} Bytes] SUCCESSFULLY!",
                    self.server_ip,
                    self.server_port,
                    data_len
                );
                Ok(data[..data_len].to_vec())
            }
            Err(e) => {
                log_error!(
                    "[TcpClient][{}:{}] Receive FAILED with [{}]",
                    self.server_ip,
                    self.server_port,
                    e
                );
                bail!(e)
            }
        }
    }
}

impl Drop for TcpClient {
    fn drop(&mut self) {
        match self.tcp_stream.shutdown(Shutdown::Both) {
            Ok(_) => log_info!(
                "[TcpClient] Disconnected from [{}:{}]",
                self.server_ip,
                self.server_port
            ),
            Err(_) => log_error!(
                "[TcpClient] FAILED to Grecefully Disconnect from [{}:{}]. Dropping...",
                self.server_ip,
                self.server_port
            ),
        }
    }
}
