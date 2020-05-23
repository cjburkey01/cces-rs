use crate::creature::Creature;
use specs::prelude::*;
use specs::{Component, SystemData};

/// Represents the position of a creature within the world.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Component)]
#[storage(VecStorage)]
struct CreaturePos {
    x: u64,
    y: u64,
}

/// Represents a possible direction the creature might be facing.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Component)]
#[storage(VecStorage)]
enum CreatureDir {
    /// Facing towards the positive y direction
    North,

    /// Facing towards the negative y direction
    South,

    /// Facing towards the positive x direction
    East,

    /// Facing towards the negative x direction
    West,
}

/// Represents a data structure that contains all the information needed to perform a tick on a
/// creature.
#[allow(dead_code)]
#[derive(SystemData)]
struct CreatureTickData<'a> {
    creature_pos: WriteStorage<'a, CreaturePos>,
    creature_dir: WriteStorage<'a, CreatureDir>,
    creature: WriteStorage<'a, Creature>,
}

/// The system responsible for ticking each creature. This is essential the brain tick stage of the
/// simulation.
struct CreatureTickSystem;

impl<'a> System<'a> for CreatureTickSystem {
    type SystemData = CreatureTickData<'a>;

    fn run(&mut self, _data: Self::SystemData) {
        unimplemented!()
    }
}
