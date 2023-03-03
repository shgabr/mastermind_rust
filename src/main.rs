// This is the main entry point of the program

mod game_rules;
use game_rules::GameRules;
use game_rules::TypeOfChoices;

mod checker;
use checker::Checker;

mod guesser;
use guesser::Guesser;

use std::time::{SystemTime, UNIX_EPOCH};

fn get_input_game () -> (i32, i32, i32, i32, TypeOfChoices){

    fn get_input () -> i32 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse().expect("Please type a number!")
    }
    
    let no_of_trials: i32;
    let pattern_size: i32;
    let hints_difficulty: i32;
    let no_of_options: i32;
    let type_of_choices: TypeOfChoices;

    println!("Enter the number of trials: ");
    no_of_trials = get_input ();

    println!("Enter the pattern size: ");
    pattern_size = get_input ();

    println!("Enter the hints difficulty (0: easy | 1: medium | 2: hard): ");
    hints_difficulty = get_input ();

    println!("Enter the number of options: ");
    no_of_options = get_input ();

    println!("Enter the type of choices: (0: Colors, 1: Numbers)");
    type_of_choices = TypeOfChoices::new(get_input());

    return (no_of_trials, pattern_size, hints_difficulty, no_of_options, type_of_choices);

    // println!("Enter the type of choices: ");

}

fn create_game() -> GameRules {

    println!("******************************");

    let mut no_of_trials: i32 = 10;
    let mut pattern_size: i32 = 4;
    let mut hints_difficulty: i32 = 1;
    let mut no_of_options: i32 = 6;
    let type_of_choices: TypeOfChoices = TypeOfChoices::Colors;
    let mut game_rule: GameRules = GameRules::new(no_of_trials, pattern_size, hints_difficulty, no_of_options, type_of_choices);

    print!("DEFAULT GAME RULES:\n");
    game_rule.print();
    println!("******************************");


    println!("Do you want to change the default game rules? (y/n): ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if input == "y" {
        let type_of_choices: TypeOfChoices;
        (no_of_trials, pattern_size, hints_difficulty, no_of_options, type_of_choices) = get_input_game();
        // let type_of_choices: game_rules::TypeOfChoices = game_rules::TypeOfChoices::Colors;
        game_rule = GameRules::new(no_of_trials, pattern_size, hints_difficulty, no_of_options, type_of_choices);
    }
    println!("******************************");

    return game_rule;
}

fn play_game(game_rules: GameRules) {
 
    let mut arr = game_rules.choices.clone();
    let mut ptrn: Vec<String> = Vec::new();

    for i in 0..game_rules.pattern_size {
        let mut random_number: usize;
        loop {
            random_number = random_int(0, game_rules.no_of_choices-1, i) as usize;  
            if arr[random_number] != ""  {
                break;
            }
        }
        ptrn.push(arr[random_number].clone());
        arr[random_number] = "".to_string();
    }
    // println!("{:?}", ptrn);

    let mut checker = Checker::new(ptrn, game_rules.clone());

    let guesser = Guesser::new(game_rules.clone());

    while checker.current_trails <= game_rules.no_of_trials {
        
        println!("Current guess: {}/{}", checker.current_trails, game_rules.no_of_trials);

        let guess = guesser.predict();
        let response = checker.clone().check(guess);
        checker.current_trails += 1;

        println!("");

        if response == 1 {
            println!("You won!");
            break;
        } else if response == -1 {
            println!("You lost!");
            println!("The pattern was: {:?}", checker.get_pattern());
            break;
        }
    }

}

fn main() {
    println!("Welcome to Mastermind Game!\n");

    let game_rule: GameRules = create_game();

    println!("CURRENT GAME RULES:");
    game_rule.print();

    println!("******************************\n");

    println!("Game is starting...\n");

    play_game(game_rule);
}

fn random_int(min: i32, max: i32, seeder: i32) -> i32 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = now.as_secs() * 1_000 + u64::from(now.subsec_nanos()) * 1_000 + (seeder * 1_111)  as u64;
    let x = (seed % 2097152) as f64;
    let rand = ((x * 34359738337.0 + 12345.0) % 2097152.0) / 2097152.0;
    (rand * (max - min + 1) as f64).floor() as i32 + min
}