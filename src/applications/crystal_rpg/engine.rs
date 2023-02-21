use super::entity::Enemy;
use alloc::vec::Vec;

pub enum Event {
    PlayerKilled,
    EntityKilled(Enemy),
}

pub enum Choice<A, B> {
    A(A),
    B(B),
}

impl core::fmt::Display for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Event::PlayerKilled => write!(f, "Player killed!"),
            Event::EntityKilled(x) => write!(f, "Entity killed! {}", x),
        }
    }
}

impl<A, B> core::fmt::Display for Choice<A, B> where 
    A: core::fmt::Display, 
    B: core::fmt::Display 
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Choice::A(a) => write!(f, "{}", a),
            Choice::B(b) => write!(f, "{}", b),
        }
    }
}

pub fn eventcheck<A>(e: (A, Option<Vec<Event>>)) -> Choice<A, Event> {
    match e.1 {
        Some(events) => {
            for event in events {
                match event {
                    Event::PlayerKilled => {
                        return Choice::B(event)
                    }
                    Event::EntityKilled(entity) => {
                        return Choice::B(event)
                    }
                }
            }
        },
        None => (),
    };

    Choice::A(e.0)
}