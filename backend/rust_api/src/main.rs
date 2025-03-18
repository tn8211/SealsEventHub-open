use database::DbClient;
use rocket_cors::CorsOptions;

#[macro_use] extern crate rocket;

mod events_management_api;
mod database;
mod auth;
mod model;
mod utils;
mod teams_api;
mod channels_api;
mod permissions_management_api;

#[launch]
async fn rocket() -> _ {

    let db_client = DbClient::connect().await.unwrap();

    rocket::build()
        .attach(CorsOptions::default().to_cors().unwrap())
        .mount("/", routes![])
        .mount("/", routes![auth::protected_route])
        .mount("/", routes![auth::login])
        .mount("/", routes![auth::signup])
        .mount("/", routes![events_management_api::test])
        .mount("/", routes![events_management_api::create_new_event])
        .mount("/", routes![events_management_api::update_existing_event])
        .mount("/", routes![events_management_api::delete_event])
        .mount("/", routes![teams_api::create_new_team])
        .mount("/", routes![channels_api::create_new_channel])
        .mount("/", routes![channels_api::edit_channel])
        .mount("/", routes![channels_api::delete_channel])
        .mount("/", routes![channels_api::join_channel])
        .mount("/", routes![channels_api::leave_channel])
        .mount("/", routes![permissions_management_api::change_permissions])
        .manage(db_client)
}
