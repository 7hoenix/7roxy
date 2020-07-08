pub mod http {
    use reqwest::header;
    use serde::Deserialize;
    use std::error::Error;

    pub mod stack_exchange {
        use serde::Deserialize;
        use serde_json::Value;

        pub enum Site {
            StackOverflow,
        }

        //
        // NOTE:
        // These fields are from [here](https://api.stackexchange.com/docs/wrapper)
        //

        #[derive(Deserialize, Debug)]
        pub struct Response<i> {
            // backoff: Option<u64>,
            // error_id: Option<u64>,
            // error_message: Option<String>,
            // error_name: Option<String>,
            // has_more: bool,
            pub items: Vec<i>,
            // page: u64,
            // page_size: u64,
            // quota_max: u64,
            // quota_remaining: u64,
            // total: u64,
            // r#type: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Item {
            // tags: Vec<String>,
            // owner: Value,
            // title: String,
            // is_answered: bool,
            // view_count: u64,
            // closed_date: Value,
            // answer_count: u64,
            // score: u64,
            // last_activity_date: Value,
            // creation_date: Value,
            pub question_id: u64,
            // link: String,
            // closed_reason: String,
            // title: String,
        }

        #[derive(Deserialize, Debug)]
        pub struct Answer {
            pub answer_id: u64,
            pub body: String,
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

                let payload: Result<stack_exchange::Response<stack_exchange::Item>, reqwest::Error> =
                  response.json().await;

                match payload {
                    Ok(r) => {
                        let question_ids: Vec<String> = r.items.iter().map(|q| q.question_id.to_string()).collect();
                        let ids = question_ids.join(";");
                        let url: String = format!("https://api.stackexchange.com/2.2/questions/{}/answers", ids);
                        let params = [
                            ("filter", "withbody"),
                            ("site", "stackoverflow"),
                        ];
                        let request = client
                            .get(&url)
                            .query(&params);
                        let response = request.send().await?;
                        let payload: Result<stack_exchange::Response<stack_exchange::Answer>, reqwest::Error> = response.json().await;
                        println!("got payload, {:#?}", payload);
                    }
                    Err(e) => println!("parsing err, {}", e),
                }

                Ok(())
            }
        }
    }
}
