/*
    GameRules:
        
        + NO of Trials,
        + Pattern Size,
        + Hints Difficulty, // 0 for easy, 1 for medium, 2 for hard
        + NO of Options, *
        + Type of Choices *

 */

// const COLORS_OPTIONS: [&str; 6] = ["red", "blue", "green", "yellow", "orange", "purple"];
// #[derive(Clone, Debug)]
// pub enum TypeOfChoices {
//     Colors,
// }

use super::options::OPTIONS;
use super::options::TypeOfChoices;


#[derive(Clone)]

pub struct GameRules {
    pub no_of_trials: i32,
    pub pattern_size: i32,
    pub hints_difficulty: i32,
    pub no_of_choices: i32,
    pub type_of_choices: TypeOfChoices,
    pub choices: Vec<String>,
}

impl GameRules {
    pub fn new(no_of_trials : i32, pattern_size: i32, hints_difficulty: i32, no_of_choices: i32, type_of_choices: TypeOfChoices) -> Self {
        
        let choices: Vec<String> = OPTIONS.get_choices(type_of_choices.clone(), no_of_choices);
        // choices.shuffle(&mut rand::thread_rng());

        GameRules {
            no_of_trials,
            pattern_size,
            hints_difficulty,
            no_of_choices: choices.len() as i32,
            type_of_choices,
            choices,
        }
    }
    pub fn print(&self) {
        println!("Number of Trials: {}", self.no_of_trials);
        println!("Pattern Size: {}", self.pattern_size);
        println!("Hints Difficulty: {}", self.hints_difficulty);
        println!("Number of Choices: {}", self.no_of_choices);
        println!("Type of Choices: {:?}", self.type_of_choices);
        println!("Choices: {:?}", self.choices);
    }
}