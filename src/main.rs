use std::fs;
use std::env;
use calc::basics::*;

fn main() {
    let mut argv = env::args();
    let argc = argv.len();

    if argc == 1{
        mainscreen::ui();
    }
}
