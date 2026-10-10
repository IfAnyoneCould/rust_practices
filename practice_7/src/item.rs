use crate::world::RoomId;

pub enum Item {
    Weapon {name: String, bonus: i32},
    Potion {name: String, heal: i32},
    Key {name: String, opens: RoomId},
    Treasure {name: String, value: u32},
}
