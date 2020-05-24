use crate::creature::{Creature, CreatureMemory};
use specs::prelude::*;
use specs::{Component, SystemData};

/// Creates an expression that will initialize the world with all of the provided component types.
/// Using the new declarative macros because they're cool and I like them :)
pub macro create_world($($component_type: ty),*) {{
    // Create the world
    let mut world: ::specs::World = ::specs::WorldExt::new();

    // Register components
    $( ::specs::WorldExt::register::<$component_type>(&mut world); )*

    // Return world
    world
}}

/// Represents the position of a creature within the world.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Component, Default)]
#[storage(VecStorage)]
pub struct CreaturePos {
    x: u64,
    y: u64,
}

/// Represents a possible direction the creature might be facing.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Component)]
#[storage(VecStorage)]
pub enum CreatureDir {
    /// Facing towards the positive y direction
    North,

    /// Facing towards the negative y direction
    South,

    /// Facing towards the positive x direction
    East,

    /// Facing towards the negative x direction
    West,
}

impl Default for CreatureDir {
    fn default() -> Self {
        Self::North
    }
}

/// Represents a data structure that contains all the information needed to perform a tick on a
/// creature.
#[allow(dead_code)]
#[derive(SystemData)]
struct CreatureTickData<'a> {
    creature_pos: WriteStorage<'a, CreaturePos>,
    creature_dir: WriteStorage<'a, CreatureDir>,
    creature_mem: WriteStorage<'a, CreatureMemory>,
    creature: WriteStorage<'a, Creature>,
}

/// The system responsible for ticking each creature. This is essentially the brain tick stage of
/// the simulation.
struct CreatureTickSystem;

impl<'a> System<'a> for CreatureTickSystem {
    type SystemData = CreatureTickData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for (pos, dir, mem, creature) in (
            &data.creature_pos,
            &data.creature_dir,
            &data.creature_mem,
            &data.creature,
        )
            .join()
        {
            println!("{:X?}\n{:#X?}\n{:#X?}\n{:#X?}", pos, dir, mem, creature);
        }
    }
}

pub fn run_tick_system(world: &World) {
    CreatureTickSystem.run_now(world)
}
