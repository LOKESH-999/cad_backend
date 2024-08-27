// @generated automatically by Diesel CLI.

diesel::table! {
    order_details (id) {
        id -> Nullable<Unsigned<Bigint>>,
        pid -> Unsigned<Tinyint>,
        package_id -> Unsigned<Tinyint>,
        quantity -> Unsigned<Integer>,
        total -> Float,
        order_id -> Unsigned<Bigint>,
        date -> Datetime,
    }
}

diesel::table! {
    order_status (id) {
        order_id -> Unsigned<Bigint>,
        status -> Unsigned<Tinyint>,
        data_time -> Datetime,
        id -> Nullable<Unsigned<Bigint>>,
    }
}

diesel::table! {
    orders (order_id) {
        shipping_id -> Nullable<Unsigned<Bigint>>,
        customerid -> Unsigned<Bigint>,
        order_id -> Nullable<Unsigned<Bigint>>,
        payment_id -> Unsigned<Bigint>,
        order_status_id -> Unsigned<Bigint>,
        order_status -> Unsigned<Tinyint>,
        order_date -> Datetime,
        total -> Double,
    }
}

diesel::table! {
    package_cat (id) {
        id -> Nullable<Unsigned<Tinyint>>,
        liter -> Float,
    }
}

diesel::table! {
    payment (payment_id) {
        method -> Unsigned<Tinyint>,
        order_id -> Unsigned<Bigint>,
        payment_id -> Nullable<Unsigned<Bigint>>,
        Status -> Unsigned<Tinyint>,
        #[max_length = 60]
        transfer_id -> Varchar,
    }
}

diesel::table! {
    payment_method (id) {
        id -> Nullable<Unsigned<Tinyint>>,
        #[max_length = 15]
        name -> Varchar,
    }
}

diesel::table! {
    prices (ppid) {
        ppid -> Nullable<Unsigned<Integer>>,
        cost -> Float,
    }
}

diesel::table! {
    product_cat (id) {
        id -> Nullable<Unsigned<Tinyint>>,
        #[max_length = 60]
        name -> Varchar,
    }
}

diesel::table! {
    shipping_details (ship_id) {
        ship_id -> Nullable<Unsigned<Bigint>>,
        #[max_length = 10]
        carrier -> Varchar,
        #[max_length = 60]
        tno -> Varchar,
        eda -> Date,
        cost -> Float,
        #[max_length = 250]
        address -> Varchar,
        order_id -> Unsigned<Bigint>,
    }
}

diesel::table! {
    status (id) {
        id -> Nullable<Unsigned<Tinyint>>,
        #[max_length = 20]
        status1 -> Varchar,
    }
}

diesel::table! {
    status_details (id) {
        id -> Nullable<Unsigned<Bigint>>,
        status_id -> Unsigned<Tinyint>,
        time_stamp -> Datetime,
        order_id -> Unsigned<Bigint>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    order_details,
    order_status,
    orders,
    package_cat,
    payment,
    payment_method,
    prices,
    product_cat,
    shipping_details,
    status,
    status_details,
);
