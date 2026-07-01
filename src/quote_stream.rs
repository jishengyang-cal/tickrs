use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, BufReader, Read};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{OnceLock, RwLock};
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::net::UnixStream;

use serde::Deserialize;

use crate::{DATA_RECEIVED, REDRAW_REQUEST};

static STARTED: AtomicBool = AtomicBool::new(false);
static CACHE: OnceLock<RwLock<HashMap<String, QuoteUpdate>>> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct QuoteUpdate {
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub symbol: String,
    pub price: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub size: Option<u64>,
    pub volume: Option<u64>,
    pub status: Option<String>,
}

impl QuoteUpdate {
    fn is_liveish(&self) -> bool {
        self.status
            .as_deref()
            .map(|status| status.eq_ignore_ascii_case("live"))
            .unwrap_or(true)
    }

    fn live_price(&self) -> Option<f64> {
        if !self.is_liveish() {
            return None;
        }
        self.price.or_else(|| match (self.bid, self.ask) {
            (Some(bid), Some(ask)) if bid > 0.0 && ask > 0.0 => Some((bid + ask) / 2.0),
            _ => None,
        })
    }

    fn volume_string(&self) -> String {
        self.volume
            .or(self.size)
            .map(|value| value.to_string())
            .unwrap_or_default()
    }
}

pub fn start_from_env() {
    let spec = match env::var("TICKRS_QUOTE_STREAM") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return,
    };
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    thread::spawn(move || run_stream_loop(spec));
}

pub fn latest_price(symbol: &str) -> Option<(f64, Option<String>)> {
    let key = normalize_symbol(symbol);
    let guard = CACHE.get_or_init(Default::default).read().ok()?;
    let update = guard.get(&key)?;
    update.live_price().map(|price| {
        (
            price,
            Some(update.volume_string()).filter(|value| !value.is_empty()),
        )
    })
}

fn run_stream_loop(spec: String) {
    loop {
        if let Ok(stream) = connect(&spec) {
            let mut reader = BufReader::new(stream);
            let _ = consume_lines(&mut reader);
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn connect(spec: &str) -> io::Result<Box<dyn Read + Send>> {
    if let Some(path) = spec.strip_prefix("unix://") {
        connect_unix(path)
    } else if let Some(addr) = spec.strip_prefix("tcp://") {
        Ok(Box::new(TcpStream::connect(addr)?))
    } else {
        Ok(Box::new(TcpStream::connect(spec)?))
    }
}

#[cfg(unix)]
fn connect_unix(path: &str) -> io::Result<Box<dyn Read + Send>> {
    Ok(Box::new(UnixStream::connect(path)?))
}

#[cfg(not(unix))]
fn connect_unix(_path: &str) -> io::Result<Box<dyn Read + Send>> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "unix quote streams are only supported on Unix",
    ))
}

fn consume_lines(reader: &mut dyn BufRead) -> io::Result<()> {
    let mut line = String::new();
    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(());
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Ok(update) = serde_json::from_str::<QuoteUpdate>(trimmed) else {
            continue;
        };
        if update.kind.as_deref() == Some("heartbeat") || update.symbol.trim().is_empty() {
            continue;
        }
        if update.live_price().is_none() {
            continue;
        }
        let key = normalize_symbol(&update.symbol);
        if key.is_empty() {
            continue;
        }
        if let Ok(mut guard) = CACHE.get_or_init(Default::default).write() {
            guard.insert(key, update);
        }
        let _ = DATA_RECEIVED.0.try_send(());
        let _ = REDRAW_REQUEST.0.try_send(());
    }
}

fn normalize_symbol(symbol: &str) -> String {
    match symbol.trim().to_uppercase().as_str() {
        "DX-Y.NYB" => "DXY".to_string(),
        other => other.trim_start_matches('^').to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_mid_price_when_last_trade_is_missing() {
        let update = QuoteUpdate {
            kind: None,
            symbol: "AAPL".to_string(),
            price: None,
            bid: Some(10.0),
            ask: Some(10.2),
            size: Some(7),
            volume: None,
            status: Some("live".to_string()),
        };
        assert_eq!(update.live_price(), Some(10.1));
        assert_eq!(update.volume_string(), "7");
    }

    #[test]
    fn normalizes_index_aliases() {
        assert_eq!(normalize_symbol("^VIX"), "VIX");
        assert_eq!(normalize_symbol("dx-y.nyb"), "DXY");
    }
}
