//! # UDP Client utility
//! A small UDP utility to connect and manipulate UDP connections to a server.
//! Useful for sending and receiving data via a UDP socket.

use anyhow::{Result, bail};
use std::net::UdpSocket;

use crate::{log_error, log_info, log_trace};

pub static BUFFER_SIZE: usize = 500;

pub struct UdpClient {
    server_ip: String,
    server_port: u16,
    udp_socket: UdpSocket,
}

impl UdpClient {
    pub fn new(local_ip: String, server_ip: String, server_port: u16) -> Result<Self> {
        let udp_socket = UdpSocket::bind(format!("{local_ip}:{server_port}"))?;

        Ok(UdpClient {
            server_ip,
            server_port,
            udp_socket,
        })
    }

    pub fn connection_info(&self) -> (String, u16) {
        (self.server_ip.clone(), self.server_port)
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        match self
            .udp_socket
            .send_to(data, format!("{}:{}", self.server_ip, self.server_port))
        {
            Ok(_) => {
                log_trace!(
                    "[UdpClient][{}:{}] Send [{} Bytes] to [{}] SUCCESSFULLY!",
                    self.server_ip,
                    self.server_port,
                    data.len(),
                    self.server_ip,
                );
                Ok(())
            }
            Err(e) => {
                log_error!(
                    "[UdpClient][{}:{}] Send FAILED with [{}]",
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

        match self.udp_socket.recv_from(&mut data) {
            Ok((data_len, src_addr)) => {
                log_trace!(
                    "[UdpClient][{}:{}] Received [{} Bytes] from [{}] SUCCESSFULLY!",
                    self.server_ip,
                    self.server_port,
                    data_len,
                    src_addr,
                );
                Ok(data[..data_len].to_vec())
            }
            Err(e) => {
                log_error!(
                    "[UdpClient][{}:{}] Receive FAILED with [{}]",
                    self.server_ip,
                    self.server_port,
                    e
                );
                bail!(e)
            }
        }
    }
}

impl Drop for UdpClient {
    fn drop(&mut self) {
        log_info!(
            "[UdpClient] Disconnected from [{}:{}]",
            self.server_ip,
            self.server_port
        );
    }
}
