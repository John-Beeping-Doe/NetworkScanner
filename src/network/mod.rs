// src/network/mod.rs

pub mod ping;
pub mod dns;
pub mod traceroute;

pub use ping::{PingResult, ping_task};
pub use traceroute::{TracerouteResult, traceroute_task};
