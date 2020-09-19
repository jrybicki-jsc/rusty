use crate::api;
use crate::types::{CartProduct, Product};
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;

use yew::prelude::*;


struct State {
    products: Vec<Product>,
    cart: Vec<CartProduct>,
    get_products_error: Option<Error>, 
    get_products_loaded: bool,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    AddToCart(i32),
    GetProducts,
    GetProductsSuccess(Vec<Product>),
    GetProductsError(Error),
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
        let products = vec![];
        link.send_message(Msg::GetProducts);

        Self {
            state: State { 
                  products, 
                  cart, 
                  get_products_error: None,
                  get_products_loaded: false
             },
             link,
             task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetProducts => {
                self.state.get_products_loaded = false;
                let handler = 
                    self.link
                        .callback(move | response: api::FetchResponse<Vec<Product>>| {
                                  let (_, Json(data)) = response.into_parts();
                                   match data {
                                        Ok(products) => Msg::getProductsSuccess(products),
                                        Err(err) => Msg::GetProductsError(err),
                                   }
                          });
                     self.task = Some(api::get_prodcuts(handler));
                     true
            }
            Msg::GetProductsSuccess(products) => {
                 self.state.products = products;
                 self.state.get_products_load = true;
                 true
            }
            Msg::GetProductsError(error) => {
                 self.state.get_products_error = Some(error);
                 self.state.get_products_loaded = true;
                 true
            }
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
                true
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

        if !self.state.get_producss_loaded {
           html! { <div> {"loading..."} </div>
        } else if let Some(_) = self.state.get_products_error {
           html! { <div> <span> {"Error loading products!" } </span> </div>
        } else {
 

        html! {
              <div>
                <span>{format!("Cart Value: {:.2}", cart_value)}</span>
                <span>{products} </span>
              </div>
        }
       }
    }
}
