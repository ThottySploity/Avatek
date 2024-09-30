-- This SQL file will create the database accordingly
-- Remove this file after running it in your database

CREATE DATABASE halcyon;
use halcyon;

CREATE TABLE users (
    UserId int PRIMARY KEY NOT NULL,
    Username varchar(255),
    Password varchar(255)
);

CREATE TABLE beacons (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    BeaconId varchar(255),
    User varchar(255),
    InternalAddress varchar(255),
    SystemOs varchar(255),
    SystemVersion varchar(255),
    Domain varchar(255),
    IdleTime varchar(255),
    Priv varchar(255),
    Process varchar(255),
    PID varchar(255),
    ParentBeacon varchar(255),
    Contact date
);

CREATE TABLE credentials (
    Beacon varchar(255) NOT NULL,
    User varchar(255),
    Password varchar(255),
    Domain varchar(255),
    Note varchar(255)
);

CREATE TABLE tasking (
    Beacon varchar(255) NOT NULL,
    Task varchar(255),
    TaskResponse varchar(100000),
    TaskComplete BIT NOT NULL,
    TaskBit BIT NOT NULL,
    TaskDate DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO Users (UserId, Username, Password) VALUES (1, "Admin", "$2y$10$JH6s/qrimy.K7fK/VftaR.25uVni6rhpWInJpQqFYY5Iah0yx4vau");