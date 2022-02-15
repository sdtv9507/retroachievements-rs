use retroachievements_rs::api::RetroAPI;

fn main() {
    let user = String::from("yourname");
    let key = String::from("yourapi");
    let retroa = RetroAPI::new(user, key);
    retroa.request_top_users();
}