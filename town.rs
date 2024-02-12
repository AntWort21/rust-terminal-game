use crate::PlayerStatus;

pub fn town_portal(x: usize, y: usize, player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status: PlayerStatus;
    if x == 16 && y == 9 {
        return_player_status = goods_store(player_status);
    } else if x == 24 && y == 9 {
        return_player_status = gear_store(player_status);
    } else if x == 24 && y == 14 {
        return_player_status = inn(player_status);
    } else if x == 16 && y == 14 {
        return_player_status = smith(player_status);
    } else if y == 24 {
        return_player_status = portal_south(player_status);
    } else {
        return_player_status = portal_west(player_status);
    }

    return_player_status
}

fn goods_store(player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status = player_status;

    return_player_status
}

fn gear_store(player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status = player_status;

    return_player_status
}

fn inn(player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status = player_status;

    return_player_status
}

fn smith(player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status = player_status;

    return_player_status
}

fn portal_south(player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status = player_status;

    return_player_status
}

fn portal_west(player_status: PlayerStatus) -> PlayerStatus {
    let mut return_player_status = player_status;

    return_player_status
}
