use std::ffi::c_char;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Class {
    Wizard,
    Rogue,
    Warlock,
    Bard,
    Druid,
    Barbarian,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Class::Wizard => write!(f, "Wizard"),
            Class::Rogue => write!(f, "Rogue"),
            Class::Warlock => write!(f, "Warlock"),
            Class::Bard => write!(f, "Bard"),
            Class::Druid => write!(f, "Druid"),
            Class::Barbarian => write!(f, "Barbarian"),
        }
    }
}

#[derive(Debug)]
enum Skill {
    Persuasion(u8),
    Intimidation(u8),
    Arcana(u8),
    Deception(u8),
    History(u8),
}

enum Ability {
    Intelligence,
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Charisma,
}

#[derive(Debug)]
struct Character {
    class: Class,
    name: String,
    level: u8,
    exp_to_lvlup: u8,
    hit_points: u8,
    proficiencies: Vec<Skill>,
    intelligence: u8,
    strength: u8,
    dexterity: u8,
    constitution: u8,
    charisma: u8,
    wisdom: u8,
}

impl Character {
    fn new(name: String, class: Class) -> Self {
        Self {
            class,
            name,
            level: 1,
            exp_to_lvlup: 100,
            hit_points: 100,
            proficiencies: Vec::new(),
            intelligence: generate_ability_score(),
            strength: generate_ability_score(),
            dexterity: generate_ability_score(),
            constitution: generate_ability_score(),
            charisma: generate_ability_score(),
            wisdom: generate_ability_score(),
        }
    }
}

// Function to generate an ability score, rolls 4d6 and takes the total of the 3 highest results, with a max return value of 18
fn generate_ability_score() -> u8 {
    use rand::Rng;
    let d1: u8 = rand::thread_rng().gen_range(1..=6);
    let d2: u8 = rand::thread_rng().gen_range(1..=6);
    let d3: u8 = rand::thread_rng().gen_range(1..=6);
    let d4: u8 = rand::thread_rng().gen_range(1..=6);
    let mut rolled_dice = [d1, d2, d3, d4];
    rolled_dice.sort();

    let rolled_total: u8 = rolled_dice[1] + rolled_dice[2] + rolled_dice[3];

    if rolled_total <= 18 {
        rolled_total
    } else {
        18
    }
}

fn main() {
    let ability = Ability::Intelligence;

    let character = create_character();

    println!("{:?}", character);

    println!("{:?}", ability_check(character, ability));
}

fn ability_check(character: Character, ability: Ability) -> bool {
    match ability {
        Ability::Intelligence => character.intelligence >= 10,
        Ability::Dexterity => character.dexterity >= 10,
        Ability::Charisma => character.charisma >= 10,
        Ability::Wisdom => character.wisdom >= 10,
        Ability::Strength => character.strength >= 10,
        Ability::Constitution => character.constitution >= 10,
        _ => false,
    }
}

fn roll_d20(modifier: u8) -> u8 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=20) + modifier
}

fn create_character() -> Character {
    println!("Hi and Welcome to the charactere creator, what is your character's name?");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to understand you! Goodbye!");
    let name = name.trim();
    println!("What class are you? Wizard, Rogue, Warlock, Bard, Druid, or Barbarian?");
    let mut given_class = String::new();
    std::io::stdin()
        .read_line(&mut given_class)
        .expect("Failed to understand you! Goodbye!");
    let class: Class = get_class_from_string(&given_class);

    Character::new(name.to_string(), class)
}

fn fuzzy_match(string_1: &str, string_2: String) -> bool {
    if string_1.trim().to_lowercase() == string_2.trim().to_lowercase() {
        true
    } else {
        false
    }
}

fn get_class_from_string(submitted_class: &str) -> Class {
    for class in Class::iter() {
        if fuzzy_match(submitted_class, class.to_string()) {
            return class;
        }
    }
    return Class::Bard;
}

//TODO: Sort functions and structs/enums into library and create an intro scenario that uses the other support functions like roll_d20 and ability_check. Also add logic for gaining xp, leveling up and gaining proficiencies.
