use yew::prelude::*;

#[derive(Clone)]
struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct CartProduct {
    product: Product,
    quantity: i32,
}

struct State {
    products: Vec<Product>,
    cart: Vec<CartProduct>,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddToCart(i32),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let products: Vec<Product> = vec![
            Product {
                id: 1,
                name: "Apple".to_string(),
                description: "An apple a day".to_string(),
                image: "/products/apple.jpg".to_string(),
                price: 3.64,
            },
            Product {
                id: 2,
                name: "Bannana".to_string(),
                description: "An old bannana".to_string(),
                image: "/products/banana.jpg".to_string(),
                price: 7.99,
            },
        ];

        let cart = vec![];

        Self {
            state: State { products, cart },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::AddToCart(product_id) => {
                let product = self
                    .state
                    .products
                    .iter()
                    .find(|p: &&Product| p.id == product_id)
                    .unwrap();
                let cart_product = self
                    .state
                    .cart
                    .iter_mut()
                    .find(|cp: &&mut CartProduct| cp.product.id == product_id);

                if let Some(cp) = cart_product {
                    cp.quantity += 1;
                } else {
                    self.state.cart.push(CartProduct {
                        product: product.clone(),
                        quantity: 1,
                    })
                }
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                let product_id = product.id;
                html! {
                    <div>
                      <img src={&product.image}/>
                      <div>{&product.name}</div>
                      <div>{"$"}{&product.price}</div>
                      <button onclick=self.link.callback(move |_| Msg::AddToCart(product_id))>{"Add to Cart"}</button>
                   </div>
                }
            })
            .collect();
        let cart_value = self
            .state
            .cart
            .iter()
            .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

        html! {
              <div>
                <span>{format!("Cart Value: {:.2}", cart_value)}</span>
                <span>{products} </span>
              </div>
        }
    }
}
