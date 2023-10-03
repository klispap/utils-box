//! # SSH Client utility
//! A small SSH utility to connect and manipulate SSH connections to a server.
//! Useful for executing commands remotely and uploading/downloading files via SSH.

use anyhow::{bail, Result};
use ssh2::Session;
use std::{
    fs::File, io::Read, io::Write, net::TcpStream, os::unix::prelude::PermissionsExt, path::PathBuf,
};

use crate::{log_info, log_trace};

#[derive(Clone)]
pub struct SshClient {
    server_ip: String,
    server_port: u16,
    ssh_session: Session,
}

impl SshClient {
    pub fn new(
        server_ip: String,
        server_port: u16,
        username: String,
        password: String,
    ) -> Result<Self> {
        // Connect to the local SSH server
        let tcp_stream = TcpStream::connect(format!("{}:{}", server_ip, server_port))?;

        let mut ssh_session = Session::new()?;
        ssh_session.set_tcp_stream(tcp_stream);
        ssh_session.handshake()?;

        ssh_session.userauth_password(&username, &password)?;

        if !ssh_session.authenticated() {
            bail!("[SshClient][new] Authentication FAILED!");
        }

        Ok(SshClient {
            server_ip,
            server_port,
            ssh_session,
        })
    }

    pub fn local(username: String, password: String) -> Result<Self> {
        Self::new("127.0.0.1".to_string(), 22, username, password)
    }

    pub fn connection_info(&self) -> (String, u16) {
        (self.server_ip.clone(), self.server_port)
    }

    pub fn upload(&self, file: PathBuf, remote_file: PathBuf) -> Result<()> {
        // Get access to the local file
        let mut local_file = File::open(&file)?;
        // Get local permissions, remove group permissions
        let local_permissions = local_file.metadata()?.permissions().mode() as i32 & 0o777;

        let mut file_stream = vec![];
        local_file.read_to_end(&mut file_stream)?;

        log_info!(
            "[SshClient][upload] [{} => {}] Permissions:[{:o}] Size: [{} Bytes]",
            file.display(),
            remote_file.display(),
            local_permissions,
            file_stream.len(),
        );

        // Write the file
        let mut remote_file = self.ssh_session.scp_send(
            &remote_file,
            local_permissions,
            file_stream.len() as u64,
            None,
        )?;

        remote_file.write_all(&file_stream)?;

        // Wait for all write operations to finish on remote machine
        std::thread::sleep(std::time::Duration::from_secs(3));

        remote_file.flush()?;

        remote_file.send_eof()?;

        // Close the channel
        remote_file.close()?;

        Ok(())
    }

    pub fn download(&self, remote_file: PathBuf, file: PathBuf) -> Result<()> {
        // Get access to the local file
        let mut local = File::create(&file)?;

        // Get access to the remote file
        let (mut remote, remote_stats) = self.ssh_session.scp_recv(&remote_file)?;

        log_info!(
            "[SshClient][download] [{} => {}] Permissions:[{:o}] Size: [{} Bytes]",
            remote_file.display(),
            file.display(),
            remote_stats.mode(),
            remote_stats.size(),
        );

        // Download content
        let mut file_stream = vec![];
        remote.read_to_end(&mut file_stream)?;

        // Close the channel and wait for the whole content to be tranferred
        remote.send_eof()?;
        remote.wait_eof()?;
        remote.close()?;
        remote.wait_close()?;

        // Write to local
        local.write_all(&file_stream)?;

        // Wait for all write operations to finish on host
        std::thread::sleep(std::time::Duration::from_secs(3));
        local.flush()?;

        // Set permissions
        local
            .metadata()?
            .permissions()
            .set_mode(remote_stats.mode() as u32);

        Ok(())
    }

    pub fn execute_cmd(&self, cmd: &str) -> Result<String> {
        // Open ssh channel
        let mut channel = self.ssh_session.channel_session()?;
        channel.exec(cmd)?;

        // Parse STDOUT
        let mut std_out = String::new();
        channel.read_to_string(&mut std_out)?;
        log_trace!("[SshClient][execute_cmd] STDOUT: \n{}", std_out);

        // Close the channel and wait until is confirmed
        channel.close()?;
        channel.wait_close()?;

        Ok(std_out)
    }
}
