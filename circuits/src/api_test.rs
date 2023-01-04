// thanks to https://github.com/tmsdev82/rust-warp-rest-api-tutorial
use crate::comms::*;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};

type ItemsDb = Arc<Mutex<HashMap<usize, ShoppingListItem>>>;
type ItemNode = Arc<Mutex<String>>;
type ItemComms<'a> = Arc<Mutex<UDPComms<'a>>>;
type Result<T> = std::result::Result<T, Rejection>;

pub struct ApiNode<'a> {
    port_in: &'a str,
    conns: HashMap<&'a str, &'a str>,
}

impl<'a> ApiNode<'a> {
    pub fn new(name: &'a str, conns: HashMap<&'a str, &'a str>) -> Self {
        let node = match get_port(name, conns.clone()) {
            Ok(c) => ApiNode { port_in: c, conns },
            Err(_) => {
                panic!(
                    "couldn't find a port to itself: {} (HINT: check name at main.rs)",
                    name
                );
            }
        };
        node
    }
    //pub async fn talk(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
    pub async fn talk(&mut self) {
        let led_node = get_port("led_action_server", self.conns.clone()).unwrap();
        let comms = UDPComms::new(self.port_in.to_owned());
        run(led_node.to_owned(), comms).await;
    }
}

async fn run(led_n: String, comms_orig: UDPComms<'static>) {
    let items_db: ItemsDb = Arc::new(Mutex::new(HashMap::new()));
    let led_node: ItemNode = Arc::new(Mutex::new(led_n));
    let comms: ItemComms = Arc::new(Mutex::new(comms_orig));
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let get_route = warp::path("get")
        .and(warp::post())
        .and(with_node(led_node.clone()))
        .and(with_comms(comms.clone()))
        .and_then(test_do);

    let shopping_list_items_route = warp::path("shopping_list_items")
        .and(warp::get())
        .and(with_items_db(items_db.clone()))
        .and_then(get_shopping_list_items);

    let shopping_list_item_route = warp::path("shopping_list_item")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_items_db(items_db.clone()))
        .and_then(create_shopping_list_item)
        .or(warp::path!("shopping_list_item" / usize)
            .and(warp::get())
            .and(with_items_db(items_db.clone()))
            .and_then(get_shopping_list_item_by_id))
        .or(warp::path!("shopping_list_item" / usize)
            .and(warp::put())
            .and(warp::body::json())
            .and(with_items_db(items_db.clone()))
            .and_then(update_shopping_list_item_by_id))
        .or(warp::path!("shopping_list_item" / usize)
            .and(warp::delete())
            .and(with_items_db(items_db.clone()))
            .and_then(delete_shopping_list_item_by_id));

    let routes = root
        .or(get_route)
        .or(shopping_list_items_route)
        .or(shopping_list_item_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

fn with_node(
    node_name: ItemNode,
) -> impl Filter<Extract = (ItemNode,), Error = Infallible> + Clone {
    warp::any().map(move || node_name.clone())
}

fn with_comms(comms: ItemComms) -> impl Filter<Extract = (ItemComms,), Error = Infallible> + Clone {
    warp::any().map(move || comms.clone())
}

fn with_items_db(
    items_db: ItemsDb,
) -> impl Filter<Extract = (ItemsDb,), Error = Infallible> + Clone {
    warp::any().map(move || items_db.clone())
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ShoppingListItemType {
    Drink,
    Desert,
    Fruit,
    Snack,
    Spread,
    Vegetable,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShoppingListItem {
    pub item_id: Option<usize>,
    pub name: String,
    pub item_type: ShoppingListItemType,
    pub description: String,
    pub price: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateShoppingListItem {
    pub name: Option<String>,
    pub item_type: Option<ShoppingListItemType>,
    pub description: Option<String>,
    pub price: Option<f32>,
}

//use crate::{models, ItemsDb, Result};
use warp::{http::StatusCode, reply, Reply};

pub async fn test_do(led_n: ItemNode, comms_orig: ItemComms<'_>) -> Result<impl Reply> {
    let result = "";

    Ok(reply::with_status(reply::json(&result), StatusCode::OK))
}

pub async fn get_shopping_list_items(items_db: ItemsDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let local_db: Vec<ShoppingListItem> = local_db.values().cloned().collect();

    Ok(reply::with_status(reply::json(&local_db), StatusCode::OK))
}

pub async fn create_shopping_list_item(
    mut shopping_list_item: ShoppingListItem,
    items_db: ItemsDb,
) -> Result<impl Reply> {
    println!("Received UserData: {:?}", shopping_list_item);
    let mut local_db = items_db.lock().await;
    let key_count = local_db.keys().len();
    shopping_list_item.item_id = Some(key_count);
    local_db.insert(key_count, shopping_list_item.clone());

    Ok(reply::with_status(
        reply::json(&shopping_list_item),
        StatusCode::CREATED,
    ))
}

pub async fn get_shopping_list_item_by_id(id: usize, items_db: ItemsDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let shopping_list_item = match local_db.get(&id) {
        Some(item) => item,
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    Ok(reply::with_status(
        reply::json(&shopping_list_item),
        StatusCode::OK,
    ))
}

pub async fn update_shopping_list_item_by_id(
    id: usize,
    updated_data: UpdateShoppingListItem,
    items_db: ItemsDb,
) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;
    let mut shopping_list_item = match local_db.get(&id) {
        Some(item) => item.clone(),
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    match updated_data.name {
        Some(name) => {
            println!("updating name from {} to {}", shopping_list_item.name, name);
            shopping_list_item.name = name;
        }
        _ => {}
    };

    match updated_data.description {
        Some(description) => {
            println!(
                "updating description from {} to {}",
                shopping_list_item.description, description
            );
            shopping_list_item.description = description;
        }
        _ => {}
    };

    match updated_data.item_type {
        Some(item_type) => {
            println!(
                "updating item_type from {:?} to {:?}",
                shopping_list_item.item_type, item_type
            );
            shopping_list_item.item_type = item_type;
        }
        _ => {}
    };

    match updated_data.price {
        Some(price) => {
            println!(
                "updating price from {} to {}",
                shopping_list_item.price, price
            );
            shopping_list_item.price = price;
        }
        _ => {}
    };

    *local_db.get_mut(&id).unwrap() = shopping_list_item.clone();

    Ok(reply::with_status(
        reply::json(&shopping_list_item),
        StatusCode::OK,
    ))
}

pub async fn delete_shopping_list_item_by_id(id: usize, items_db: ItemsDb) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;

    println!("deleting shopping list item with id: {}", id);
    local_db.remove(&id);

    Ok(reply::with_status(
        reply::html("delete success"),
        StatusCode::OK,
    ))
}
