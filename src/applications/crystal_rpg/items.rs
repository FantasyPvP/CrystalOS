

#[derive(Debug, Clone, Copy)]
pub struct Armour;
#[derive(Debug, Clone, Copy)]
pub struct Weapon;
#[derive(Debug, Clone, Copy)]
pub struct Charm;
#[derive(Debug, Clone, Copy)]
pub struct OtherItem;


#[derive(Debug, Clone, Copy)]
pub enum Item {
    Armour(Armour),
    Weapon(Weapon),
    Charm(Charm),
    Other(OtherItem),
}
