use mongodb::{ bson::{doc, oid::ObjectId, Document}, options::{ClientOptions, FindOneOptions, FindOptions}, Client, Database };
use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData {
    pub username : String,
    pub password : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserId {
    _id: ObjectId,
}

pub async fn connect_to_database() -> Database {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await;

    let mut client_options = match client_options {
        Ok(co) => co,
        Err(err) => {
            panic!("client options parse failed: {}", err);
        }
    };

    client_options.app_name = Some("silent_server".to_string());

    let client = Client::with_options(client_options);

    let client = match client {
        Ok(c) => c,
        Err(err) => {
            panic!("client creation faild: {}", err);
        }
    };

    client.database("silent_sedation_db") 
}

pub async fn list_collections(db: &Database) {
    match db.list_collection_names(None).await {
        Ok(collections) => {
            for collection_name in collections.iter() {
                println!("{}", collection_name);
            }
        },
        Err(e) => {
            eprintln!("Collection request failed: {}", e);
        }
    }
}

pub async fn get_uesr_id(db: &Database, login_data: LoginData) -> Option<UserId> {
    let user_data_collection = db.collection::<UserId>("users_data");
    let filter = doc! { "username" : login_data.username, "password" : login_data.password };
    let find_options = FindOneOptions::builder().projection(doc! {"_id": 1}).build();
    let find_result = user_data_collection.find_one(filter, find_options).await;

     match find_result {
        Ok(fr) => fr,
        Err(e) => {
            eprintln!("getting user id failed: {}", e);
            return None;
        }
    }    
}

pub async fn get_user_basic_info() {

}

pub async fn get_card() {

}

pub async fn update_card() {

}

pub async fn save_new_card() {

}

pub async fn delete_card() {

}