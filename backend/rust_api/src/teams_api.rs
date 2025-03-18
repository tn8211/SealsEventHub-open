use rocket::{http::Status, serde::json::Json, State};
use crate::{auth::JwtToken, database::DbClient, model::{GenericResponse, TeamData, TeamMemberData}};

/*
    Create a team for a specific upcoming event
    The team creator gets added to the team as the captain
*/
#[post("/teams/create", data= "<team_data>")]
pub async fn create_new_team(_jwt_token: JwtToken, team_data: Json<TeamData>, db_client: &State<DbClient>) -> Result<Json<GenericResponse>, Status> {
    let client = db_client.client.lock().await;

    //Stop if the event doesn't exist, it has already started, the team name is taken or the creator is already in a team
    if !team_data.validate(&client).await {
        return Err(Status::BadRequest);
    }

    //Create the team and add the creator to it as captain
    let statement = client.prepare("
        INSERT INTO teams (team_name, event_id, created_by)
        VALUES ($1, $2, $3)
        RETURNING team_id
    ").await
    .map_err(|e| {
        eprint!("Error with statement preparation: {:?}", e);
        Status::InternalServerError
    })?;

    let rows = client.query_one(&statement, &[&team_data.name, &team_data.event_id, &_jwt_token.0.username]).await
        .map_err(|e| {
            eprint!("Error with query execution: {:?}", e);
            Status::InternalServerError
        })?;

    let statement = client.prepare("
        INSERT INTO participants (user_id, event_id, team_id, team_role)
        VALUES ($1, $2, $3, $4)
    ").await
        .map_err(|e| {
            eprint!("Error with statement preparation: {:?}", e);
            Status::InternalServerError
        })?;

    let team_id: i32;
    if let Some(row) = rows.get(0) {
        team_id = row;
    } else {
        return Err(Status::InternalServerError);
    }

    client.execute(&statement, &[&_jwt_token.0.username, &team_data.event_id, &team_id, &"captain"]).await
    .map_err(|e| {
        eprint!("Error with query execution: {:?}", e);
        Status::InternalServerError
    })?;

    return Ok(Json(GenericResponse {
        message: "Team created successfully".to_string()
    }));
}

/*
    TODO
    Create a join request to a team for a specific player
    Sender must be the team creator (captain)
    Team and Player are identified by id
*/
#[post("/teams/add-member", data = "<team_member_data>")]
pub async fn add_team_member_request(_jwt_token: JwtToken, team_member_data: Json<TeamMemberData>, db_client: &State<DbClient>) {
    let client = db_client.client.lock().await;


}

/*  
    TODO
    /teams/invites/kick
    Remove a player from a team and/or delete any join invite for that team the player may have  
    Sender must be the team creator (captain)
    Team and Player are identified by id
*/

/*
    TODO
    /teams/invites/accept
    Accept a join invite and officially join a team as a member/substitute
    Sender must be a player who has been sent the invite
    The invite expires after the event in question begins or when all slots are full for the role in question
    Team and Player are identified by id
*/

/*
    TODO
    /teams/invites/reject
    Deletes a join invite for a team
    Sender must be a player who has been sent the invite
    Team and Player are identified by id
*/
