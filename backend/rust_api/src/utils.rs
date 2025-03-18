use rocket::http::Status;

use crate::model::{ChannelRole, WebsiteRole};

pub async fn get_website_role(user_id: i32, client: &rocket::tokio::sync::MutexGuard<'_, tokio_postgres::Client>) -> Result<WebsiteRole, Status> {
    
    let statement = client.prepare("
    SELECT role FROM users WHERE user_id = $1
    ").await
    .map_err(|e| {
        eprintln!("Error with statement preparation: {:?}", e);
        Status::InternalServerError
    })?;

    let rows = client.query(&statement, &[&user_id]).await.
    map_err(|e| {
        eprintln!("Error with Error with query execution: {:?}", e);
        Status::InternalServerError
    })?;

    if let Some(row) = rows.get(0) {
        let role = WebsiteRole::from_str(row.get(0));
        match role {
            Some(role) => return Ok(role),
            None => {
                eprintln!("Error: failed to unwrap a WebsiteRole item in function get_website_role");
                return Err(Status::InternalServerError);
            }
        }
    } else {
        return Err(Status::InternalServerError)
    }

}

pub async fn get_channel_role(user_id: i32, channel_id: i32, client: &rocket::tokio::sync::MutexGuard<'_, tokio_postgres::Client>) -> Result<ChannelRole, Status> {
    
    match get_website_role(user_id, &client).await {
        Ok(WebsiteRole::Admin) => return Ok(ChannelRole::SuperAdmin),
        Err(e) => return Err(e),
        _ => ()
    }
    
    let statement = client.prepare("
    SELECT role FROM event_channel_roles WHERE user_id = $1 AND channel_id = $2
    ").await
    .map_err(|e| {
        eprintln!("Error with statement preparation: {:?}", e);
        Status::InternalServerError
    })?;

    let rows = client.query(&statement, &[&user_id, &channel_id]).await.
    map_err(|e| {
        eprintln!("Error with Error with query execution: {:?}", e);
        Status::InternalServerError
    })?;

    if let Some(row) = rows.get(0) {
        let role = ChannelRole::from_str(row.get(0));
        match role {
            Some(role) => return Ok(role),
            None => {
                eprintln!("Error: failed to unwrap a ChannelRole item in function get_channel_role");
                return Err(Status::InternalServerError);
            }
        }
    } else {
        return Err(Status::NotFound);
    }
}
