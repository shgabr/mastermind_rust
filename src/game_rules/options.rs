use std::cmp::min;

pub static OPTIONS: Options = Options {
    colors: ["R", "B", "G", "Y", "O", "P", "V", "C", "M", "K"],
    numbers: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
};


#[derive(Clone, Debug)]
pub enum TypeOfChoices {
    Colors,
    Numbers
}

impl TypeOfChoices {
    pub fn new (type_of_choices: i32) -> Self {
        match type_of_choices {
            0 => TypeOfChoices::Colors,
            1 => TypeOfChoices::Numbers,
            _ => TypeOfChoices::Colors,
        }
    }
}

pub struct Options <'a> {
    colors: [&'a str; 10],    
    numbers: [&'a str; 10],
}

impl Options <'_> {
    pub fn get_choices(&self, type_of_choices: TypeOfChoices, no_of_options: i32) -> Vec<String> {
        let mut choices: Vec<String> = Vec::new();
        match type_of_choices {
            TypeOfChoices::Colors => {
                let sz = min(self.colors.len(), no_of_options as usize);
                for i in 0..sz {
                    choices.push(self.colors[i].to_string());
                }
            },
            TypeOfChoices::Numbers => {
                let sz = min(self.numbers.len(), no_of_options as usize);
                for i in 0..sz {
                    choices.push(self.numbers[i].to_string());
                }
            }
        }
        return choices;
    }
}