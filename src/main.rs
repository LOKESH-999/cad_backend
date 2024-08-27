mod model;
mod schema;
mod db;
mod spinlock;
use std::ops::DerefMut;
use std::sync::Arc;
use lazy_static::lazy_static;
// mod insert;
use spinlock::SpinLock;
use diesel::dsl::max;
// use diesel::expression::is_aggregate::No;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_types::BigInt;
use crate::schema::{status,order_status};
use crate::schema::status::dsl::*;
use diesel::sql_query;
use diesel::dsl::sql;
use diesel::sql_types::{Unsigned, Integer,TinyInt}; 
use diesel::QueryableByName;
use diesel::sql_types::Nullable;
// use diesel::sql_types::Unsigned;

#[derive(QueryableByName, Debug)]
struct MaxId {
    #[sql_type = "Nullable<Unsigned<TinyInt>>"]
    mid: Option<u8>,
}
lazy_static!{
    static ref CONN:Arc<SpinLock<MysqlConnection>>={
        let conn = establish_conn();
        Arc::new(SpinLock::new(conn))
    };
}

fn main() {
    let mut x = establish_conn();
    println!("Hello, world! {:?}", 12);
    let c=db::CONN.clone();
    let n = model::Status {
        id:None,
        status1: "f11ine2".to_string(), // Ensure the field name matches your model
    };
    let mut q=CONN.lock();
    // Perform the insert operation
    let r=diesel::insert_into(status::table)
        .values(&n)
        .execute(&mut *q);
    println!("{:?}",r);
    // let inserted_id = create_status(&mut x, "Active")
    //     .expect("Error inserting status");
    // let last_id: (u64,) = sql_query("SELECT LAST_INSERT_ID() AS id").get_result(&mut x).unwrap();
    // println!("{:?}",status::table.filter(last).first(&mut x));
    // println!("{:?}",inserted_id);
    // If you need to fetch the inserted row, perform a separate query
    let inserted_status: model::Status = status::table
        .filter(id.eq(1)) // Adjust filter criteria as needed
        .first(&mut *q)
        .expect("Error fetching inserted status");
    // let t=status.load::<order_status::status>(&mut x);

    // println!("Inserted status: {:?}", inserted_status);
    // let t=status.select::<u64>(max(status::id));
    // println!("{:?}",t);
    // diesel::select(status::table)
    // let re=status.select(max(sql::<Unsigned<Integer>>("id"))).first::<Option<u8>>(&mut x);
    // println!("{:?}",re);
    use diesel::sql_query;

let result: MaxId = sql_query("SELECT MAX(id) as mid FROM status")
    .get_result::<MaxId>(&mut *q)
    .expect("Error loading max id");
drop(q);
    println!("{:?}",db::add_order_details(CONN.clone(), 1, 1, 500, 89990.0, 1, chrono::prelude::Utc::now().naive_utc()));
println!("Maximum ID is: {:?}", result.mid);

}

fn establish_conn() -> MysqlConnection {
    MysqlConnection::establish("mysql://root:qwerty@localhost:3306/canadian_pride_oil")
        .expect("Error connecting to the database")
}
