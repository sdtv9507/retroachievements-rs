use retroachievements_rs::RetroAPI;

fn main() {
    let user = String::from("yourname");
    let key = String::from("yourapi");
    let retroa = RetroAPI::new(user, key);
    let response = retroa.request_top_users();
    println!("{}", response);
}