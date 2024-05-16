use std::io;

fn main() {
    println!("Welcome to the Mysterious Forest Adventure!");
    println!("You find yourself standing at the edge of a dark and mysterious forest.");
    println!("Your goal is to find the hidden treasure deep within the forest.");

    let mut choice = String::new();

    loop {
        println!("\nWhat do you want to do?");
        println!("1. Enter the forest.");
        println!("2. Turn back and leave.");
        println!("3. Take a nap (just kidding, don't do that).");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => enter_forest(),
            "2" => {
                println!("You decide to leave the forest. Goodbye!");
                break;
            }
            "3" => println!("Nice try, but you can't take a nap here! Choose something else."),
            _ => println!("Invalid choice. Please enter a number from 1 to 3."),
        }
    }
}

fn enter_forest() {
    println!("You step into the dark forest and begin your journey.");
    println!("As you venture deeper, you come across a fork in the path.");

    let mut choice = String::new();

    loop {
        println!("\nWhich path do you choose?");
        println!("1. Take the left path.");
        println!("2. Take the right path.");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("You chose the left path.");
                left_path();
                break;
            }
            "2" => {
                println!("You chose the right path.");
                right_path();
                break;
            }
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}

fn left_path() {
    println!("You follow the left path and encounter a river blocking your way.");
    println!("You must decide how to cross it.");

    let mut choice = String::new();

    loop {
        println!("\nHow do you want to cross the river?");
        println!("1. Swim across.");
        println!("2. Look for a bridge.");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("You bravely attempt to swim across the river...");
                println!("But unfortunately, you get swept away by the current and washed downstream. Game over!");
                break;
            }
            "2" => {
                println!("You search around and find a hidden bridge.");
                println!("You cross the bridge safely and continue your journey.");
                println!("After a while, you stumble upon a clearing with a chest.");
                println!("Congratulations! You found the treasure! You win!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}

fn right_path() {
    println!("You chose the right path and encounter a dense thicket of thorns.");
    println!("You must find a way to get through it.");

    let mut choice = String::new();

    loop {
        println!("\nHow do you want to get through the thicket?");
        println!("1. Cut through the thorns with your sword.");
        println!("2. Search for a path around the thicket.");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("You start hacking away at the thorns with your sword.");
                println!("After a long struggle, you manage to clear a path and continue your journey.");
                println!("You eventually find a hidden chest containing the treasure! You win!");
                break;
            }
            "2" => {
                println!("You search around and find a narrow path leading around the thicket.");
                println!("You carefully navigate through the path and continue your journey.");
                println!("After a while, you stumble upon a clearing with a chest.");
                println!("Congratulations! You found the treasure! You win!");
                break;
            }
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}