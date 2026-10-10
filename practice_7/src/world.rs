use crate::command::Direction;
use crate::entity::Monster;
use crate::entity::Player;
use crate::item::Item;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RoomId(usize);

pub struct Room {
    name: String,
    description: String,
    exits: HashMap<Direction, RoomId>,
    items: Vec<Item>,
    monster: Option<Monster>,
}

pub struct World {
    rooms: Vec<Room>,
    player: Player,
    current: RoomId,
}

impl World {
    fn build() -> World;
    fn current_room(&self) -> Room;
    fn current_room_mut(&self) -> &mut Room;
    fn apply(&mut self, cmd: Command) -> String;
    fn is_won(&self) -> bool;
    fn is_lost(&self) -> bool;
}
