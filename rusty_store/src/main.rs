use std::io;
use std::env;
use dotenv::dotenv;

fn authenticate() -> bool {

    dotenv().ok();

    let username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| {
        eprintln!("Warning: ADMIN_USERNAME not set in environment");
        "admin".to_string()
    });
    
    let password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| {
        eprintln!("Warning: ADMIN_PASSWORD not set in environment");
        "password".to_string()
    });

    println!("Enter username:");
    let mut input_username = String::new();
    std::io::stdin().read_line(&mut input_username).unwrap();

    println!("Enter password:");
    let mut input_password = String::new();
    std::io::stdin().read_line(&mut input_password).unwrap();

    input_username.trim() == username && input_password.trim() == password
}


fn main() {
    if !authenticate() {
        println!("Invalid credentials!");
        return;
    }

    let mut store = Store::new();

    loop {
        println!("\n1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. List Inventory");
        println!("5. Record Sale");
        println!("6. Record Purchase");
        println!("7. Generate Reports");
        println!("8. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter Product Name:");
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let name = choice.trim().to_string();

                println!("Enter Product Description:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let description = choice.trim().to_string();

                println!("Enter Product Price:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let price: f64 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                println!("Enter Product Quantity:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let quantity: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                let id = store.inventory.products.len() as u32 + 1;
                store.inventory.add_product(Product::new(id, name, description, price, quantity));
                println!("Product added successfully! {:?}", store.inventory.products);
            }
            "2" => {
                println!("Enter Product ID:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let id: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                match {
                    store.inventory.check_product(id)
                } {
                    Some(product) => {
                        println!("Product found! {:?}", product);
                    }
                    None => {
                        println!("Product not found!");
                    }
                }

                println!("Enter Product Name:");
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let name = choice.trim().to_string();

                println!("Enter Product Description:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let description = choice.trim().to_string();

                println!("Enter Product Price:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let price: f64 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                println!("Enter Product Quantity:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let quantity: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                store.inventory.edit_product(id, name, description, price, quantity);
                println!("Product edited successfully! {:?}", store.inventory.products);
            }
            "3" => {
                println!("Enter Product ID:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let id: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                match {
                    store.inventory.check_product(id)
                } {
                    Some(product) => {
                        println!("Product found! {:?}", product);
                    }
                    None => {
                        println!("Product not found!");
                    }
                }

                store.inventory.delete_product(id);
                println!("Product deleted successfully! {:?}", store.inventory.products);
            }

            "4" => {
                store.inventory.list_products();
            }
            "5" => {
                println!("Enter Product ID:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let product_id: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                println!("Enter Sale Quantity:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let quantity: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                println!("Enter Sale Price:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let sale_price: f64 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                match store.record_sale(product_id, quantity, sale_price) {
                    Ok(_) => {
                        println!("Sale recorded successfully! {:?}", store.sales);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            "6" => {
                println!("Enter Product ID:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let product_id: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                match {
                    store.inventory.check_product(product_id)
                } {
                    Some(product) => {
                        println!("Product found! {:?}", product);
                    }
                    None => {
                        println!("Product not found!");
                    }
                }

                println!("Enter Purchase Quantity:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let quantity: u32 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                println!("Enter Purchase Price:");
                choice.clear();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let purchase_price: f64 = match choice.trim().parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("Invalid number input. Please restart the program.");
                        return;
                    }
                };

                store.record_purchase(product_id, quantity, purchase_price);
                println!("Purchase recorded successfully! {:?}", store.purchases);
            }
            "7" => {
                println!("1. Generate Inventory Report");
                println!("2. Generate Sales Report");
                println!("3. Generate Purchase Report");

                let mut report_choice = String::new();
                io::stdin().read_line(&mut report_choice).unwrap();

                match report_choice.trim() {
                    "1" => store.generate_inventory_report(),
                    "2" => store.generate_sales_report(),
                    "3" => store.generate_purchase_report(),
                    _ => println!("Invalid report option!"),
                }
            }
            "8" => break,
            _ => println!("Invalid option!"),
        }
    }
}

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

#[derive(Debug, Clone)]
struct Sale {
    product_id: u32,
    quantity: u32,
    sale_price: f64,
    profit: f64,
}

#[derive(Debug, Clone)]
struct Purchase {
    product_id: u32,
    quantity: u32,
    purchase_price: f64,
    total_cost: f64,
}

impl Product {
    fn new(id: u32, name: String, description: String, price: f64, quantity: u32) -> Self {
        Self { id, name, description, price, quantity }
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self {
        Self { products: Vec::new() }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn check_product(&self, id: u32) -> Option<&Product> {
        self.products.iter().find(|p| p.id == id)
    }

    fn edit_product(&mut self, id: u32, name: String, description: String, price: f64, quantity: u32) {
        if let Some(product) = self.products.iter_mut().find(|p| p.id == id) {
            product.name = name;
            product.description = description;
            product.price = price;
            product.quantity = quantity;
        }
    }

    fn delete_product(&mut self, id: u32) {
        self.products.retain(|p| p.id != id);
    }

    fn list_products(&self) {
        for product in &self.products {
            println!("{:?}", product);
        }
    }
}

struct Store {
    inventory: Inventory,
    sales: Vec<Sale>,
    purchases: Vec<Purchase>,
}

impl Store {
    fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
        }
    }

    fn record_sale(&mut self, product_id: u32, quantity: u32, sale_price: f64) -> Result<(), String> {
        if let Some(product) = self.inventory.products.iter_mut().find(|p| p.id == product_id) {
            if product.quantity >= quantity {
                product.quantity -= quantity;
                let profit = (sale_price - product.price) * quantity as f64;
                self.sales.push(Sale { product_id, quantity, sale_price, profit });
                Ok(())
            } else {
                Err("Not enough stock".to_string())
            }
        } else {
            Err("Product not found".to_string())
        }
    }

    fn record_purchase(&mut self, product_id: u32, quantity: u32, purchase_price: f64) {
        if let Some(product) = self.inventory.products.iter_mut().find(|p| p.id == product_id) {
            product.quantity += quantity;
            self.purchases.push(Purchase {
                product_id,
                quantity,
                purchase_price,
                total_cost: purchase_price * quantity as f64,
            });
        }
    }
}

impl Store {
    fn generate_inventory_report(&self) {
        println!("Inventory Report:");
        for product in &self.inventory.products {
            println!("{:?}", product);
        }
    }

    fn generate_sales_report(&self) {
        println!("Sales Report:");
        for sale in &self.sales {
            println!("{:?}", sale);
        }
    }

    fn generate_purchase_report(&self) {
        println!("Purchase Report:");
        for purchase in &self.purchases {
            println!("{:?}", purchase);
        }
    }
}
