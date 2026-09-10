use diesel::{ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, dsl::insert_into};
use crate::{model::{NewProduct, Product}, schema::product};
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum AppError{
    #[error("Part Number PartNumberAlreadyExits")]
    PartNumberAlreadyExits,

    #[error("Database issue")]
    Database(#[from]diesel::result::Error)
}

pub fn handle_product_insertion(connection: &mut MysqlConnection, part:NewProduct)  -> Result<String, AppError> {
    let check = product::table
    .select(Product::as_select())
    .filter(product::part_number.eq(&part.part_number))
    .first(connection).optional();

    match check {
        Ok(Some(_)) => return Err(AppError::PartNumberAlreadyExits),
        Ok(None) => {},
        Err(err) => return Err(AppError::Database(err)),
    }

   let insert_into =  insert_into(product::table).values(part).execute(connection);

    match insert_into {
        Ok(_) => Ok(" Everything is done bro".to_string()),
        Err(e) => return Err(AppError::Database(e))
    }

}