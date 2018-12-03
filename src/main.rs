#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]

extern crate gl;
extern crate image;
extern crate cgmath;
extern crate tobj;

mod common;
mod shader;
mod macros;
mod camera;
mod mesh;
mod model;
mod utils;

mod tree;
mod objects;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the `objects` or `tree`");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        "tree" => tree::main_tree(),
        "objects" => objects::main_objects(),
        _     => println!("Unknown id")
    }
}
