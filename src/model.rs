use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable,Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::product)]
pub struct NewProduct{
     pub name: String,
     pub price: i32,
     pub descri: Option<String>,
     pub part_number : Option<String>,
}