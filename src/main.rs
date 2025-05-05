extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::io::{self, Read};

fn main() {
    // Read all of stdin into a string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Initialize clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // Set clipboard contents
    ctx.set_contents(input).unwrap();
}
