use super::{
    items::{Item, Armour},
    entity::{Entity, EntityObject, AttackResult},
    engine::Event,
};


use alloc::{string::String, vec::Vec, vec};

use crate::std::random;

pub struct Player {
    pub username: String,
    pub health_points: f64,
    pub max_health_points: f64,
    pub base_attack_damage: f64,
    pub speed: f64,

    pub inventory: [ Item ; 15 ],
    pub equipped: [ Item ; 7 ], // helmet, chestplate, leggings, boots, mainhand, offhand, charm
}
impl Player {
    pub fn new(username: String) -> Self {
        Self {
            username,
            health_points: 100.0,
            max_health_points: 100.0,
            base_attack_damage: 10.0,
            speed: 100.0,
            inventory: [ Item::Armour(Armour {}); 15 ],
            equipped: [ Item::Armour(Armour {}); 7 ],
        }
    }
}
impl core::fmt::Display for Player {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}/{}", self.username, self.health_points, self.max_health_points)
    }
}



impl Entity for Player {
    fn attack_entity(&mut self, target: &mut EntityObject) -> (AttackResult, Option<Vec<Event>>) {
        let mut entity = if let EntityObject::Enemy(enemy) = target {
            enemy
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
            return (attack, Some(vec![Event::EntityKilled(entity.clone())]));
        } else {
            return (attack, None)
        }
    }
}