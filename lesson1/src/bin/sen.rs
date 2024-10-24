use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

struct Store {
    products: HashMap<String, Product>,
}

impl Store {
    fn new() -> Store {
        Store {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, name: String, price: f64, quantity: u32) -> Result<(), &'static str> {
        // Реализуйте добавление товара с проверкой корректности данных
        if price <= 0.0 || quantity == 0 {
            return Err("Invalid price or quantity.");
        }

        if self.products.contains_key(&name) {
            return Err("Product already exists.");
        }

        self.products.insert(name.clone(), Product { name, price, quantity });
        Ok(())
    }

    // fn update_product(&mut self, name: &str, new_price: Option<f64>, new_quantity: Option<u32>) -> Result<(), &'static str> {
    //     // Реализуйте обновление данных о товаре с проверкой наличия товара
        
    // }

    fn find_product(&self, name: &str) -> Option<&Product> {
        // Реализуйте поиск товара по названию
        return self.products.get(name)
    }

    fn remove_product(&mut self, name: &str) -> Result<(), &'static str> {
        // Реализуйте удаление товара с проверкой наличия товара
        if self.products.remove(name).is_none() {
            Err("Product not found.")
        } else {
            Ok(())
        }
    }
}

fn main() {
    let mut store = Store::new();
    
    // Попытка добавить товар
    match store.add_product("Laptop".to_string(), 1000.0, 10) {
        Ok(_) => println!("Product added successfully"),
        Err(e) => println!("Error adding product: {}", e),
    }
    
    // // Обновление товара
    // match store.update_product("Laptop", Some(900.0), None) {
    //     Ok(_) => println!("Product updated successfully"),
    //     Err(e) => println!("Error updating product: {}", e),
    // }
    
    // Поиск товара
    if let Some(product) = store.find_product("Laptop") {
        println!("Found product: {:?}", product);
    } else {
        println!("Product not found");
    }
    
    // Удаление товара
    match store.remove_product("Laptop") {
        Ok(_) => println!("Product removed successfully"),
        Err(e) => println!("Error removing product: {}", e),
    }
}