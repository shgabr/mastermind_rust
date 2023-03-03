use std::hash::Hash;

// use rand::seq::SliceRandom;

use crate::game_rules::GameRules;
use super::pattern::Pattern;
use super::hint::Hint;

#[derive(Clone)]
pub struct Checker <T> 
where
    T: Eq + Hash + Clone
{
    pattern: Pattern<T>,
    hint: Hint<T>,
    game_rules: GameRules,
    pub current_trails: i32,
}

impl <T> Checker <T> 
where
    T: Eq + Hash + Clone
{
    pub fn new (arr: Vec<T>, game_rules: GameRules) -> Self {
        let hint = Hint::new(game_rules.hints_difficulty);
        let pattern = Pattern::new(arr, true);

        Checker {
            pattern,
            hint,
            game_rules,
            current_trails: 1,
        }
    }
    pub fn check (mut self, arr: Vec<T>) -> i32 {
        let guess = Pattern::new(arr, false);
        if self.pattern.get_equality(&guess) {
            return 1;
        }
        if self.current_trails >= self.game_rules.no_of_trials {
            return -1;
        }
        self.hint.provide_hint(self.pattern, guess);
        0
    }
    pub fn get_pattern (&self) -> Vec<T> {
        self.pattern.pattern.clone()
    }
}
