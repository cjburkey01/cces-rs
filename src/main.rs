#[macro_use]
extern crate num_derive;

pub mod instructions;

use enum_iterator::IntoEnumIterator;
use instructions::Instruction;
use std::convert::TryInto;

fn main() {
    println!("Instructions: {{");

    for instruction in Instruction::into_enum_iter() {
        println!(
            "  [{:04b}] [{}] {:?}",
            (instruction.try_into().unwrap_or(0b00000000) >> 4),
            instruction.get_args(),
            instruction,
        );
    }

    println!("}}");
}
