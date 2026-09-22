use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Item {
    Wood,
    Stone,
    Coal,
    Iron,
    Stick,
    StoneShovel,
}

impl Item {
    fn name(&self) -> String {
        match self {
            Item::Wood => "Wood".to_string(),
            Item::Stone => "Stone".to_string(),
            Item::Coal => "Coal".to_string(),
            Item::Iron => "Iron".to_string(),
            Item::Stick => "Stick".to_string(),
            Item::StoneShovel => "StoneShovel".to_string(),
        }
    }
    fn from_name(name: &str) -> Option<Item> {
        match name {
            "Wood" => Some(Item::Wood),
            "Stone" => Some(Item::Stone),
            "Coal" => Some(Item::Coal),
            "Iron" => Some(Item::Iron),
            "Stick" => Some(Item::Stick),
            "StoneShovel" => Some(Item::StoneShovel),
            _ => None,
        }
    }
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    fn count(&self, name: &str) -> usize {
        let mut count = 0;
        for item in &self.items {
            if item.name() == *name {
                count += 1
            }
        }
        count
    }

    fn take(&mut self, name: &str) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|i| i.name() == name) {
            return Some(self.items.remove(pos));
        }
        None
    }

    fn contains(&self, name: &str) -> bool {
        match Item::from_name(name) {
            Some(item) => self.items.contains(&item),
            None => false,
        }
    }

    fn print_contents(&self) {
        for item in &self.items {
            println!("{}", item.name())
        }
    }
}

struct Recipe {
    name: String,
    inputs: HashMap<Item, u32>,
    output: Item,
}

impl Recipe {
    fn new(name: &str, inputs: HashMap<Item, u32>, output: Item) -> Recipe {
        Recipe {
            name: name.to_string(),
            inputs,
            output,
        }
    }

    fn can_craft(&self, inv: &Inventory) -> bool {
        let mut inv_hash = HashMap::new();
        for item in &inv.items {
            if self.inputs.contains_key(item) {
                *inv_hash.entry(item).or_insert(0) += 1;
            }
        }
        for (item, count) in &self.inputs {
            if inv_hash.get(item).copied().unwrap_or(0) < *count {
                return false;
            }
        }
        true
    }
}

fn craft(recipe: &Recipe, inv: &mut Inventory) -> Result<(), String> {
    if recipe.can_craft(inv) {
        for (item, count) in &recipe.inputs {
            for _ in 0..*count {
                let _ = inv.take(&item.name()).unwrap();
            }
        }
        inv.add(recipe.output);
        return Ok(());
    }
    Err("not enough resources in inventory to craft".to_string())
}
fn main() {
    let mut inv = Inventory::new();
    inv.add(Item::Wood);
    inv.add(Item::Wood);
    inv.add(Item::Stick);
    inv.add(Item::Stick);
    inv.add(Item::Stone);

    let mut recipe_hash = HashMap::new();
    recipe_hash.insert(Item::Stick, 2);
    recipe_hash.insert(Item::Stone, 1);
    let recipe = Recipe::new("StoneShovel", recipe_hash, Item::StoneShovel);

    println!("Before crafting: ");
    inv.print_contents();
    println!("Can craft shovel: {:?}", recipe.can_craft(&inv));
    match craft(&recipe, &mut inv) {
        Ok(_) => println!("Craft succeeded!"),
        Err(e) => println!("Craft failed: {e}"),
    }
    println!("After crafting: ");
    inv.print_contents();
}
