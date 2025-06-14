use crate::json_serialization::to_do_item;
use super::super::enums::TaskStatus; 
use serde::Serialize; 
#[derive(Serialize)] 
pub struct Base { 
    pub title: String, 
    pub status: TaskStatus 
}