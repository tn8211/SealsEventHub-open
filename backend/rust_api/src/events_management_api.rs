use rocket::{http::Status, serde::json::Json, State};
use tokio_postgres::types::ToSql;
use crate::{auth::JwtToken, database::DbClient, model::{EventRequestData, GenericResponse}, utils::get_channel_role};

#[get("/test")]
pub fn test() -> &'static str {
    "Test function called successfully"
}

/*
    Create a new event from an EventRequestData object
    User must be authenticated and authorized (admin or organizer)
*/
#[post("/events/create", data = "<event_data>")]
pub async fn create_new_event(_jwt_token: JwtToken, event_data: Json<EventRequestData>, db_client: &State<DbClient>) -> Result<Json<GenericResponse>, Status> {

    if event_data.validate() == false {
        return Err(Status::BadRequest);
    }

    let user_id = _jwt_token.0.user_id;
    let client = db_client.client.lock().await;

    let role = match get_channel_role(user_id, event_data.event_channel.unwrap(), &client).await {
        Ok(channel_role) => channel_role,
        Err(e) => return Err(e)
    };

    if !role.can_create_or_edit_event() {
        return Err(Status::Unauthorized);
    }

    let stmt = client.prepare("
        INSERT INTO events (
        name, 
        description, 
        event_type, 
        event_specific_category, 
        teams_size, 
        max_subs_amount, 
        capacity,
        start_date, 
        end_date, 
        location, 
        status, 
        created_by,
        event_channel
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        ").await
        .map_err(|e|{
            eprintln!("Error with statement preparation: {:?}", e);
            Status::InternalServerError
        })?;

    client.execute(&stmt, &[
        &event_data.name.as_ref().map(|s| s.trim()).unwrap_or("Somehow EventRequestData.validate has failes and a null value passed"),
        &event_data.description.as_ref().map(|s| s.trim()).unwrap_or("Somehow EventRequestData.validate has failes and a null value passed"),
        &event_data.event_type.as_ref().unwrap(),
        &event_data.event_specific_category.as_ref().unwrap(),
        &event_data.teams_size.as_ref().unwrap(),
        &event_data.max_subs_amount.as_ref().unwrap(),
        &event_data.capacity.as_ref().unwrap(),
        &event_data.start_date.as_ref().unwrap(),
        &event_data.end_date.as_ref().unwrap(),
        &event_data.location.as_ref().map(|s| s.trim()).unwrap_or("Somehow EventRequestData.validate has failes and a null value passed"),
        &event_data.status.as_ref().unwrap(),
        &user_id,
        &event_data.event_channel.as_ref().unwrap(),
        ]).await
        .map_err(|e|{
            eprintln!("Error with query execution: {:?}", e);
            Status::InternalServerError
        })?;

    return Ok(Json(GenericResponse {
        message: "Event created successfully".to_string()
    }));

}

/*
    Change an event's details
    cannot change the Channel the event is in
*/
#[put("/events/update", data = "<updated_event_data>")]
pub async fn update_existing_event(_jwt_token: JwtToken, updated_event_data: Json<EventRequestData>, db_client: &State<DbClient>) -> Result<Json<GenericResponse>, Status> {
    let client = db_client.client.lock().await;

    let user_id = _jwt_token.0.user_id;

    let statement = client.prepare("SELECT event_id, event_channel FROM events WHERE event_id = $1").await
        .map_err(|_| Status::InternalServerError)?;
    let rows = client.query(&statement, &[&updated_event_data.id]).await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(row) = rows.get(0) {
        let event_id: i32 = row.get(0);
        let channel: i32 = row.get(1);

        let role = match get_channel_role(user_id, channel, &client).await {
            Ok(channel_role) => channel_role,
            Err(e) => return Err(e)
        };

        if !role.can_create_or_edit_event() {
            return Err(Status::Unauthorized);
        }

        if updated_event_data.validate_for_updates() == false {
            return Err(Status::BadRequest);
        }
        
        let mut query = String::from("UPDATE events SET ");
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut param_index = 1;

        if let Some(name) = &updated_event_data.name {
            query.push_str(&format!("name = ${}, ", param_index));
            params.push(Box::new(name.trim().to_string()));
            param_index += 1;
        }
        if let Some(description) = &updated_event_data.description {
            query.push_str(&format!("description = ${}, ", param_index));
            params.push(Box::new(description.trim().to_string()));
            param_index += 1;
        }
        if let Some(event_type) = &updated_event_data.event_type {
            query.push_str(&format!("event_type = ${}, ", param_index));
            params.push(Box::new(event_type));
            param_index += 1;
        }
        if let Some(event_specific_category) = &updated_event_data.event_specific_category {
            query.push_str(&format!("event_specific_category = ${}, ", param_index));
            params.push(Box::new(event_specific_category));
            param_index += 1;
        }
        if let Some(start_date) = updated_event_data.start_date {
            query.push_str(&format!("start_date = ${}, ", param_index));
            params.push(Box::new(start_date));
            param_index += 1;
        }
        if let Some(end_date) = updated_event_data.end_date {
            query.push_str(&format!("end_date = ${}, ", param_index));
            params.push(Box::new(end_date));
            param_index += 1;
        }
        if let Some(location) = &updated_event_data.location {
            query.push_str(&format!("location = ${}, ", param_index));
            params.push(Box::new(location.trim().to_string()));
            param_index += 1;
        }
        if let Some(status) = &updated_event_data.status {
            query.push_str(&format!("status = ${}, ", param_index));
            params.push(Box::new(status));
            param_index += 1;
        }
        if let Some(teams_size) = &updated_event_data.teams_size {
            query.push_str(&format!("teams_size = ${}, ", param_index));
            params.push(Box::new(teams_size));
            param_index += 1;
        }
        if let Some(max_subs_amount) = &updated_event_data.max_subs_amount {
            query.push_str(&format!("max_subs_amount = ${}, ", param_index));
            params.push(Box::new(max_subs_amount));
            param_index += 1;
        }
        if let Some(capacity) = &updated_event_data.capacity {
            query.push_str(&format!("capacity = ${}, ", param_index));
            params.push(Box::new(capacity));
            param_index += 1;
        }

        query = query.trim_end_matches(", ").to_string();

        query.push_str(&format!(" WHERE event_id = ${}", param_index));
        params.push(Box::new(event_id));

        let statement = client.prepare(&query).await.map_err(|e| {
            eprintln!("Error preparing query: {:?}", e);
            Status::InternalServerError
        })?;

        let param_refs: Vec<&(dyn ToSql + Sync)> = params
        .iter()
        .map(|p| p.as_ref() as &(dyn ToSql + Sync))
        .collect();


        client.execute(&statement, &param_refs).await.map_err(|e| {
            eprintln!("Error executing query: {:?}", e);
            Status::InternalServerError
        })?;


    } else {
        return Err(Status::NotFound);
    }

    return Ok(Json(GenericResponse {
        message: "Event updated successfully".to_string()
    }));
}

/*
    Delete an event
    The sender must be the event's creator or a channel/website admin
*/
#[delete("/events/delete/<event_to_delete_id>")]
pub async fn delete_event(_jwt_token: JwtToken, event_to_delete_id: i32, db_client: &State<DbClient>) -> Result<Json<GenericResponse>, Status> {
    let client = db_client.client.lock().await;

    let user_id = _jwt_token.0.user_id;

    let statement = client.prepare("SELECT event_channel, created_by FROM events WHERE event_id = $1").await
        .map_err(|_| Status::InternalServerError)?;
    let rows = client.query(&statement, &[&event_to_delete_id]).await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(row) = rows.get(0) {
        let channel_id: i32 = row.get(0);
        let creator: i32 = row.get(1);

        let role = match get_channel_role(user_id, channel_id, &client).await {
            Ok(channel_role) => channel_role,
            Err(e) => return Err(e)
        };

        if !role.can_delete_event() && creator != user_id {
            return Err(Status::Unauthorized);
        }

        let statement = client.prepare(
            "DELETE FROM events WHERE event_id = $1"
        ).await
            .map_err(|_| Status::InternalServerError)?;

        client.execute(&statement, &[
            &event_to_delete_id,
            ]).await
            .map_err(|_| Status::InternalServerError)?;

    } else {
        return Err(Status::NotFound);
    }

    return Ok(Json(GenericResponse {
        message: "Event deleted successfully".to_string()
    }));
}