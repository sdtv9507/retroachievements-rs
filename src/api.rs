use reqwest;

pub struct RetroAPI {
    user: String,
    key: String,
}

impl RetroAPI {
    pub fn new(user: String, key: String) -> Self {
        RetroAPI {
            user,
            key,
        }
    }

    #[tokio::main]
    pub async fn request_top_users(&self) {
        let base = "https://retroachievements.org/API/";
        let top_users = "API_GetTopTenUsers.php";
        let url = format!("{}{}?z={}&y={}", base, top_users, self.user, self.key);
        let result = reqwest::get(url).await.unwrap();
        println!("{}", result.text().await.unwrap());
    }
}