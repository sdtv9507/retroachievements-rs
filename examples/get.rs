use retroachievements_rs::RetroAPI;

fn main() {
    let user = String::from("youruser");
    let key = String::from("yourapikey");
    let retro = RetroAPI::new(user, key);
    let mut response = retro.request_top_users();
    println!("Top Users");
    println!("{}", response);
    let hash = String::from("4af22b114a64db19e3e707b28ebb6e68");
    response = retro.request_id_from_hash(hash);
    println!("PSX Final Fantasy Tactics game id from its hash");
    println!("{}", response);
    response = retro.request_game(11246);
    println!("PSX Final Fantasy Tactics game data");
    println!("{}", response);
    response = retro.request_user_rank_score(String::from("sdtv9507"));
    println!("User rank score");
    println!("{}", response);
    response = retro.request_game_info_progress(String::from("sdtv9507"), 11246);
    println!("User progress on Final Fantasy Tactics");
    println!("{}", response);
}