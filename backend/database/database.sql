-- Connect to the PostgreSQL database
\c postgres_db;

CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) CHECK(role IN ('admin', 'user')) DEFAULT 'user',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ---TODO: ADD STATUS TO SUSPEND/BAN USERS 
);

CREATE TABLE Events (
    event_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    event_type VARCHAR(50) CHECK(event_type IN ('tournament', 'workshop', 'conference', 'other')) NOT NULL,
    event_specific_category VARCHAR(50) NOT NULL,
    event_group VARCHAR(50) NOT NULL,
    start_date TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    location VARCHAR(255),
    players_in_teams INT NOT NULL,
    subs_in_teams INT NOT NULL,
    capacity INT NOT NULL,
    status VARCHAR(50) CHECK(status IN ('upcoming', 'ongoing', 'completed', 'canceled')) DEFAULT 'upcoming',
    event_channel INT NOT NULL,
    created_by INT REFERENCES Users(user_id) ON DELETE SET NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Teams (
    team_id SERIAL PRIMARY KEY,
    team_name VARCHAR(255) NOT NULL,
    event_id INT REFERENCES Events(event_id) ON DELETE CASCADE,
    created_by INT REFERENCES Users(user_id) ON DELETE SET NULL,
    status VARCHAR(50) CHECK(status IN ('pending', 'accepted', 'rejected')) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Channels (
    channel_id SERIAL PRIMARY KEY,
    channel_name VARCHAR(50) NOT NULL,
    channel_description TEXT,
    created_by INT REFERENCES Users(user_id) ON DELETE SET NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE event_invites (
    event_id INT PRIMARY KEY,
    sender_id INT NOT NULL,
    team_id INT NOT NULL,
    player_id INT NOT NULL,
    status VARCHAR(10) CHECK (status IN ('pending', 'accepted', 'rejected', 'invalid')) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Foreign key constraints
    FOREIGN KEY (event_id) REFERENCES Events(event_id) ON DELETE CASCADE,
    FOREIGN KEY (sender_id) REFERENCES Users(user_id) ON DELETE SET NULL,
    FOREIGN KEY (team_id) REFERENCES Teams(team_id) ON DELETE CASCADE
);

CREATE TABLE event_channel_roles (
    user_id INT REFERENCES Users(user_id) ON DELETE CASCADE,
    channel_id INT REFERENCES Channels(channel_id) ON DELETE CASCADE,
    role VARCHAR(50) CHECK(role IN ('admin', 'organizer', 'user')) DEFAULT 'user',
    PRIMARY KEY (channel_id, user_id),
    CONSTRAINT fk_event FOREIGN KEY (channel_id) REFERENCES Channels(channel_id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES Users(user_id) ON DELETE CASCADE
);


CREATE TABLE Participants (
    participant_id SERIAL PRIMARY KEY,
    user_id INT REFERENCES Users(user_id) ON DELETE CASCADE,
    event_id INT REFERENCES Events(event_id) ON DELETE CASCADE,
    team_id INT REFERENCES Teams(team_id) ON DELETE CASCADE,
    registration_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(50) CHECK(status IN ('registered', 'confirmed', 'eliminated', 'completed')) DEFAULT 'registered'
    team_role VARCHAR(50) CHECK(status IN ('player', 'captain', 'substitute')) DEFAULT 'player'
);

CREATE TABLE Matches (
    match_id SERIAL PRIMARY KEY,
    event_id INT REFERENCES Events(event_id) ON DELETE CASCADE,
    
    -- Nullable references to teams and participants
    team1_id INT REFERENCES Teams(team_id) ON DELETE CASCADE,
    team2_id INT REFERENCES Teams(team_id) ON DELETE CASCADE,
    participant1_id INT REFERENCES Participants(participant_id) ON DELETE CASCADE,
    participant2_id INT REFERENCES Participants(participant_id) ON DELETE CASCADE,
    
    -- Match metadata
    match_date TIMESTAMP,
    score_team1 INT DEFAULT 0,
    score_team2 INT DEFAULT 0,
    score_participant1 INT DEFAULT 0,
    score_participant2 INT DEFAULT 0,
    
    -- Status and winner
    status VARCHAR(50) CHECK(status IN ('scheduled', 'completed')) DEFAULT 'scheduled',
    winner_team_id INT REFERENCES Teams(team_id) ON DELETE SET NULL,
    winner_participant_id INT REFERENCES Participants(participant_id) ON DELETE SET NULL,

    -- Match type (team vs team, individual vs individual, team vs individual)
    match_type VARCHAR(50) CHECK(match_type IN ('Teams', 'Solo'))
);