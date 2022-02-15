use retroachievements_rs::RetroAPI;

fn main() {
    let user = String::from("youruser");
    let key = String::from("yourapikey");
    let retroa = RetroAPI::new(user, key);
    let mut response = retroa.request_top_users();
    println!("Top Users");
    println!("{}", response);
    println!("PSX Final Fantasy Tactics game");
    response = retroa.request_game(11246);
    println!("{}", response);
}