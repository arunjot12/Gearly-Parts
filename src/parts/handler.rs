use diesel::{ExpressionMethods, MysqlConnection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::{schema::product, model::Product};
use axum::http::StatusCode;

pub async fn handle_product_insertion(connection: &mut MysqlConnection, part_number: &String) {

    let check = product::table
    .select(Product::as_select())
    .filter(product::part_number.eq(&part_number))
    .first(connection).optional();
}