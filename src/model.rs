use diesel::prelude::*;
use crate::schema::*;

use chrono::{NaiveDate,NaiveDateTime};

#[derive(Queryable,Selectable,Insertable,Debug)]
#[diesel(table_name = status)]
#[diesel(primary_key(id))]
pub struct Status{
    pub id: Option<u8>,
    pub status1: String
}

#[derive(Queryable, Selectable,Insertable, Debug)]
#[diesel(table_name = order_details)]
#[diesel(primary_key(id))]
pub struct OrderDetails {
    pub id: Option<u64>, // Unsigned<Bigint>
    pub pid: u8, // Unsigned<Tinyint>
    pub package_id: u8, // Unsigned<Tinyint>
    pub quantity: u32, // Unsigned<Integer>
    pub total: f32, // Unsigned<Float>
    pub order_id: u64, // Unsigned<Bigint>
    pub date: NaiveDateTime, // Datetime
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = product_cat)]
#[diesel(primary_key(id))]
pub struct ProductCat {
    pub id: Option<u8>, // Corresponds to TINYINT UNSIGNED
    pub name: String, // Corresponds to VARCHAR(60)
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = package_cat)]
#[diesel(primary_key(id))]
pub struct PackageCat {
    pub id: Option<u8>, // Corresponds to TINYINT UNSIGNED
    pub liter: f32, // Corresponds to FLOAT UNSIGNED
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = payment_method)]
#[diesel(primary_key(id))]
pub struct PaymentMethod {
    pub id: Option<u8>, // Corresponds to TINYINT UNSIGNED
    pub name: String, // Corresponds to VARCHAR(15)
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = order_status)]
#[diesel(primary_key(id))]
pub struct OrderStatus {
    pub order_id: u64, // Corresponds to BIGINT UNSIGNED
    pub status: u8, // Corresponds to TINYINT
    pub data_time: NaiveDateTime, // Corresponds to DATETIME
    pub id: Option<u64>, // Corresponds to BIGINT UNSIGNED
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = shipping_details)]
#[diesel(primary_key(ship_id))]
pub struct ShippingDetails {
    pub ship_id: Option<u64>, // Corresponds to BIGINT UNSIGNED
    pub carrier: String, // Corresponds to VARCHAR(10)
    pub tno: String, // Corresponds to VARCHAR(60)
    pub eda: NaiveDate, // Corresponds to DATE
    pub cost: f32, // Corresponds to FLOAT UNSIGNED
    pub address: String, // Corresponds to VARCHAR(250)
    pub order_id: u64, // Corresponds to BIGINT UNSIGNED
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = payment)]
#[diesel(primary_key(payment_id))]
pub struct Payment {
    pub method: u8, // Corresponds to TINYINT UNSIGNED
    pub order_id: u64, // Corresponds to BIGINT UNSIGNED
    pub payment_id: Option<u64>, // Corresponds to BIGINT UNSIGNED
    pub Status:u8, // Corresponds to INT UNSIGNED
    pub transfer_id: String, // Corresponds to VARCHAR(60)
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = status_details)]
#[diesel(primary_key(id))]
pub struct StatusDetails {
    pub id: Option<u64>,
    pub status_id: u8,
    pub time_stamp:NaiveDateTime,
    pub order_id:u64
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = orders)]
#[diesel(primary_key(shipping_id))]
pub struct Orders {
    pub shipping_id: Option<u64>, // Corresponds to BIGINT UNSIGNED
    pub customerid: u64, // Corresponds to BIGINT UNSIGNED
    pub order_id: Option<u64>, // Corresponds to BIGINT UNSIGNED
    pub payment_id: u64, // Corresponds to BIGINT
    pub order_status_id: u64, // Nullable BIGINT UNSIGNED
    pub order_status: u8, // Corresponds to TINYINT UNSIGNED
    pub order_date: NaiveDateTime, // Corresponds to DATETIME
    pub total: f64, // Corresponds to DOUBLE UNSIGNED
}


#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = prices)]
#[diesel(primary_key(shipping_id))]
pub struct Price{
    pub ppid:u32,
    pub cost:f32
}