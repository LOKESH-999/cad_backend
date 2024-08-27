use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
// use prices::cost;
// use crate::schema::status::dsl::*;
use crate::schema::*;
use crate::model::*;
use diesel::sql_types::{Unsigned, BigInt,TinyInt,Nullable}; 
use diesel::sql_query;
use std::sync::Arc;
use crate::spinlock::SpinLock;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref CONN:Arc<SpinLock<MysqlConnection>>={
        let conn = establish_conn();
        Arc::new(SpinLock::new(conn))
    };
}


#[derive(QueryableByName, Debug)]
struct MaxId {
    #[sql_type = "Nullable<Unsigned<TinyInt>>"]
    mid: Option<u8>,
}

#[derive(QueryableByName, Debug)]
struct MaxIdU {
    #[sql_type = "Nullable<Unsigned<BigInt>>"]
    mid: Option<u64>,
}
pub fn establish_conn() -> MysqlConnection {
    MysqlConnection::establish("mysql://root:qwerty@localhost:3306/canadian_pride_oil")
        .expect("Error connecting to the database")
}

#[allow(unused)]
pub fn add_status_catagory(conn:Arc<SpinLock<MysqlConnection>>,name:String)->Result<u8, diesel::result::Error>{
    let mut conn=conn.lock();
    match diesel::insert_into(status::table)
    .values(
        Status{
            id:None,
            status1:name
        }
    ).execute(&mut *conn){
        Ok(_)=>{
            let result: MaxId = sql_query("SELECT MAX(id) as mid FROM status")
            .get_result::<MaxId>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
            }
        Err(x)=>Err(x)
    }

}

#[allow(unused)]
pub fn add_package_catagory(conn:Arc<SpinLock<MysqlConnection>>,liter:f32)->Result<u8, diesel::result::Error>{
    let mut conn=conn.lock();
    match diesel::insert_into(package_cat::table)
    .values(
        PackageCat{
        id:None,
        liter:liter
    }
    ).execute(&mut *conn){
        Ok(_)=>{
            let result: MaxId = sql_query("SELECT MAX(id) as mid FROM package_cat")
            .get_result::<MaxId>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}

#[allow(unused)]
pub fn add_product_catogory(conn:Arc<SpinLock<MysqlConnection>>,name:String)->Result<u8,diesel::result::Error>{
    let mut conn=conn.lock();
    match diesel::insert_into(product_cat::table)
    .values(
        ProductCat{
            id:None,
            name:name
        }
    ).execute(&mut *conn){
        Ok(_)=>{
            let result: MaxId = sql_query("SELECT MAX(id) as mid FROM product_cat")
            .get_result::<MaxId>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}


#[allow(unused)]
pub fn add_order_details(conn:Arc<SpinLock<MysqlConnection>>,product_id:u8,package_id:u8,quantity:u32,total:f32,order_id:u64,date:NaiveDateTime)->Result<u64, diesel::result::Error>{
    let val=OrderDetails{
        id:None,
        pid:product_id,
        package_id:package_id,
        quantity:quantity,
        total:total,
        order_id:order_id,
        date:date.into()
    };
    let mut conn=conn.lock();
    match diesel::insert_into(order_details::table)
    .values(val)
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxIdU = sql_query("SELECT MAX(id) as mid FROM order_details")
            .get_result::<MaxIdU>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}

#[allow(unused)]
pub fn add_order_status(conn:Arc<SpinLock<MysqlConnection>>,status_id:u8,order_id:u64,date_time:NaiveDateTime)->Result<u64, diesel::result::Error>{
    let val=OrderStatus{
        id:None,
        order_id:order_id,
        status:status_id,
        data_time:date_time
    };
    let mut conn=conn.lock();
    match diesel::insert_into(order_status::table)
    .values(val)
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxIdU = sql_query("SELECT MAX(id) as mid FROM order_status")
            .get_result::<MaxIdU>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }

}

#[allow(unused)]
pub fn add_orders(conn:Arc<SpinLock<MysqlConnection>>,shipping_id:Option<u64>,cid:u64,payment_id:u64,order_status_id:u64,order_status:u8,date:NaiveDateTime,total:f64)->Result<u64, diesel::result::Error>{
    let val=Orders{
        shipping_id:shipping_id,
        customerid:cid,
        order_id:None,
        order_status_id:order_status_id,
        payment_id:payment_id,
        order_status:order_status,
        order_date:date.into(),
        total:total
    };
    let mut conn=conn.lock();
    match diesel::insert_into(orders::table)
    .values(val)
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxIdU = sql_query("SELECT MAX(id) as mid FROM orders")
            .get_result::<MaxIdU>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}

#[allow(unused)]
pub fn add_payments(conn:Arc<SpinLock<MysqlConnection>>,method:u8,order_id:u64,status_id:u8,transfer_id:String)->Result<u64, diesel::result::Error>{
    let mut conn=conn.lock();
    let val=Payment{
        method:method,
        order_id:order_id,
        payment_id:None,
        Status:status_id,
        transfer_id:transfer_id
    };
    match diesel::insert_into(payment::table)
    .values(val)
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxIdU = sql_query("SELECT MAX(id) as mid FROM payment")
            .get_result::<MaxIdU>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}

#[allow(unused)]
pub fn add_payment_method(conn:Arc<SpinLock<MysqlConnection>>,name:String)->Result<u8, diesel::result::Error>{
    let mut conn=conn.lock();
    match diesel::insert_into(payment_method::table)
    .values(PaymentMethod{
        id:None,
        name:name
    })
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxId = sql_query("SELECT MAX(id) as mid FROM payment")
            .get_result::<MaxId>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}


#[allow(unused)]
pub fn add_shipping_details(conn:Arc<SpinLock<MysqlConnection>>,carrier:String,trackin_id:String,eda:NaiveDate,cost:f32,address:String,order_id:u64)->Result<u64, diesel::result::Error>{
    let mut conn=conn.lock();
    let val=ShippingDetails{
        ship_id:None,
        carrier:carrier,
        tno:trackin_id,
        eda:eda.into(),
        cost:cost,
        address:address,
        order_id:order_id
    };
    match diesel::insert_into(shipping_details::table)
    .values(val)
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxIdU = sql_query("SELECT MAX(id) as mid FROM shipping_details")
            .get_result::<MaxIdU>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}

#[allow(unused)]
pub fn add_status_detail(conn:Arc<SpinLock<MysqlConnection>>,status_id:u8,time_stamp:NaiveDateTime,order_id:u64)->Result<u64, diesel::result::Error>{
    let val=StatusDetails{
        id:None,
        status_id:status_id,
        time_stamp:time_stamp,
        order_id:order_id
    };
    let mut conn=conn.lock();
    match diesel::insert_into(status_details::table)
    .values(val)
    .execute(&mut *conn){
        Ok(_)=>{
            let result: MaxIdU = sql_query("SELECT MAX(id) as mid FROM status_details")
            .get_result::<MaxIdU>(&mut *conn).unwrap();
            Ok(result.mid.unwrap())
        }
        Err(x)=>Err(x)
    }
}

#[allow(unused)]
pub fn add_pp_price(conn:Arc<SpinLock<MysqlConnection>>,product_id:u8,package_id:u8,price:f32)->Result<u32, diesel::result::Error>{
    let ppid=product_id as u32*100+package_id as u32;
    let mut conn=conn.lock();
    match diesel::insert_into(prices::table).
    values(Price{
        ppid:ppid,
        cost:price
    }).execute(&mut *conn) {
        Ok(_)=>Ok(ppid),
        Err(x)=>Err(x)
    }
}