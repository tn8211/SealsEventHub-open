use rocket::{http::Status, serde::json::Json, State};

use crate::{auth::JwtToken, database::DbClient, model::{ChannelRole, GenericResponse}, utils::get_channel_role};

/*
    Change a user's permissions for a specific channel by setting their role
    Sender must be a channel or website admin
*/
#[get("/channels/change-permissions/<channel_id>/<user_id>/<role>")]
pub async fn change_permissions(_jwt_token: JwtToken, channel_id: i32, user_id: i32, role: String, db_client: &State<DbClient>) -> Result<Json<GenericResponse>, Status> {

    let sender_id = _jwt_token.0.user_id;
    let target_id = user_id;
    let new_role = ChannelRole::from_str(&role);
    if new_role.is_none() {
        return Err(Status::BadRequest);
    }

    let client = db_client.client.lock().await;
    
    //Check sender's permissions
    let sender_role = match get_channel_role(sender_id, channel_id, &client).await {
        Ok(channel_role) => channel_role,
        Err(e) => return Err(e)
    };

    if !sender_role.can_edit_channel_permissions() {
        return Err(Status::Unauthorized);
    }

    //Find the target
    let statement = client.prepare("
    SELECT user_id FROM event_channel_roles WHERE user_id = $1 AND channel_id = $2
    ").await
    .map_err(|e| {
        eprint!("Error with statement preparation: {:?}", e);
        Status::InternalServerError
    })?;

    let res = client.query(&statement, &[&target_id, &channel_id]).await
    .map_err(|e| {
        eprint!("Error with query execution: {:?}", e);
        Status::InternalServerError
    })?;

    if res.get(0).is_none() {
        return Err(Status::NotFound);
    }

    //Update the role
    let statement = client.prepare("
    UPDATE event_channel_roles SET role = $1 WHERE user_id = $2 AND channel_id = $3
    ").await
    .map_err(|e| {
        eprint!("Error with statement preparation: {:?}", e);
        Status::InternalServerError
    })?;

    client.execute(&statement, &[&new_role.unwrap().to_string(), &target_id, &channel_id]).await
    .map_err(|e| {
        eprint!("Error with query execution: {:?}", e);
        Status::InternalServerError
    })?;


    return Ok(Json(GenericResponse {
        message: "Permissions changed successfully".to_string()
    }));
}


/*
    TODO:
    /ban?user
    Set a user's status to suspended
    Sender must be a website admin
    Node: ban will only take effect once the target's token expires, for now
*/