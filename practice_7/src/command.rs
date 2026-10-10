use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub enum Command {
    Go(Direction),
    Take(String),
    Drop(String),
    Use(String),
    Attack,
    Loot,
    Inventory,
    Quit,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "east" => Ok(Direction::East),
            "west" => Ok(Direction::West),
            _ => Err(format!("unrecognized direction {s}")),
        }
    }
}
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lo = s.to_lowercase();
        let arr: Vec<&str> = lo.split_whitespace().collect();
        match arr[0] {
            "go" => {
                if arr.len() < 2 {
                    return Err("command for 'go' must include direction".to_string());
                }
                let dir = Direction::from_str(arr[1]);
                match dir {
                    Ok(d) => Ok(Command::Go(d)),
                    Err(e) => Err(e),
                }
            }
            "take" => {
                if arr.len() < 2 {
                    return Err("command for 'take' must include item name".to_string());
                }
                Ok(Command::Take(arr[1].to_string()))
            }
            "drop" => {
                if arr.len() < 2 {
                    return Err("command for 'drop' must include item name".to_string());
                }
                Ok(Command::Drop(arr[1].to_string()))
            }
            "use" => {
                if arr.len() < 2 {
                    return Err("command for 'drop' must incude item name".to_string());
                }
                Ok(Command::Use(arr[1].to_string()))
            }
            "attack" => Ok(Command::Attack),
            "loot" => Ok(Command::Loot),
            "inventory" => Ok(Command::Inventory),
            "quit" => Ok(Command::Quit),
            e => Err(format!("{e} is not a recognized command")),
        }
    }
}
