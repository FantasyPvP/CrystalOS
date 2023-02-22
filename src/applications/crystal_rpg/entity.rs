use super::player::Player;
use super::engine::Event;

use alloc::{string::String, vec::Vec, vec};
use crate::std::random;


pub trait Entity {
    fn attack_entity(&mut self, _: &mut EntityObject) -> (AttackResult, Option<Vec<Event>>) {
        (AttackResult::Miss, None)
    }
}
pub enum EntityObject<'a> {
    Player(&'a mut Player),
    Enemy(&'a mut Enemy),
}

#[derive(Debug, Clone, Copy)]
pub enum AttackResult {
    Miss,
    GlancingBlow(f64),
    Hit(f64),
    CriticalHit(f64),
    FriendlyFire,
}

impl core::fmt::Display for AttackResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AttackResult::Miss => write!(f, "Missed!"),
            AttackResult::GlancingBlow(damage) => write!(f, "Glancing Blow: {}", damage),
            AttackResult::Hit(damage) => write!(f, "Hit: {}", damage),
            AttackResult::CriticalHit(damage) => write!(f, "Critical Hit: {}", damage),
            AttackResult::FriendlyFire => write!(f, "Friendly Fire (no damage dealt)!"),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub health_points: f64,
    pub max_health_points: f64,
    pub base_attack_damage: f64,
    pub speed: f64,
}
impl Enemy {
    pub fn new() -> Self {
        Self {
            health_points: 200.0,
            max_health_points: 200.0,
            base_attack_damage: 5.0,
            speed: 100.0,
        }
    }
}
impl core::fmt::Display for Enemy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Enemy: {}/{}", self.health_points, self.max_health_points)
    }
}

impl Entity for Enemy {
    fn attack_entity(&mut self, target: &mut EntityObject) -> (AttackResult, Option<Vec<Event>>) {
        let mut entity = if let EntityObject::Player(player) = target {
            player
        } else {
            return (AttackResult::FriendlyFire, None);
        };

        // combat implementation

        let dmg: f64;

        let r = random::Random::int(0, 125) as f64;
        let rs = self.speed / entity.speed * 100 as f64;

        let attack = if r < rs * 0.2 {
            dmg = self.base_attack_damage * 1.5;
            entity.health_points -= dmg;
            AttackResult::CriticalHit(dmg)

        } else if r < rs * 0.8 {
            dmg = self.base_attack_damage;
            entity.health_points -= dmg;
            AttackResult::Hit(dmg)

        } else if r < rs {
            dmg = self.base_attack_damage * 0.5;
            entity.health_points -= dmg;
            AttackResult::GlancingBlow(dmg)
        } else {
            AttackResult::Miss
        };

        if entity.health_points <= 0.0 {
            return (attack, Some(vec![Event::PlayerKilled]));
        } else {
            return (attack, None)
        }
    }
}