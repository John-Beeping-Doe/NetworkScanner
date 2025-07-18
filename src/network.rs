use tokio::sync::mpsc::Sender;
use std::process::Stdio;

#[derive(Clone, Debug)]
pub struct PingResult {
    pub addr: String,
    pub ok: bool,
    pub rtt_ms: Option<u128>,
    pub err: Option<String>,
    pub timestamp: Option<std::time::SystemTime>,
    pub seq: usize,
}

// Spawns system ping command once per second and sends result via channel
pub async fn ping_task(addr: String, tx: Sender<PingResult>) {
    let mut seq = 0;
    loop {
        let output = if cfg!(target_os = "windows") {
            tokio::process::Command::new("ping")
                .arg("-n").arg("1")
                .arg(&addr)
                .stdout(Stdio::piped())
                .output()
                .await
        } else {
            tokio::process::Command::new("ping")
                .arg("-c").arg("1")
                .arg(&addr)
                .stdout(Stdio::piped())
                .output()
                .await
        };

        let mut result = PingResult {
            addr: addr.clone(),
            ok: false,
            rtt_ms: None,
            err: None,
            timestamp: Some(std::time::SystemTime::now()),
            seq, // <-- THIS FIELD ADDED!
        };

        match output {
            Ok(out) if out.status.success() => {
                result.ok = true;
                let stdout = String::from_utf8_lossy(&out.stdout);
                result.rtt_ms = parse_rtt_ms(&stdout);
            }
            Ok(out) => {
                result.err = Some(format!("Ping failed: {}", String::from_utf8_lossy(&out.stdout)));
            }
            Err(e) => {
                result.err = Some(format!("Failed to start ping: {e}"));
            }
        }

        let _ = tx.send(result).await;
        seq += 1;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

// Basic RTT parser for Unix and Windows ping output
fn parse_rtt_ms(output: &str) -> Option<u128> {
    if cfg!(target_os = "windows") {
        // Look for "Average = 12ms"
        output.lines().find_map(|line| {
            if line.contains("Average =") {
                // Use next_back() instead of last() to silence clippy warning!
                line.split('=').next_back()?.trim().replace("ms", "").parse().ok()
            } else {
                None
            }
        }).map(|ms: u128| ms)
    } else {
        // Look for "time=12.3 ms"
        output.lines().find_map(|line| {
            line.split_whitespace().find_map(|part| {
                if part.starts_with("time=") {
                    part.trim_start_matches("time=").replace("ms", "").parse::<f64>().ok().map(|f| f as u128)
                } else {
                    None
                }
            })
        })
    }
}
