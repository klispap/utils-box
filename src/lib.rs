//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Archives
//! Extract files from Tar, Gz and Zip Files
//!
//! Mininal Example:
//! ```ignore
//! let archive: PathBuf = std::env::current_exe()
//!     .unwrap()
//!     .parent()
//!     .unwrap()
//!     .join("test_archive.tar.gz");
//!
//! let file: PathBuf = "treasure.hex".into();
//!
//! let destination: PathBuf = std::env::current_exe()
//!     .unwrap()
//!     .parent()
//!     .unwrap();
//!
//! archives::extract_file(archive, ArchiveType::Gz, file, destination).unwrap();
//!
//! ```
//!
//! ## Bits
//! Convertions between different representations of raw bit streams
//!
//! Mininal Example:
//! ```ignore
//! let received_bit_stream: u64 = 0b110101000100111010110;
//!
//! let bytes = bits::bits_to_vec(received_bit_stream,21);
//!
//! println!("Received bit stream: {} ", bits::bit_vec_to_hex_string(&bytes));
//!
//! ```
//!
//! ## Config
//! Manipulate INI-style configuration files by checking for changes, updates etc
//!
//! Mininal Example:
//! ```ignore
//!     let mut config_changes = ini_compare(
//!         &old_config_path.to_path_buf(),
//!         &new_config_path.to_path_buf(),
//!     )
//!     .unwrap();
//!
//!    println!("{:#?}", config_changes);
//!
//! ```
//!
//! ## Logger
//! Initialize terminal and file loggers fast. Macros for log printing to either log or stdout (if a global logger is not initialized)
//!
//! Mininal Example:
//! ```ignore
//!     log_info!("INFO Test TO PRINTLN!");
//!     log_debug!("DEBUG Test TO PRINTLN!");
//!
//!     terminal_logger_init(LevelFilter::Debug);
//!
//!     log_info!("INFO Test TO LOGGER!");
//!     log_debug!("DEBUG Test TO LOGGER!");
//!
//! ```
//!
//! ## Paths
//! Search paths for a specific file in directories with known or unknown paths
//!
//! Mininal Example:
//! ```ignore
//!     let paths = IncludePathsBuilder::new()
//!             .include_exe_dir()
//!             .include_known("/home/user/")
//!             .include_unknown("utils-box")
//!             .build();
//!
//!     let pattern = "test_*.tar";
//!
//!     let file_found_in = paths.search_glob(pattern);
//!
//! ```
//!
//! ## Stopwatch and Timekeper
//! Keep track of execution times in various points in binaries. Print records.
//!
//! Minimal Example:
//! ```ignore
//!    let mut s = TimeKeeper::init();
//!    let mut t = TimeKeeper::init();
//!
//!    s.totals();
//!
//!    s.lap("init");
//!
//!    for _ in 0..5 {
//!        std::thread::sleep(Duration::from_millis(5));
//!        s.lap("loop");
//!        t.lap("loop");
//!    }
//!    s.lap_totals("loop");
//!    std::thread::sleep(Duration::from_millis(1234));
//!    s.lap("after");
//!
//!    s.totals();
//!    t.totals();
//!
//!    s.merge(t);
//!
//!    s.totals();
//!
//! ```
//!
//! ## Versions
//! version parser from strings using the `semver.org` notations
//!
//! Mininal Example:
//! ```ignore
//!    let version = "0.9.2-1e341234";
//!
//!     let mut expected = Version::new(0, 9, 2);
//!     expected.pre = Prerelease::new("1e341234").unwrap();
//!
//!     assert_eq!(semver_parse(version).unwrap(), expected);
//!
//! ```
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

pub mod archives;
pub mod bits;
pub mod config;
#[cfg(not(tarpaulin_include))]
pub mod debug;
pub mod paths;
pub mod versions;

#[cfg(not(tarpaulin_include))]
#[cfg(target_family = "unix")]
pub mod ssh_client;
#[cfg(not(tarpaulin_include))]
pub mod tcp_client;
#[cfg(not(tarpaulin_include))]
pub mod udp_client;
#[cfg(not(tarpaulin_include))]
pub mod zmq_client;

#[macro_use]
pub mod logger;
pub mod stopwatch;

#[cfg(target_family = "unix")]
#[cfg(not(tarpaulin_include))]
pub mod threads;
