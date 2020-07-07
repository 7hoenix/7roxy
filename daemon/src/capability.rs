pub mod http {
    use std::error::Error;

    pub mod stack_exchange {
        pub enum Site {
            StackOverflow,
        }

        //
        // NOTE:
        // These fields are from [here](https://api.stackexchange.com/docs/wrapper)
        //

        pub struct Response<I> {
            backoff: Option<u64>,
            error_id: Option<u64>,
            error_message: Option<String>,
            error_name: Option<String>,
            has_more: bool,
            items: Vec<I>,
            page: u64,
            page_size: u64,
            quota_max: u64,
            quota_remaining: u64,
            total: u64,
            r#type: String,
        }
    }

    pub enum Target {
        StackExchange(stack_exchange::Site),
    }

    pub async fn make_request(search: String, target: Target) -> Result<(), Box<dyn Error>> {
        match target {
            Target::StackExchange(stack_exchange::Site::StackOverflow) => {
                let client = reqwest::Client::new();
                let params = [
                    ("order", "desc"),
                    ("sort", "activity"),
                    ("q", &search),
                    ("site", "stackoverflow"),
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
