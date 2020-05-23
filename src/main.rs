#[macro_use]
extern crate num_derive;

pub mod creature;
pub mod processor;
pub mod world;

use enum_iterator::IntoEnumIterator;
use processor::Instruction;
use std::convert::TryInto;

fn main() {
    // Print out all of the instructions possible for debug purposes
    println!("Instructions: {{");
    for instruction in creature::Instruction::into_enum_iter() {
        println!(
            "  [0b{0:06b}=0x{0:02X}] [{1}] {2:?}",
            (instruction.try_into().unwrap_or(0b00000000) >> 2),
            instruction.get_args(),
            instruction,
        );
    }
    println!("}}");
}
