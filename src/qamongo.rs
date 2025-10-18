use mongodb::bson::{doc, ser::serialize_to_document, Document};
use mongodb::sync::{Client, Collection};
use serde::Serialize;
use lazy_static::lazy_static;
use crate::config::CONFIG;
use qifi_rs::QIFI;
use log::error;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

pub fn struct_to_doc<T>(value: T) -> Document
    where
        T: Serialize + std::fmt::Debug,
{
    serialize_to_document(&value)
        .unwrap()
}

fn create_mongo_client() -> Client {
    Client::with_uri_str(&CONFIG.common.database_ip)
        .expect("Failed to initialize client. Please check the uri first")
}

pub fn get_collection(coll_name: &str) -> Collection<Document> {
    MONGO.database("QAREALTIME").collection(coll_name)
}


pub fn update_qifi(qifi: QIFI) {
    let account_cookie = qifi.account_cookie.clone();
    let slice = struct_to_doc(qifi);
    if let Err(e) = get_collection("account")
        .update_one(doc! {"account_cookie": &account_cookie}, doc! {"$set": slice})
        .upsert(true)
        .run()
    {
        error!("MONGO {:?}", e);
    };
}