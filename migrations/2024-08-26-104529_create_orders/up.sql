CREATE TABLE `product_cat` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(60) NOT NULL,
    PRIMARY KEY (`id`) USING HASH,
    UNIQUE `pname` (`name`(60))
) ENGINE = InnoDB;

CREATE TABLE `package_cat` (
    `id` TINYINT UNSIGNED NOT NULL  AUTO_INCREMENT,
    `liter` FLOAT NOT NULL,
    PRIMARY KEY (`id`) USING HASH,
    UNIQUE `ml` (`liter`)
) ENGINE = InnoDB;

CREATE TABLE `payment_method` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(15) NOT NULL,
    PRIMARY KEY (`id`) USING HASH,
    UNIQUE `methods` (`name`) USING HASH
) ENGINE = InnoDB;

CREATE TABLE `status` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `status1` VARCHAR(20) NOT NULL,
    PRIMARY KEY (`id`) USING HASH,
    UNIQUE `status` (`status1`)
) ENGINE = InnoDB;

CREATE TABLE `order_status` (
    `order_id` BIGINT UNSIGNED NOT NULL,
    `status` TINYINT UNSIGNED NOT NULL,
    `data_time` DATETIME NOT NULL,
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    PRIMARY KEY (`id`) USING HASH
) ENGINE = InnoDB;

CREATE TABLE `shipping_details` (
    `ship_id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `carrier` VARCHAR(10) NOT NULL,
    `tno` VARCHAR(60) NOT NULL,
    `eda` DATE NOT NULL,
    `cost` FLOAT NOT NULL,
    `address` VARCHAR(250) NOT NULL,
    `order_id` BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (`ship_id`) USING HASH
) ENGINE = InnoDB;

CREATE TABLE `payment` (
    `method` TINYINT UNSIGNED NOT NULL,
    `order_id` BIGINT UNSIGNED NOT NULL,
    `payment_id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `Status` TINYINT UNSIGNED NOT NULL,
    `transfer_id` VARCHAR(60) NOT NULL,
    PRIMARY KEY (`payment_id`) USING HASH
) ENGINE = InnoDB;

-- CREATE TABLE `status_cat` (
--     `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
--     `status_detail` VARCHAR(20) NOT NULL,
--     PRIMARY KEY (`id`) USING HASH,
--     UNIQUE `details` (`status_detail`)
-- ) ENGINE = InnoDB;

CREATE TABLE `order_details` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `pid` TINYINT UNSIGNED NOT NULL,
    `package_id` TINYINT UNSIGNED NOT NULL,
    `quantity` INT UNSIGNED NOT NULL,
    `total` FLOAT NOT NULL,
    `order_id` BIGINT UNSIGNED NOT NULL,
    `date` DATETIME NOT NULL,
    PRIMARY KEY (`id`) USING HASH
) ENGINE = InnoDB;

CREATE TABLE `orders` (
    `shipping_id` BIGINT UNSIGNED ,
    `customerid` BIGINT UNSIGNED NOT NULL,
    `order_id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `payment_id` BIGINT UNSIGNED NOT NULL,
    `order_status_id` BIGINT UNSIGNED NOT NULL ,
    `order_status` TINYINT UNSIGNED NOT NULL,
    `order_date` DATETIME NOT NULL,
    `total` DOUBLE NOT NULL,
    PRIMARY KEY (`order_id`) USING HASH,
    UNIQUE (`shipping_id`) USING HASH
) ENGINE = InnoDB;

CREATE TABLE `status_details` (
    `id` BIGINT UNSIGNED NOT NULL , 
    `status_id` TINYINT UNSIGNED NOT NULL ,
    `time_stamp` DATETIME NOT NULL , 
    `order_id` BIGINT UNSIGNED NOT NULL ,
    PRIMARY KEY (`id`) USING HASH
) ENGINE = InnoDB;


CREATE TABLE `prices` (
    `ppid` INT UNSIGNED NOT NULL ,
    `cost` FLOAT NOT NULL , 
    PRIMARY KEY (ppid) USING HASH
) ENGINE = InnoDB;