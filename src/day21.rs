use std::cmp::Reverse;
use std::iter::once;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl FromStr for Player {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let hp = lines.next().unwrap().strip_prefix("Hit Points: ").unwrap();
        let damage = lines.next().unwrap().strip_prefix("Damage: ").unwrap();
        let armor = lines.next().unwrap().strip_prefix("Armor: ").unwrap();
        let hp = hp.parse().unwrap();
        let damage = damage.parse().unwrap();
        let armor = armor.parse().unwrap();
        Ok(Player { hp, damage, armor })
    }
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Player {
    input.parse().unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Item {
    name: &'static str,
    cost: u32,
    damage: i32,
    armor: i32,
}

impl Item {
    const fn new(name: &'static str, cost: u32, damage: i32, armor: i32) -> Self {
        Self {
            name,
            cost,
            damage,
            armor,
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item::new("Dagger", 8, 4, 0),
    Item::new("Shortsword", 10, 5, 0),
    Item::new("Warhammer", 25, 6, 0),
    Item::new("Longsword", 40, 7, 0),
    Item::new("Greataxe", 74, 8, 0),
];

const ARMOR: [Item; 5] = [
    Item::new("Leather", 13, 0, 1),
    Item::new("Chainmail", 31, 0, 2),
    Item::new("Splintmail", 53, 0, 3),
    Item::new("Bandedmail", 75, 0, 4),
    Item::new("Platemail", 102, 0, 5),
];

const RINGS: [Item; 6] = [
    Item::new("Damage +1", 25, 1, 0),
    Item::new("Damage +2", 50, 2, 0),
    Item::new("Damage +3", 100, 3, 0),
    Item::new("Defense +1", 20, 0, 1),
    Item::new("Defense +2", 40, 0, 2),
    Item::new("Defense +3", 80, 0, 3),
];

fn loadouts() -> impl Iterator<Item = Vec<Item>> {
    weapons()
        .cartesian_product(armor())
        .cartesian_product(rings())
        .map(|((weapon, armor), rings)| {
            weapon
                .into_iter()
                .chain(armor)
                .chain(IntoIterator::into_iter(rings).flatten())
                .collect::<Vec<_>>()
        })
}

fn weapons() -> impl Iterator<Item = Option<Item>> + Clone {
    WEAPONS.iter().cloned().map(Some)
}

fn armor() -> impl Iterator<Item = Option<Item>> + Clone {
    once(None)
        .into_iter()
        .chain(ARMOR.iter().cloned().map(Some))
}

fn rings() -> impl Iterator<Item = [Option<Item>; 2]> + Clone {
    let no_rings = once([None, None]);
    let one_ring = RINGS.iter().cloned().map(|ring| [Some(ring), None]);
    let two_rings = RINGS
        .iter()
        .cloned()
        .cartesian_product(RINGS.iter().cloned())
        .filter(|(ring1, ring2)| ring1 != ring2)
        .map(|(ring1, ring2)| [Some(ring1), Some(ring2)]);
    no_rings.chain(one_ring).chain(two_rings)
}

impl Player {
    fn attack(&self, other: &mut Player) {
        other.hp -= (self.damage - other.armor).max(1);
    }

    fn equip(&mut self, item: Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }
}

fn player_wins(mut player: Player, mut boss: Player) -> bool {
    loop {
        player.attack(&mut boss);
        if boss.hp <= 0 {
            return true;
        }
        boss.attack(&mut player);
        if player.hp <= 0 {
            return false;
        }
    }
}

fn loadout_cost(loadout: &[Item]) -> u32 {
    loadout.iter().map(|item| item.cost).sum()
}

#[aoc(day21, part1)]
pub fn part1(boss: &Player) -> u32 {
    let player = Player {
        hp: 100,
        damage: 0,
        armor: 0,
    };
    // Try all possible loadouts, in incrementing total cost
    let mut loadouts = loadouts().collect::<Vec<_>>();
    loadouts.sort_unstable_by_key(|loadout| loadout_cost(loadout));
    for loadout in loadouts {
        // Apply item buffs
        let mut player = player.clone();
        let cost = loadout_cost(&loadout);
        for item in loadout {
            player.equip(item);
        }
        // See if we can win
        if player_wins(player, boss.clone()) {
            return cost;
        }
    }
    panic!("cannot win")
}

#[aoc(day21, part2)]
pub fn part2(boss: &Player) -> u32 {
    let player = Player {
        hp: 100,
        damage: 0,
        armor: 0,
    };
    // Try all possible loadouts, in decrementing total cost
    let mut loadouts = loadouts().collect::<Vec<_>>();
    loadouts.sort_unstable_by_key(|loadout| Reverse(loadout_cost(loadout)));
    for loadout in loadouts {
        // Apply item buffs
        let mut player = player.clone();
        let cost = loadout_cost(&loadout);
        for item in loadout {
            player.equip(item);
        }
        // See if we can lose
        if !player_wins(player, boss.clone()) {
            return cost;
        }
    }
    panic!("cannot lose")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let player = Player {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Player {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        assert!(player_wins(player, boss));
    }
}
