use reqwest;

pub struct RetroAPI {
    user: String,
    key: String,
    base: String,
    image: String,
}

impl RetroAPI {
    pub fn new(user: String, key: String) -> Self {
        RetroAPI {
            user,
            key,
            base: String::from("https://retroachievements.org/API/"),
            image: String::from("http://i.retroachievements.org"),
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

    #[tokio::main]
    pub async fn get_image(&self, url: String) -> String {
        let url = format!(
            "{}{}",
            self.image, url
        );
        println!("{}", url);
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

    pub fn request_game_extend(&self, game_id: i32) -> String {
        let url = String::from("API_GetGameExtended.php");
        let game = format!("&i={}", game_id);
        let response = self.get_request_with_param(url, game);
        return response;
    }

    pub fn request_console_id(&self) -> String {
        let url = String::from("API_GetConsoleIDs.php");
        let response = self.get_request(url);
        return response;
    }

    pub fn request_game_list(&self, console_id: i32) -> String {
        let url = String::from("API_GetGameList.php");
        let console = format!("&i={}", console_id);
        let response = self.get_request_with_param(url, console);
        return response;
    }

    pub fn request_feed(&self, user: String, count: i32, offset: i32) -> String {
        let url = String::from("API_GetFeed.php");
        let feed = format!("&u={}&c={}&o={}", user, count, offset);
        let response = self.get_request_with_param(url, feed);
        return response;
    }

    pub fn request_user_rank_score(&self, user: String) -> String {
        let url = String::from("API_GetUserRankAndScore.php");
        let user = format!("&u={}", user);
        let response = self.get_request_with_param(url, user);
        return response;
    }

    pub fn request_recently_played(&self, user: String, count: i32, offset: i32) -> String {
        let url = String::from("API_GetUserRecentlyPlayedGames.php");
        let param = format!("&u={}&c={}&o={}", user, count, offset);
        let response = self.get_request_with_param(url, param);
        return response;
    }

    pub fn request_summary(&self, user: String, games: i32) -> String {
        let url = String::from("API_GetUserSummary.php");
        let param = format!("&u={}&g={}&a=5", user, games);
        let response = self.get_request_with_param(url, param);
        return response;
    }

    pub fn request_game_info_progress(&self, user: String, game: i32) -> String {
        let url = String::from("API_GetGameInfoAndUserProgress.php");
        let param = format!("&u={}&g={}", user, game);
        let response = self.get_request_with_param(url, param);
        return response;
    }

    pub fn request_games_completed(&self, user: String) -> String {
        let url = String::from("API_GetUserCompletedGames.php");
        let param = format!("&u={}", user);
        let response = self.get_request_with_param(url, param);
        return response;
    }

    #[tokio::main]
    pub async fn request_id_from_hash(&self, hash: String) -> String {
        let url = format!("https://retroachievements.org/dorequest.php?r=gameid&m={}", hash);
        let result = reqwest::get(url).await.unwrap();
        let response = result.text().await.unwrap();
        return response;
    }
}
