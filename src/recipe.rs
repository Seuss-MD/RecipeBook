

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::File;
use std::io::Write;


// Define a struct (like a class in C++)
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    name: String,
    ingredients: Vec<String>,
    instructions: String,
    author: Option<String>
}

// Define methods for the struct using `impl`
impl Recipe {
    // Constructor-like function to create a new Person

    
    pub fn new(name: String) -> Self {
        Recipe { name, ingredients: Vec::new(), instructions: String::new(), author: None }
    }

    // Method that acts like a member function in a class
    pub fn add_ingredient(&mut self, ingredient: String) {
        self.ingredients.push(ingredient.clone());
        println!("Ingredient {} added successfully", ingredient);
    }

    

    pub fn remove_ingredient(&mut self, mut index: usize) {
        index = index - 1;
        if index > self.ingredients.len() {
            println!("Invalid index");
            return;
        }
        self.ingredients.remove(index);
    }

    pub fn update_ingredient(&mut self, index: usize, new_ingredient: String) {
        self.ingredients[index - 1] = new_ingredient;
    }
    
    
    pub fn edit_instruction(&mut self, instruction: String) {
        self.instructions = instruction;
    }
    
    pub fn add_author(&mut self, author:Option<String>) {
        self.author = author;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    pub fn get_instructions(&self) -> &str {
        &self.instructions
    }

    pub fn get_ingredients(&self) -> &Vec<String> {
        &self.ingredients
    }

    // pub fn display_ingredients(&self) {
    //     for item in &self.ingredients{
    //         println!("{}", item);
    //     }
    // }

    pub fn display_ingredients_with_index(&self) {
        for (index, item) in self.ingredients.iter().enumerate() {
            println!("       {}. {}", index+1, item);
        }
        println!();
    }
    pub fn get_author(&self) -> Option<&str> {
        self.author.as_deref()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_recipe(&self) {
        println!("Recipe Name: {}", self.name);
        println!("Author: {}\n", self.author.as_deref().unwrap_or("Unknown"));
        println!("Ingredients: ");
        for item in &self.ingredients {
            println!("       {}", item);
        }
        println!();
        println!("Instructions: {}", self.instructions);
        println!();
    }

    // Method to get the person's age

    pub fn save_recipe(&self) {
        let json_string = serde_json::to_string(&self).unwrap();
        let filename = format!("recipe_book/{}.json", self.name);
        let mut file = File::create(filename).expect("Failed to create file");
        let _ =file.write_all(json_string.as_bytes()).expect("Failed to write to file");

    }


}






