pub struct Supabase {
    url: String,
    api_key: String,
}

impl Supabase {

    pub fn new(url: String, api_key: String) -> Self {
        Self { url, api_key }
    }

    pub async fn create_user() -> anyhow::Result<()> {
        let client = self.build_client();
        let request_url = format!("{}/auth/v1/signup", &self.url);

        let json = json!({
            "email": "evan",
        });

        let response = self.send_post_request(client, request_url, json).await.unwrap();

        Ok(())
    }

    fn build_client(&self) -> reqwest::blocking::Client {
        let builder = reqwest::blocking::ClientBuilder::new();
        let client = builder.user_agent(&self.user_agent).build().unwrap();
        client
    }

    async fn send_post_request(
        &self,
        client: reqwest::Client,
        request_url: String,
        json: serde_json::Value,
    ) -> anyhow::Result<()> {
        println!("> Sending POST request to: {}", request_url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "apikey",
            reqwest::header::HeaderValue::from_str(&self.api_key)?,
        );
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", &self.api_key))?,
        );

        let request = client
            .post(request_url)
            .headers(headers)
            .json(&json)
            .send()
            .await?;

        Ok(response)
    }
}
