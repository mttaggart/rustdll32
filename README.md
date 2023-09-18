# Rustdll32: A Template for Rust DLLs

## Purpose

I frequently build Rust-based DLLs for security work, as demoed in [this YouTube Video](https://www.youtube.com/watch?v=K6dAquqj5E4). I figured it would save time to create a template for doing so.


## Usage

1. Clone this repo
2. Write your code in `lib.rs::do_stuff()`. 
3. `cargo build --target x86_64-pc-windows-gnu`
4. Enjoy your DLL!
