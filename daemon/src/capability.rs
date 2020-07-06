pub mod http {
    use std::error::Error;

    pub enum Site {
        Site(String),
    }

    pub enum Target {
        StackExchange(Site),
    }

    pub async fn make_request(search: String, target: Target) -> Result<(), Box<dyn Error>> {
        match target {
            Target::StackExchange(Site::Site(raw_site)) => {
                println!("parsing?, {}", raw_site);
                let client = reqwest::Client::new();
                let params = [
                    ("order", "desc"),
                    ("sort", "activity"),
                    ("q", &search),
                    ("site", &raw_site),
                ];
                let request = client
                    .get("https://api.stackexchange.com/2.2/search/advanced")
                    .query(&params);

                let response = request.send().await?;

                let body = response.bytes().await?;

                let v = body.to_vec();

                let s = String::from_utf8_lossy(&v);
                println!("response, {:#}", s);
                Ok(())
            }
        }
    }
}
