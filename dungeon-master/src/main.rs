enum Class {
    Wizard,
    Rogue,
    Warlock,
    Bard,
    Druid,
    Barbarian,
}

enum Ability {
    Intelligence,
    Strength,
    Dexterity,
    Constitution,
    Charisma,
    Wisdom,
}

struct Character {
    class: Class,
    name: String,
    level: u32,
    proficiencies: Vec<Ability>,
}

fn main() {
    let class = Class::Wizard;
    let ability = Ability::Intelligence;

    println!("{:?}", ability_check(class, ability));
}

fn ability_check(class: Class, ability: Ability) -> bool {
    match (class, ability) {
        (Class::Wizard, Ability::Intelligence) => true,
        (Class::Rogue, Ability::Dexterity) => true,
        (Class::Warlock, Ability::Charisma) => true,
        (Class::Bard, Ability::Charisma) => true,
        (Class::Druid, Ability::Wisdom) => true,
        (Class::Barbarian, Ability::Strength) => true,
        _ => false,
    }
}

fn roll_d20() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=20)
}
