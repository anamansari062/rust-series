mod order;
mod product;

use order::place_order;
use product::show_products;

use inquire::Select;

fn main() {
    println!("-------------------------------------------");
    println!("Welcome to Schrute Farms\nBy Dwight Schrute");
    println!("-------------------------------------------\n");
    menu();
}

fn menu(){
    let options = vec![
        "Place order",
        "See Product List",
        "Exit"
    ];

    let option = Select::new("Menu:", options.clone()).prompt();

    match option {
        Ok(option) => {
    
            if  option.eq(options[0]) {
                place_order();
            }
            else if option.eq(options[1]) {
                show_products();
            }
            else if option.eq(options[2]) {
                return;
            }
        },
        Err(err) => {
            println!("Err while reading choice: {}", err);
        },
    }
}





