

use std::io::*;
use std::fs;

use book_module::{Recipe, Book};


mod book_module;


fn display_menu() {
    print!("\n");
    print!("Recipe Book Menu:\n");
    print!("1. Add Recipe\n");
    print!("2. View Recipes\n");
    print!("3. Search Recipe\n");
    print!("4. Exit\n");    
}

fn process_input(input: &str, recipe_book: &mut Book) -> () {
    match input {
        "1" => add_recipe(),
        "2" => view_recipes(recipe_book),
        "3" => search_recipe(recipe_book),
        "4" => (),
        _ => println!("Invalid input"),
    }
}

fn search_recipe(recipe_book: &mut Book) {
    recipe_book.search_menu();
}


fn edit_ingredient(recipe: &mut Recipe) {
    if recipe.get_ingredients().len() == 0 {
        println!("\nThere are no ingredients in the recipe to edit.");
        println!("Please add an ingredient first.\n");
        return;
    }
    recipe.display_ingredients_with_index();
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
    println!("\nUpdated ingredient {} to {}\n", recipe.get_ingredients()[index - 1], new_ingredient);
    recipe.update_ingredient(index, new_ingredient);

}

fn remove_ingredient(recipe: &mut Recipe) {
    if recipe.get_ingredients().len() == 0 {
        println!("\nThere are no ingredients in the recipe to remove.");
        println!("Please add an ingredient first.\n");
        return;
    }
    recipe.display_ingredients_with_index();
    println!("Enter the index of the ingredient you want to remove:");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let index: usize = input.trim().parse().unwrap();
    recipe.remove_ingredient(index);
}

fn add_ingredient(recipe: &mut Recipe) {
    let mut input = String::new();
    let mut is_done: bool;
    is_done = false;
    while is_done == false {
        println!("Enter ingredients for {} (or type 'd' to finish)", recipe.get_name());
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
            recipe.add_ingredient(ingredient);
            println!("Current ingredients in the recipe:");
            recipe.display_ingredients_with_index();
        }
    }
    println!("Ingredients added successfully\n");

}

fn ingredients_recipe(mut recipe: &mut Recipe) {
    let mut input = String::new();
    let mut is_done: bool;
    is_done = false;
    while is_done == false {
        recipe.display_ingredients_with_index();
        println!("Do you want to:");
        println!("1. Edit an ingredient");
        println!("2. Remove an ingredient");
        println!("3. Add ingredients");
        println!("4. Continue");
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let choice: &str = input.trim();
        match choice {
            "1" => edit_ingredient(&mut recipe),
            "2" => remove_ingredient(&mut recipe),
            "3" => add_ingredient(&mut recipe),
            "4" => is_done = true,
            _ => println!("Invalid input: {}", choice)
        }
    }
}

fn author_recipe(recipe: &mut Recipe) {
    let mut input = String::new();
    println!("Enter the author name:");
    stdin().read_line(&mut input).unwrap();
    let author = input.trim().to_string();
    recipe.add_author(Some(author));
}

fn instructions_recipe(recipe: &mut Recipe) {
    let mut input = String::new();
    println!("Enter the instructions for the recipe:");
    stdin().read_line(&mut input).unwrap();
    let instructions = input.trim().to_string();
    recipe.edit_instruction(instructions);
    println!("\nInstructions added successfully\n");
    println!("Instructions: \n{}", recipe.get_instructions());
}

fn name_recipe(recipe: &mut Recipe) {
    let mut input = String::new();
    println!("Enter the recipe name:");
    stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();
    recipe.set_name(name);
    println!("\nRecipe name updated successfully\n");
    println!("Recipe name: {}\n", recipe.get_name());
}

fn save_recipe(recipe: &Recipe) {
    recipe.save_recipe();
}

fn add_recipe() {
    println!("Recipe Name:");
    let mut input = String::new();
    let _ = stdin().read_line(&mut input).unwrap();
    let recipe_name = input.trim().to_string();
    let mut recipe =Recipe::new(recipe_name);
    println!("Recipe added: {}", recipe.get_name());

    ingredients_recipe(&mut recipe);
    instructions_recipe(&mut recipe);

    let mut is_done: bool;
    is_done = false;
    while is_done == false {
        println!("\nDo you want to:");
        println!("1. Edit the recipe name");
        println!("2. Edit the ingredients");
        println!("3. Edit the instructions");
        println!("4. Add/Edit the author");
        println!("5. Exit");
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let choice: &str = input.trim();
        match choice {
            "1" => name_recipe(&mut recipe),
            "2" => ingredients_recipe(&mut recipe),
            "3" => instructions_recipe(&mut recipe),
            "4" => author_recipe(&mut recipe),
            "5" => {
                save_recipe(&recipe);
                println!("Recipe saved successfully\n");
                is_done = true;

            }
            _ => println!("Invalid input: {}", choice)
        }}

}

fn view_recipes(recipe_book: &mut Book) {
    recipe_book.menu_view_recipe();
}



fn main() {

    // Create a directory to store the recipes
    let _ = fs::create_dir("recipe_book");
    let mut recipe_book = Book::new();
    let mut input = String::new();
    while input.trim() != "4"  {
        recipe_book.create_recipe_book();
        input.clear();
        display_menu();
        stdin().read_line(&mut input).unwrap();
        println!();
        process_input(&input.trim(), &mut recipe_book);
    }


    

}
