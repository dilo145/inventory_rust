use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    name: String,
    quantity: u32,
}

const INVENTORY_FILE: &str = "inventory.json";

fn main() {
    let mut inventory = load_inventory();

    loop {
        println!("\n************ Menu ************ Inventory Management ************");
        println!("1. List all Products");
        println!("2. Add a Product");
        println!("3. Update a Product");
        println!("4. Remove a Product");
        println!("5. Exit");
        println!("***************************************************************");

        save_inventory(&inventory);

        let choice = read_input("Enter your choice: ");
        match choice.trim() {
            "1" => list_products(&inventory),
            "2" => add_product(&mut inventory),
            "3" => update_product(&mut inventory),
            "4" => remove_product(&mut inventory),
            "5" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn add_product(inventory: &mut Vec<Product>) {
    println!("--- Add a new Product ---");
    let name = read_input("Product Name: ");
    let quantity_input = read_input("Quantity: ");

    match quantity_input.parse::<u32>() {
        Ok(quantity) => {
            let product = Product {
                name: name.clone(),
                quantity,
            };
            inventory.push(product);
            println!("Product '{}' added successfully!", name);
        }
        Err(_) => {
            println!("Invalid quantity. Please enter a number.");
        }
    }
}

fn update_product(inventory: &mut Vec<Product>) {
    println!("--- Update a Product ---");

    if inventory.is_empty() {
        println!("The inventory is currently empty.");
        return;
    }

    list_products(inventory);

    let index_input = read_input("Enter the number of the product to update: ");
    let index: usize = match index_input.parse::<usize>() {
        Ok(num) if num > 0 && num <= inventory.len() => num - 1,
        _ => {
            println!("Invalid product number. Please enter a valid number.");
            return;
        }
    };

    let quantity_input = read_input("Enter the new quantity: ");
    let quantity: u32 = match quantity_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity. Please enter a number.");
            return;
        }
    };

    inventory[index].quantity = quantity;
    println!("Product '{}' updated successfully!", inventory[index].name);
}

fn list_products(inventory: &Vec<Product>) {
    println!("--- List of all products ---");
    if inventory.is_empty() {
        println!("The inventory is currently empty.");
    } else {
        for (i, product) in inventory.iter().enumerate() {
            println!("{}. {:?}", i + 1, product);
        }
    }
}

fn remove_product(inventory: &mut Vec<Product>) {
    println!("--- Remove a Product ---");

    if inventory.is_empty() {
        println!("The inventory is currently empty.");
        return;
    }

    // List all products with indices
    list_products(inventory);

    let index_input = read_input("Enter the number of the product to remove: ");
    let index: usize = match index_input.parse::<usize>() {
        Ok(num) if num > 0 && num <= inventory.len() => num - 1,
        _ => {
            println!("Invalid product number. Please enter a valid number.");
            return;
        }
    };

    let product_name = inventory[index].name.clone();
    inventory.remove(index);
    println!("Product '{}' removed successfully!", product_name);
}

fn load_inventory() -> Vec<Product> {
    if Path::new(INVENTORY_FILE).exists() {
        let content = fs::read_to_string(INVENTORY_FILE).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_inventory(inventory: &Vec<Product>) {
    let data = serde_json::to_string(inventory).expect("Error serializing inventory.");
    fs::write(INVENTORY_FILE, data).expect("Error saving inventory to file.");
}
