//! # Zero-MQ Client utility
//! A small ZMQ utility to connect and manipulate ZMQ connections to a server.
//! Useful for sending and receiving data via the ZMQ transport layer.

use anyhow::{bail, Result};
use zmq::{Context, Socket};

use crate::{log_error, log_info, log_trace};

pub struct ZmqClient {
    server_ip: String,
    server_port: u16,
    socket: Socket,
}

impl ZmqClient {
    /// Create a new ZMQ connection to the specified server and connect to it
    pub fn new(server_ip: String, server_port: u16) -> Result<Self> {
        let ctx = Context::new();

        let socket = ctx.socket(zmq::REQ)?;
        socket.connect(&format!("tcp://{server_ip}:{server_port}"))?;

        Ok(Self {
            server_ip,
            server_port,
            socket,
        })
    }

    /// Get the server IP and port information
    pub fn server_info(&self) -> (String, u16) {
        (self.server_ip.clone(), self.server_port)
    }

    pub fn send(&self, data: &[u8]) -> Result<()> {
        match self.socket.send(data, 0) {
            Ok(_) => {
                log_trace!(
                    "[ZmqClient][{}:{}] Send [{} Bytes] SUCCESSFULLY!",
                    self.server_ip,
                    self.server_port,
                    data.len()
                );
                Ok(())
            }
            Err(e) => {
                log_error!(
                    "[ZmqClient][{}:{}] Send FAILED with [{}]",
                    self.server_ip,
                    self.server_port,
                    e
                );
                bail!(e)
            }
        }
    }

    pub fn receive(&self) -> Result<Vec<u8>> {
        match self.socket.recv_bytes(0) {
            Ok(data) => {
                log_trace!(
                    "[ZmqClient][{}:{}] Received [{} Bytes] SUCCESSFULLY!",
                    self.server_ip,
                    self.server_port,
                    data.len()
                );
                Ok(data)
            }
            Err(e) => {
                log_error!(
                    "[ZmqClient][{}:{}] Receive FAILED with [{}]",
                    self.server_ip,
                    self.server_port,
                    e
                );
                bail!(e)
            }
        }
    }
}

impl Drop for ZmqClient {
    fn drop(&mut self) {
        match self
            .socket
            .disconnect(&format!("tcp://{}:{}", self.server_ip, self.server_port))
        {
            Ok(_) => log_info!(
                "[ZmqClient] Disconnected from [{}:{}]",
                self.server_ip,
                self.server_port
            ),
            Err(_) => log_error!(
                "[ZmqClient] FAILED to Grecefully Disconnect from [{}:{}]. Dropping...",
                self.server_ip,
                self.server_port
            ),
        }
    }
}
