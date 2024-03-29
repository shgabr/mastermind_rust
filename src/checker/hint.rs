use std::hash::Hash;
use std::marker::PhantomData;

use super::pattern::Pattern;

#[derive(Clone)]

pub struct Hint <T> 
where 
    T: Eq + Hash + Clone
{
    difficulty: i32,
    right_guess: i32,
    wrong_place: i32,
    right_indices: Vec<i32>,
    phantom: PhantomData<T>
}

impl <T> Hint <T> 
where 
    T: Eq + Hash + Clone
    {
    pub fn new(difficulty: i32) -> Self {
        Hint {
            difficulty,
            right_guess: 0,
            wrong_place: 0,
            right_indices: Vec::new(),
            phantom: PhantomData
        }
    }
    pub fn provide_hint(&mut self, truth: Pattern<T>, guess: Pattern<T>) {
        let mut right_guess = 0;
        let mut wrong_place = 0;
        let mut right_indices = Vec::new();
        let mut elements_guessed: Vec<T> = Vec::new();  // to handle duplicate elements in guess 
        for i in 0..truth.size {
            if truth.pattern[i] == guess.pattern[i] {
                right_guess += 1;
                right_indices.push(i as i32);
                elements_guessed.push(truth.pattern[i].clone());
            }
        }
        for i in 0..truth.size {
            if !right_indices.contains(&(i as i32)) && !elements_guessed.contains(&guess.pattern[i]) {
                if truth.indices.contains_key(&guess.pattern[i]) {
                    elements_guessed.push(guess.pattern[i].clone());
                    wrong_place += 1;
                }
            }
        }
        self.right_guess = right_guess;
        self.wrong_place = wrong_place;
        self.right_indices = right_indices;
        self.print_hint();
    }
    fn print_hint (&self){
        match self.difficulty {
            0 => {
                println!("Right Guess: {}", self.right_guess);
                println!("Wrong Place: {}", self.wrong_place);
                println!("Right Indices: {:?}", self.right_indices);
            },
            1 => {
                println!("Right Guess: {}", self.right_guess);
                println!("Wrong Place: {}", self.wrong_place);
            },
            2 => {
                println!("Right Guess: {}", self.right_guess);
            },
            _ => {
                println!("Right Guess: {}", self.right_guess);
                println!("Wrong Place: {}", self.wrong_place);
            }
        }
    }
}

// fn main() {

//     let arr = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
//     let truth = Pattern::new(arr, true);

//     let arr2 = ["10", "2", "3", "4", "5", "6", "7", "8", "9", "1"];
//     let guess = Pattern::new(arr2, false);

//     let mut hint = Hint::new(1);

//     if !truth.get_equality(&guess) {
//         hint.provide_hint(truth, guess);
//     } else {
//         println!("You guessed it!");
//     }
// }




