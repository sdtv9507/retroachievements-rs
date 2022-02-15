use retroachievements_rs::RetroAPI;

fn main() {
    let user = String::from("youruser");
    let key = String::from("yourapikey");
    let retro = RetroAPI::new(user, key);
    let mut response = retro.request_top_users();
    println!("Top Users");
    println!("{}", response);
    response = retro.request_game(11246);
    println!("PSX Final Fantasy Tactics game");
    println!("{}", response);
}