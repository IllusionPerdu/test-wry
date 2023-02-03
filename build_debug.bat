echo off
cargo build
rem set RUST_BACKTRACE=full
rem set MIMALLOC_SHOW_STATS=1
.\target\debug\dico-gui.exe
