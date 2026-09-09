use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable,Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::product)]
pub struct NewProduct{
     pub name: String,
     pub price: i32,
     pub descri: String,
     pub part_number : String,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::product)]
pub struct Product {
     pub id: i32,
     pub name: String,
     pub price: i32,
     pub descri: String,
     pub part_number : String,
     pub created_at: Option<NaiveDateTime>,
     pub updated_at: Option<NaiveDateTime>
}

     //    id -> Integer,
     //    #[max_length = 255]
     //    name -> Varchar,
     //    price -> Integer,
     //    descri -> Nullable<Text>,
     //    #[max_length = 100]
     //    part_number -> Nullable<Varchar>,
     //    created_at -> Nullable<Timestamp>,
     //    updated_at -> Nullable<Timestamp>,