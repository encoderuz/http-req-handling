use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::{enums::TaskStatus, to_do_factory, ItemTypes};
use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;

pub async fn get() -> impl Responder {
    ToDoItems::get_state()
}