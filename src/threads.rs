//! # Linux OS utilities
//! A toolbox of small utilities for Linux environments.
//! Useful for fast binary development via the provided signal handling and panic hooks.

use futures::stream::StreamExt;
use log::{error, info};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::panic;
use tokio::task::JoinHandle;

pub fn signal_init() -> (signal_hook::iterator::Handle, JoinHandle<()>) {
    // Termination Signals Initilization
    let signals = Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT]).unwrap();
    let signals_handle = signals.handle();

    let signals_task = tokio::spawn(handle_signals(signals));

    (signals_handle, signals_task)
}

#[cfg(target_family = "unix")]
pub async fn signal_wait(
    signals_handle: signal_hook::iterator::Handle,
    signals_task: JoinHandle<()>,
) {
    signals_handle.close();
    signals_task.await.unwrap();
}

/// Helper Function: Panic Handler
pub fn panic_handler_init() {
    panic::set_hook(Box::new(|panic_info| {
        match panic_info.payload().downcast_ref::<String>() {
            Some(as_string) => {
                error!("PANIC occurred: {as_string}");
            }
            None => {
                error!("PANIC occurred: {panic_info:?}");
            }
        }
        if let Some(location) = panic_info.location() {
            error!(
                "PANIC occurred in file '{}' at line {}",
                location.file(),
                location.line()
            );
        } else {
            error!("PANIC occurred but can't get location information...");
        }

        std::process::abort();
    }));

    info!("SP Scanner [Panic Handler] REGISTERED!");
}

/// Helper Function: Handle of System Termination Signals
#[cfg(target_family = "unix")]
#[allow(clippy::never_loop)]
async fn handle_signals(signals: Signals) {
    use log::warn;

    let mut signals = signals.fuse();

    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP | SIGTERM | SIGINT | SIGQUIT => {
                warn!(
                    "SP Scanner [Termination signals] [{signal:?}] RECEIVED! Terminating Tasks & Exiting..."                   
                );
                signal_hook::low_level::exit(1);
            }
            _ => unreachable!(),
        }
    }
}
