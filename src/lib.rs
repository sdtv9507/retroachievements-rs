use reqwest;

pub struct RetroAPI {
    user: String,
    key: String,
    base: String,
}

impl RetroAPI {
    pub fn new(user: String, key: String) -> Self {
        RetroAPI {
            user,
            key,
            base: String::from("https://retroachievements.org/API/"),
        }
    }

    #[tokio::main]
    async fn get_request(&self, url: String) -> String {
        let url = format!("{}{}?z={}&y={}", self.base, url, self.user, self.key);
        let result = reqwest::get(url).await.unwrap();
        return result.text().await.unwrap();
    }

    #[tokio::main]
    async fn get_request_with_param(&self, url: String, param: String) -> String {
        let url = format!(
            "{}{}?z={}&y={}{}",
            self.base, url, self.user, self.key, param
        );
        let result = reqwest::get(url).await.unwrap();
        return result.text().await.unwrap();
    }

    pub fn request_top_users(&self) -> String {
        let url = String::from("API_GetTopTenUsers.php");
        let response = self.get_request(url);
        return response;
    }

    pub fn request_game(&self, game_id: i32) -> String {
        let url = String::from("API_GetGame.php");
        let game = format!("&i={}", game_id);
        let response = self.get_request_with_param(url, game);
        return response;
    }
}
