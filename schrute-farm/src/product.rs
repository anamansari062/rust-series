use::std::error::Error;

use csv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProductInfo {
    name: String,
    rate: u8
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers();
    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: ProductInfo = result?;
        println!("{:?}", record);
    }

    Ok(())
}


pub fn show_products(){
    if let Err(e) = read_from_file("./data/products.csv") {
        eprintln!("{}", e);
    }
}