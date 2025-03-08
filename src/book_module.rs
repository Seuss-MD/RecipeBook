

use std::fs::{self, read_dir};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::File;
use std::io::*;


// Define a struct (like a class in C++)
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipe {
    name: String,
    ingredients: Vec<String>,
    instructions: String,
    author: Option<String>
}

pub struct Book {
    recipes: Vec<Recipe>,
}

impl Book{
    pub fn new() -> Self {
        Book { recipes: Vec::new() }
    }

    pub fn new_recipe(&mut self, name: String) {
        let recipe = Recipe::new(name);
        self.recipes.push(recipe);
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    pub fn display_recipes(&self) {
        println!("Recipes in the book: ");
        for (index, recipe) in self.recipes.iter().enumerate() {
            println!("  {}. {}", index+1, recipe.get_name());
        }
        println!();
    }


    fn open_recipe(&mut self, index: usize) {
        let recipe = self.recipes.get(index-1).unwrap();
        recipe.display();
    }

    pub fn create_recipe_book(&mut self) {
        let folder = "recipe_book";
        for entry in read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_path = path.to_str().unwrap();
            let recipe = Book::parse_file_into_recipe(file_path);
            if !self.recipes.contains(&recipe) {
                
                self.add_recipe(recipe);
            }
        }
    }

    pub fn parse_file_into_recipe(file_path: &str) -> Recipe {
        let data = fs::read_to_string(file_path).unwrap(); // Read file contents
        let recipe: Recipe = serde_json::from_str(&data).unwrap(); // Deserialize JSON
        return recipe;
    }

    fn edit_recipe(&mut self, index: usize) {
        let recipe = self.recipes.remove(index-1);
        let mut recipe = recipe.copy();
        recipe.edit();
        self.add_recipe(recipe);
    }
    
    pub fn search_menu(&self) {
        let mut search_results = Book::new();

        println!("Search for Recipe");
        println!("\nYou can search by name, ingredients, instructions, author, or keyword.");
        println!("Do you want to search by:");
        println!("Search by Name of the recipe? y/n");
        let mut input_name = String::new();
        stdin().read_line(&mut input_name).unwrap();
        println!("Ingredients? y/n");
        let mut input_ingredients = String::new();
        stdin().read_line(&mut input_ingredients).unwrap();
        println!("Instructions? y/n");
        let mut input_instructions = String::new();
        stdin().read_line(&mut input_instructions).unwrap();
        println!("Author? y/n");
        let mut input_author = String::new();
        stdin().read_line(&mut input_author).unwrap();
        println!("Enter a keyword:");
        let mut input_keyword = String::new();
        stdin().read_line(&mut input_keyword).unwrap();
        input_keyword = input_keyword.trim().to_string();

        if input_name.trim() == "y" {


            let recipe = self.search_by_name(&input_keyword);
            if let Some(recipe) = recipe {
                if !search_results.recipes.contains(&recipe) {
                    search_results.add_recipe(recipe.copy());
                }
            }
        }

        if input_ingredients.trim() == "y" {

            let recipe = self.search_by_ingredient(&input_keyword);
            if let Some(recipe) = recipe {
                if search_results.recipes.contains(&recipe) {
                    search_results.add_recipe(recipe.copy());
                }
                }
            }
        

        if input_instructions.trim() == "y" {

            let recipe: Option<&Recipe> = self.search_by_instructions(&input_keyword);
            if let Some(recipe) = recipe {
                if search_results.recipes.contains(&recipe) {
                    search_results.add_recipe(recipe.copy());
                }            }
        }

        if input_author.trim() == "y" {
            println!("ran");

            let recipe = self.search_by_author(&input_keyword);
            if let Some(recipe) = recipe {
                if search_results.recipes.contains(&recipe) {
                    search_results.add_recipe(recipe.copy());
                }            }
        }
        if search_results.is_empty() {

            println!("No results found.");
        }
        else {
            search_results.display_recipes();

        }
    }

    fn is_empty(&self) -> bool {
        self.recipes.is_empty()
    }

    fn search_by_name(&self, keyword: &str) -> Option<&Recipe> {
        println!("Searching for recipe with word: {}", keyword);
        for recipe in &self.recipes {
            if recipe.get_name().contains(keyword) {
                return Some(recipe);
            }
        }
        None

    }

    fn search_by_instructions(&self, keyword: &str) -> Option<&Recipe> {
        println!("Searching for recipe with word: {}", keyword);
        for recipe in &self.recipes {
            if recipe.get_instructions().contains(keyword) {
                return Some(recipe);
            }
        }
        None
    }

    fn search_by_author(&self, keyword: &str) -> Option<&Recipe> {
        println!("Searching for recipe with word: {}", keyword);
        for recipe in &self.recipes {
            if recipe.get_author().map_or(false, |author| author.contains(keyword)) {
                return Some(recipe);
            }
        }
        None
    } 

    fn search_by_ingredient(&self, keyword: &str) -> Option<&Recipe> {
        println!("Searching for recipe with word: {}", keyword);
        for recipe in &self.recipes {
            for ingredient in recipe.get_ingredients() {
                if ingredient.contains(keyword) {
                    return Some(recipe);
                }
            }
        }
        None
    }

    fn delete_recipe(&mut self, index: usize) {
        self.recipes.remove(index-1);
    }
    

    pub fn menu_view_recipe(&mut self) -> Option<usize> {
        loop {
            self.display_recipes();
            println!("Select an option:");
            println!("1. Open recipe");
            println!("2. Edit recipe");
            println!("3. Delete recipe");
            println!("4. Exit");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => {
                    self.display_recipes();
                    println!("Enter the recipe number to open:");
                    let mut uinput = String::new();
                    stdin().read_line(&mut uinput).unwrap();
                    self.open_recipe(uinput.trim().parse().unwrap());
                }
                "2" => {
                    self.display_recipes();
                    println!("Enter the recipe number to edit:");
                    let mut uinput = String::new();
                    stdin().read_line(&mut uinput).unwrap();
                    self.edit_recipe(uinput.trim().parse().unwrap());
                }
                "3" => {
                    self.display_recipes();
                    println!("Enter the recipe number to delete:");
                    let mut uinput = String::new();
                    stdin().read_line(&mut uinput).unwrap();
                    self.delete_recipe(uinput.trim().parse().unwrap());
                }
                "4" => {
                    return Some(input.trim().parse().unwrap());
                }
                _ => println!("Invalid option, please try again."),
            }
    }
}
}




// Define methods for the struct using `impl`
impl Recipe {
    // Constructor-like function to create a new Person

    
    pub fn new(name: String) -> Self {
        Recipe { name, ingredients: Vec::new(), instructions: String::new(), author: None }
    }

    pub fn copy(&self) -> Self {
        Recipe { name: self.name.clone(), ingredients: self.ingredients.clone(), instructions: self.instructions.clone(), author: self.author.clone() }
    }

    // Method that acts like a member function in a class
    pub fn add_ingredient(&mut self, ingredient: String) {
        self.ingredients.push(ingredient.clone());
        println!("Ingredient {} added successfully", ingredient);
    }

    pub fn display(&self) {
        println!("Recipe Name: {}", self.name);
        println!("Author: {}\n", self.author.as_deref().unwrap_or("Unknown"));
        self.display_ingredients_with_index();
        println!();
        println!("Instructions: {}", self.instructions);
        println!();
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

    pub fn edit(&mut self) {
        let mut input = String::new();
        loop {
            println!("Editing recipe: {}", self.name);
            println!("1. Edit name");
            println!("2. Edit ingredients");
            println!("3. Edit instructions");
            println!("4. Edit author");
            println!("5. Save and exit");
            stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => {
                    println!("Enter new name:");
                    input.clear();
                    stdin().read_line(&mut input).unwrap();
                    self.name = input.trim().to_string();
                }
                "2" => {
                    
                    self.ingredients_recipe_menu();
                }
                "3" => {
                    println!("Enter new instructions:");
                    input.clear();
                    stdin().read_line(&mut input).unwrap();
                    self.instructions = input.trim().to_string();
                }
                "4" => {
                    println!("Enter new author:");
                    input.clear();
                    stdin().read_line(&mut input).unwrap();
                    self.author = Some(input.trim().to_string());
                }
                "5" => {
                    self.save_recipe();
                    break;
                }
                _ => println!("Invalid option, please try again."),
            }
            input.clear();
        }
    }

    fn ingredients_recipe_menu(&mut self) {
        let mut input = String::new();
        let mut is_done: bool;
        is_done = false;
        while is_done == false {
            self.display_ingredients_with_index();
            println!("Do you want to:");
            println!("1. Edit an ingredient");
            println!("2. Remove an ingredient");
            println!("3. Add ingredients");
            println!("4. Continue");
            input.clear();
            stdin().read_line(&mut input).unwrap();
            let choice: &str = input.trim();
            match choice {
                "1" => self.edit_ingredient(),
                "2" => self.remove_ingredient_ui(),
                "3" => self.add_ingredient_ui(),
                "4" => is_done = true,
                _ => println!("Invalid input: {}", choice)
            }
        }
    }

    fn edit_ingredient(&mut self) {
        if self.ingredients.is_empty() {
            println!("\nThere are no ingredients in the recipe to edit.");
            println!("Please add an ingredient first.\n");
            return;
        }
        self.display_ingredients_with_index();
        println!("Enter the number for the ingredient you want to edit:");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().unwrap();
        let mut new_ingredient = String::new();
        while new_ingredient.is_empty() == true {
            println!("Enter the new ingredient:");
            stdin().read_line(&mut new_ingredient).unwrap();
            new_ingredient = new_ingredient.trim().to_string();
            if new_ingredient.is_empty() {
                println!("Please enter a valid ingredient");
                println!()
            }
        }
        println!("\nUpdated ingredient {} to {}\n", &self.ingredients[index - 1], new_ingredient);
        self.update_ingredient(index, new_ingredient);
    
    }

    fn remove_ingredient_ui(&mut self) {
        if self.ingredients.is_empty() {
            println!("\nThere are no ingredients in the recipe to remove.");
            println!("Please add an ingredient first.\n");
            return;
        }
        self.display_ingredients_with_index();
        println!("Enter the index of the ingredient you want to remove:");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().unwrap();
        self.remove_ingredient(index);
    }

    fn add_ingredient_ui(&mut self) {
        let mut input = String::new();
        let mut is_done: bool;
        is_done = false;
        while is_done == false {
            println!("Enter ingredients for {} (or type 'd' to finish)", self.name);
            input.clear();
            stdin().read_line(&mut input).unwrap();
            println!();
    
            let ingredient: String = input.trim().to_owned();
            if ingredient.trim() == "d" {
                is_done = true;
            }
            else if ingredient.is_empty() {
                println!("Please enter a valid ingredient");
            }
            else {
                self.add_ingredient(ingredient);
                println!("Current ingredients in the recipe:");
                self.display_ingredients_with_index();
            }
        }
        println!("Ingredients added successfully\n");
    
    }

}



