pub struct PlayerStatus {
    pub status_points: i32,
    pub strength: i32,
    pub vitality: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub dexterity: i32,
    pub luck: i32,
    pub nickname: String,
    pub money: i32,
    pub level: i32,
    pub current_hp: i32,
    pub max_hp: i32,
    pub current_sp: i32,
    pub max_sp: i32,
    pub job: String,
    pub xp: i32,
    pub required_xp: i32,
}

pub fn initialize_player_status() -> PlayerStatus {
    let player_status = PlayerStatus {
        status_points: 24,
        strength: 0,
        vitality: 0,
        agility: 0,
        intelligence: 0,
        dexterity: 0,
        luck: 0,
        nickname: "".to_string(),
        money: 250,
        level: 1,
        current_hp: 100,
        max_hp: 100,
        current_sp: 100,
        max_sp: 100,
        job: "Freelancer".to_string(),
        xp: 0,
        required_xp: 10,
    };

    player_status
}

pub fn preset_player_status() -> PlayerStatus {
    let player_status = PlayerStatus {
        status_points: 0,
        strength: 6,
        vitality: 2,
        agility: 6,
        intelligence: 2,
        dexterity: 6,
        luck: 2,
        nickname: "Testing".to_string(),
        money: 250,
        level: 1,
        current_hp: 100,
        max_hp: 100,
        current_sp: 100,
        max_sp: 100,
        job: "Game Master".to_string(),
        xp: 0,
        required_xp: 10,
    };

    player_status
}
