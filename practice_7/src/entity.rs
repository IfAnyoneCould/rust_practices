use crate::item::Item;
use rand::RngExt;

trait Combatant {
    fn name(&self) -> &str;
    fn hp(&self) -> i32;
    fn is_alive(&self) -> bool {
        self.hp() > 0
    }
    fn attack_roll(&self, rng: &mut impl RngExt) -> i32;
    fn take_damage(&mut self, amount: i32);
}

pub struct Player {
    hp: i32,
    max_hp: i32,
    inventory: Vec<Item>,
    weapon: Option<Item>,
    default_damage: (i32, i32),
}

pub struct Monster {
    name: String,
    hp: i32,
    damage: (i32, i32),
}

impl Combatant for Player {
    fn name(&self) -> &str {
        "player"
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn attack_roll(&self, rng: &mut impl RngExt) -> i32 {
        let damage = rng.random_range(self.default_damage.0..=self.default_damage.1);
        match self.weapon {
            Some(Item::Weapon { bonus, .. }) => damage + bonus,
            _ => damage,
        }
    }
    fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }
}
impl Combatant for Monster {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn attack_roll(&self, rng: &mut impl RngExt) -> i32 {
        rng.random_range(self.damage.0..=self.damage.1)
    }
    fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }
}

fn strike(
    attacker: &impl Combatant,
    defender: &mut impl Combatant,
    rng: &mut impl RngExt,
) -> String {
    let damage = attacker.attack_roll(rng);
    defender.take_damage(damage);
    if defender.is_alive() {
        return format!(
            "{} withstood {} damage from {}",
            defender.name(),
            damage,
            attacker.name()
        );
    }
    format!(
        "{} took {} damage from {} and perished!",
        defender.name(),
        damage,
        attacker.name()
    )
}
