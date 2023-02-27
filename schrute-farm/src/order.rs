use inquire::{Select, CustomType};
use::std::error::Error;
use crate::utils::ProductInfo;

fn generate_options(path: &str) -> Result<Vec<ProductInfo>, Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    let mut records: Vec<ProductInfo> = vec![];
    for result in reader.deserialize() {
        let record: ProductInfo = result.unwrap_or(ProductInfo::new());
        records.push(record);
    }

    Ok(records)
}

fn take_order() -> Result<Option<u64>, Box<dyn Error>>{
    let product_options =  generate_options("./data/products.csv")?;
    let mut total:u64 = 0;
    let next_options = vec!["+", "bill", "quit"];
    loop{
        let product = Select::new("Vegetable: ", product_options.clone()).prompt()?;
        
        let unit = CustomType::<u64>::new("Units: ")
            .with_formatter(&|i| format!("{:} units", i))
            .with_error_message("Please type a valid number")
            .with_help_message("Maximum unit is 10")
            .prompt()?;
        total = total + (product.rate * unit);

        let next = Select::new("", next_options.clone())
        .with_formatter(&|_i| format!(""))
        .prompt()?;
        
        if next.eq(next_options[1]) {
            return Ok(Some(total));
        }
        else if next.eq(next_options[2]) {
            return Ok(None);
        }
        
    }
}

pub fn place_order(){
    let order = take_order();
    match order {
        Ok(amount) => {
            if let Some(a) = amount {
                println!("Your total is: ${}", a);
            };
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}