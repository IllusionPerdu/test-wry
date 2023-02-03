echo off
cargo build --release
rem set RUST_BACKTRACE=full
rem set MIMALLOC_SHOW_STATS=1
.\target\release\dico-gui.exe
