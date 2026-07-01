# tick-rs
[![Actions Status](https://github.com/tarkah/tickrs/workflows/Test/badge.svg)](https://github.com/tarkah/tickrs/actions)

Realtime ticker data in your terminal 📈 Built with Rust. Data sourced from Yahoo! Finance.

  - [Installation](#installation)
  - [Config File](#config-file)
  - [CLI Usage](#cli-usage)
    - [Windows](#windows)
  - [Acknowledgments](#acknowledgments)

<img src="./assets/demo.gif">

## Installation

### Binary

Download the latest [release](https://github.com/tarkah/tickrs/releases/latest) for your platform

### Cargo

```
cargo install tickrs
```

### Arch Linux

```
pacman -S tickrs
```

### Homebrew

```
brew tap tarkah/tickrs
brew install tickrs
```

## Managed Trading Host Defaults

The services Zellij wrapper registers the standard intraday tickrs board with 20 symbols:
SPY, QQQ, IWM, VIX, TLT, DXY, XLK, SMH, XLY, XLC, XLF, XLE, NVDA, AAPL,
MSFT, AMZN, META, GOOGL, TSLA, and AVGO.

When `TICKRS_DATABENTO_LIVE_URL` is configured, current prices are overlaid from
the local Databento live bridge first. If Databento is unavailable or a row is
not live, tickrs falls back to its default Yahoo Finance source. `VIX` and `DXY`
remain the registered UI symbols; Yahoo fallback requests use `^VIX` and
`DX-Y.NYB` respectively.

## Config File

See [wiki entry](https://github.com/tarkah/tickrs/wiki/Config-file)

## CLI Usage

```
tickrs
Realtime ticker data in your terminal 📈

USAGE:
    tickrs [FLAGS] [OPTIONS]

FLAGS:
    -p, --enable-pre-post    Enable pre / post market hours for graphs
    -h, --help               Prints help information
        --hide-help          Hide help icon in top right
        --hide-prev-close    Hide previous close line on 1D chart
        --hide-toggle        Hide toggle block
        --show-volumes       Show volumes graph
    -x, --show-x-labels      Show x-axis labels
        --summary            Start in summary mode
        --trunc-pre          Truncate pre market graphing to only 30 minutes prior to markets opening
    -V, --version            Prints version information

OPTIONS:
    -c, --chart-type <chart-type>              Chart type to start app with [default: line] [possible values: line,
                                               candle, kagi]
    -s, --symbols <symbols>...                 Comma separated list of ticker symbols to start app with
    -t, --time-frame <time-frame>              Use specified time frame when starting program and when new stocks are
                                               added [default: 1D] [possible values: 1D, 1W, 1M, 3M, 6M, 1Y, 5Y]
    -i, --update-interval <update-interval>    Interval to update data from API (seconds) [default: 1]
```

### Windows

Use [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal-preview/9n0dx20hk701) to properly display this app.

## Acknowledgments
- [fdehau](https://github.com/fdehau) / [tui-rs](https://github.com/fdehau/tui-rs) - great TUI library for Rust
- [cjbassi](https://github.com/cjbassi) / [ytop](https://github.com/cjbassi/ytop) - thanks for the inspiration!
