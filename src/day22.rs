use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Boss {
    hp: u32,
    damage: u32,
}

impl FromStr for Boss {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let hp = lines.next().unwrap().strip_prefix("Hit Points: ").unwrap();
        let damage = lines.next().unwrap().strip_prefix("Damage: ").unwrap();
        let hp = hp.parse().unwrap();
        let damage = damage.parse().unwrap();
        Ok(Boss { hp, damage })
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Boss {
    input.parse().unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Player {
    hp: u32,
    mana: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const ALL: [Spell; 5] = [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ];

    fn cost(self) -> u32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn effect(self) -> Option<EffectKind> {
        match self {
            Spell::Shield => Some(EffectKind::Shield),
            Spell::Poison => Some(EffectKind::Poison),
            Spell::Recharge => Some(EffectKind::Recharge),
            _ => None,
        }
    }

    fn all() -> impl Iterator<Item = Spell> {
        IntoIterator::into_iter(Self::ALL)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Effect {
    kind: EffectKind,
    duration: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum EffectKind {
    Shield,
    Poison,
    Recharge,
}

impl EffectKind {
    fn duration(self) -> u32 {
        match self {
            EffectKind::Shield => 6,
            EffectKind::Poison => 6,
            EffectKind::Recharge => 5,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game {
    player: Player,
    boss: Boss,
    effects: Vec<Effect>,
    turn: u32,
    cost: u32,
}

impl Game {
    fn new(player: Player, boss: Boss) -> Self {
        Self {
            player,
            boss,
            effects: vec![],
            turn: 0,
            cost: 0,
        }
    }

    fn can_cast(&self, spell: Spell) -> bool {
        // You must have enough mana to cast a spell.
        if self.player.mana < spell.cost() {
            return false;
        }
        // You cannot cast a spell that would start an effect which is already active.
        if let Some(kind) = spell.effect() {
            if self.effects.iter().any(|effect| effect.kind == kind) {
                return false;
            }
        }
        true
    }

    fn cast(&mut self, spell: Spell) {
        debug_assert!(self.can_cast(spell));
        self.player.mana -= spell.cost();
        self.cost += spell.cost();
        match spell {
            Spell::MagicMissile => {
                self.damage_boss(4);
            }
            Spell::Drain => {
                self.damage_boss(2);
                self.heal_player(2);
            }
            spell => self.add_effect(spell.effect().unwrap()),
        }
    }

    fn damage_boss(&mut self, amount: u32) {
        self.boss.hp = self.boss.hp.saturating_sub(amount);
    }

    fn damage_player(&mut self, amount: u32) {
        let amount = amount.saturating_sub(self.player_armor()).max(1);
        self.player.hp = self.player.hp.saturating_sub(amount);
    }

    fn heal_player(&mut self, amount: u32) {
        self.player.hp += amount;
    }

    fn add_effect(&mut self, kind: EffectKind) {
        self.effects.push(Effect {
            kind,
            duration: kind.duration(),
        })
    }

    fn player_armor(&self) -> u32 {
        if self
            .effects
            .iter()
            .any(|effect| effect.kind == EffectKind::Shield)
        {
            7
        } else {
            0
        }
    }

    fn start_turn(&mut self) {
        for effect in self.effects.iter_mut() {
            match effect.kind {
                EffectKind::Shield => {
                    // do nothing
                }
                EffectKind::Poison => {
                    self.boss.hp = self.boss.hp.saturating_sub(3);
                }
                EffectKind::Recharge => {
                    self.player.mana += 101;
                }
            }
            effect.duration -= 1;
        }
        self.effects.retain(|effect| effect.duration > 0);
        self.turn += 1;
    }

    fn boss_attack(&mut self) {
        self.damage_player(self.boss.damage)
    }

    fn player_wins(&self) -> bool {
        self.boss.hp == 0
    }

    fn player_loses(&self) -> bool {
        self.player.hp == 0
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        other
            .cost
            .cmp(&self.cost)
            // Sort by lowest boss's HP.
            .then_with(|| other.boss.hp.cmp(&self.boss.hp))
            // Use other fields to distinguish.
            .then_with(|| self.player.cmp(&other.player))
            .then_with(|| self.boss.cmp(&other.boss))
            .then_with(|| self.turn.cmp(&other.turn))
            .then_with(|| self.effects.cmp(&other.effects))
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(game: Game, part2: bool) -> Option<Game> {
    let mut queue = BinaryHeap::<Game>::new();
    queue.push(game);
    while let Some(mut game) = queue.pop() {
        if game.turn > 0 {
            // Boss's turn
            game.start_turn();
            if game.player_wins() {
                return Some(game);
            }
            game.boss_attack();
            if game.player_loses() {
                continue;
            }
        }
        // Player's turn
        if part2 {
            // At the start of each player turn (before any other effects apply),
            // you lose 1 hit point. If this brings you to or below 0 hit points, you lose.
            game.damage_player(1);
            if game.player_loses() {
                continue;
            }
        }
        game.start_turn();
        if game.player_wins() {
            return Some(game);
        }
        for spell in Spell::all() {
            if !game.can_cast(spell) {
                continue;
            }
            let mut game = game.clone();
            game.cast(spell);
            // We don't yet evaluate the boss's turn here. Even if we can win (by poison damage),
            // there may still be a different game state with a lower cost or a cheaper attack
            // that can also defeat the boss.
            // Instead, we push the game back onto the queue and evaluate the boss's turn when
            // it pops back off the queue. Only at that point can we be *sure* that we've found
            // the winning game with the lowest possible cost.
            queue.push(game);
        }
    }
    None
}

#[aoc(day22, part1)]
pub fn part1(boss: &Boss) -> u32 {
    let player = Player { hp: 50, mana: 500 };
    let game = Game::new(player, boss.clone());
    let game = solve(game, false).expect("no solution found");
    game.cost
}

#[aoc(day22, part2)]
pub fn part2(boss: &Boss) -> u32 {
    let player = Player { hp: 50, mana: 500 };
    let game = Game::new(player, boss.clone());
    let game = solve(game, true).expect("no solution found");
    game.cost
}
