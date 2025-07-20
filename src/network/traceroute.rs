// src/network/traceroute.rs

use tokio::sync::mpsc::Sender;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct TracerouteResult {
    pub addr: String,
    pub ok: bool,
    pub hops: Vec<String>,
    pub err: Option<String>,
}

pub async fn traceroute_task(addr: String, tx: Sender<TracerouteResult>) {
    let mut result = TracerouteResult {
        addr: addr.clone(),
        ok: false,
        hops: Vec::new(),
        err: None,
    };

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("tracert");
        c.arg(&addr);
        c
    } else {
        let mut c = Command::new("traceroute");
        c.arg(&addr);
        c
    };
    cmd.stdout(Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout).lines();
                tokio::pin!(reader);

                // Skip the first line (header), then read each hop as a line
                let mut first = true;
                while let Some(line) = reader.next_line().await.unwrap_or(None) {
                    let trimmed = line.trim();
                    if trimmed.is_empty() { continue; }
                    // Only include lines that start with a digit (hop lines)
                    if trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                        result.hops.push(trimmed.to_string());
                        // Send partial result after each hop
                        let _ = tx.send(result.clone()).await;
                    }
                    // Otherwise, ignore warnings, headers, etc.
                }

                result.ok = true;
            } else {
                result.err = Some("No stdout for traceroute".to_string());
            }
        }
        Err(e) => {
            result.err = Some(format!("Failed to start traceroute: {e}"));
        }
    }
    // Final send to ensure UI has last state
    let _ = tx.send(result).await;
}
