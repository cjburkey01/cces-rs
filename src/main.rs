#[macro_use]
extern crate num_derive;

pub mod creature;
pub mod processor;

use enum_iterator::IntoEnumIterator;
use processor::Instruction;
use std::convert::TryInto;

fn main() {
    println!("Instructions: {{");

    for instruction in creature::Instruction::into_enum_iter() {
        println!(
            "  [{:06b}] [{}] {:?}",
            (instruction.try_into().unwrap_or(0b00000000) >> 2),
            instruction.get_args(),
            instruction,
        );
    }

    println!("}}");
}
