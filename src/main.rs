#![feature(decl_macro)]

#[macro_use]
extern crate num_derive;

pub mod creature;
pub mod processor;
pub mod world;

use crate::creature::{Creature, CreatureMemory};
use enum_iterator::IntoEnumIterator;
use processor::Instruction;
use specs::{Builder, World, WorldExt};
use std::convert::TryInto;
use world::{CreatureDir, CreaturePos};

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

    // Initialize the world with the creature components
    let mut world: World = world::create_world![CreaturePos, CreatureDir, CreatureMemory, Creature];

    // Test entity
    world
        .create_entity()
        .with(CreaturePos::default())
        .with(CreatureDir::default())
        .with(CreatureMemory::default())
        .with({
            let mut c: Creature = creature::create_creature![Move, RotateCW, Goto];
            c.get_instructions_mut().push(0b000000000);
            c
        })
        .build();

    // Execute a test cycle
    world::run_tick_system(&world);

    // Propagate updates made during execution
    world.maintain();
}
