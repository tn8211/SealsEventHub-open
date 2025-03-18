use chrono::{NaiveDateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GenericResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventRequestData {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub event_specific_category: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub location: Option<String>,
    pub status: Option<String>,
    pub created_by: Option<i32>,
    pub teams_size: Option<i32>,
    pub max_subs_amount: Option<i32>,
    pub capacity: Option<i32>,
    pub event_channel: Option<i32>,
}

impl EventRequestData {
    pub fn validate(&self) -> bool {
        self.name.is_some() && !self.name.as_ref().unwrap().is_empty() &&
        self.description.is_some() && !self.description.as_ref().unwrap().is_empty() &&
        self.event_type.is_some() && !self.event_type.as_ref().unwrap().is_empty() &&
        self.event_specific_category.is_some() && !self.event_specific_category.as_ref().unwrap().is_empty() &&
        self.teams_size.is_some() && self.teams_size > Some(0) &&
        self.max_subs_amount.is_some() && self.max_subs_amount >= Some(0) &&
        self.capacity.is_some() && self.capacity > Some(1) &&
        self.start_date.is_some() && self.start_date > Some(Utc::now().naive_local()) &&
        self.end_date.is_some() && self.end_date > self.start_date &&
        self.location.is_some() && !self.location.as_ref().unwrap().is_empty() &&
        self.status.is_some() && !self.status.as_ref().unwrap().is_empty() &&
        self.event_channel.is_some()
    }

    //Check validity of the non-null fields only
    pub fn validate_for_updates(&self) -> bool {
        if self.name.is_some() && self.name.as_ref().unwrap().is_empty() {return false;}
        if self.description.is_some() && self.description.as_ref().unwrap().is_empty() {return false;}
        if self.event_type.is_some() && self.event_type.as_ref().unwrap().is_empty() {return false;}
        if self.event_specific_category.is_some() && self.event_specific_category.as_ref().unwrap().is_empty() {return false;}
        if self.teams_size.is_some() && !(self.teams_size > Some(0)) {return false;}
        if self.max_subs_amount.is_some() && !(self.max_subs_amount >= Some(0)) {return false;}
        if self.capacity.is_some() && !(self.capacity > Some(1)) {return false;}
        if self.start_date.is_some() && !(self.start_date > Some(Utc::now().naive_local())) {return false;}
        if self.end_date.is_some() && !(self.end_date > self.start_date) {return false;}
        if self.location.is_some() && self.location.as_ref().unwrap().is_empty() {return  false;}
        if self.status.is_some() && self.status.as_ref().unwrap().is_empty() {return false;}
        if self.event_channel.is_some() {return false;} //Do not allow to change the channel the event was published on
        return true;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TeamData {
    pub id: i32,
    pub name: String,
    pub event_id: i32,
    pub created_by: i32,
}

impl TeamData {
    pub async fn validate(&self, client: &rocket::tokio::sync::MutexGuard<'_, tokio_postgres::Client>) -> bool {

        if self.name.trim().is_empty() {
            return false;
        }

        //Check if the event exists and hasn't started yet
        let statement = match client.prepare("SELECT event_id, status FROM events WHERE event_id = $1").await {
            Ok(statement) => {statement},
            Err(e) => {
                eprintln!("Error with query preparation: {:?}", e);
                return false;
            },
        };

        let rows = match client.query(&statement, &[&self.event_id]).await {
            Ok(statement) => {statement},
            Err(e) => {
                eprintln!("Error with query preparation: {:?}", e);
                return false;
            },
        };

        if let Some(row) = rows.get(0) {
            let event_status: String = row.get(1);
            if event_status != "upcoming" {
                return false
            }
            return true;
        }

        ////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //
        //TODO: check if the team name already exists and if the creator is already in a team
        //
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////

        return false;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TeamMemberData {
    pub team_id: i32,
    pub player_id: i32,
    pub player_role: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct  ChannelData {
    pub channel_id: Option<i32>,
    pub channel_name: Option<String>,
    pub channel_description: Option<String>,
}

impl ChannelData {
    pub fn validate_for_creation(&self) -> bool {
        self.channel_name.is_some() && !self.channel_name.as_ref().unwrap().is_empty() &&
        self.channel_description.is_some() && !self.channel_description.as_ref().unwrap().is_empty()
    }

    //Needs either a change in name or in description
    pub  fn validate_for_updates(&self) -> bool {
        if self.channel_id.is_none() {return false;}
        if self.channel_name.is_some() && self.channel_name.as_ref().unwrap().is_empty() {return false;} 
        if self.channel_description.is_some() && self.channel_description.as_ref().unwrap().is_empty() {return false;}

        //Make sure fully empty updates don't pass 
        if self.channel_name.is_none() && self.channel_description.is_none() {return false;}
        return true;
    }
}

pub enum WebsiteRole {
    Admin,
    User,
}

impl WebsiteRole {
    pub fn to_string(&self) -> String {
        match *self {
            WebsiteRole::Admin => "admin".to_string(),
            WebsiteRole::User => "user".to_string(),
        } 
    }

    pub fn from_str(role_str: &str) -> Option<WebsiteRole> {
        match role_str {
            "admin" => Some(WebsiteRole::Admin),
            "user" => Some(WebsiteRole::User),
            _ => None,
        }
    }
}

pub enum ChannelRole {
    SuperAdmin,
    Admin,
    Organizer,
    User,
}

impl ChannelRole {
    pub fn to_string(&self) -> String {
        match *self {
            ChannelRole::SuperAdmin => "superadmin".to_string(),
            ChannelRole::Admin => "admin".to_string(),
            ChannelRole::Organizer => "organizer".to_string(),
            ChannelRole::User => "user".to_string(),
        } 
    }

    pub fn from_str(role_str: &str) -> Option<ChannelRole> {
        match role_str {
            "admin" => Some(ChannelRole::Admin),
            "organizer" => Some(ChannelRole::Organizer),
            "user" => Some(ChannelRole::User),
            _ => None,
        }
    }

    pub fn can_create_or_edit_event(&self) -> bool {
        match *self {
            ChannelRole::SuperAdmin => true,
            ChannelRole::Admin => true,
            ChannelRole::Organizer => true,
            _ => false
        }
    }

    pub fn can_delete_event(&self) -> bool {
        match *self {
            ChannelRole::SuperAdmin => true,
            ChannelRole::Admin => true,
            _ => false
        }
    }

    pub fn can_edit_channel(&self) -> bool {
        match *self {
            ChannelRole::SuperAdmin => true,
            ChannelRole::Admin => true,
            _ => false
        }
    }

    pub fn can_delete_channel(&self) -> bool {
        match *self {
            ChannelRole::SuperAdmin => true,
            ChannelRole::Admin => true,
            _ => false
        }
    }

    pub fn can_edit_channel_permissions(&self) -> bool {
        match *self {
            ChannelRole::SuperAdmin => true,
            ChannelRole::Admin => true,
            _ => false
        }
    }
}

