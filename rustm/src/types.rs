use serde::{Deserialize, Serialize}


#[derive(Deserialize, Serialize, Clone, Debug)]
struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

#[derive(Clone, Debug)]
struct CartProduct {
    product: Product,
    quantity: i32,
}
