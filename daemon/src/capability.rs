pub mod http {
    use std::error::Error;
    use libxml::parser::Parser;
    use libxml::xpath;

    pub mod stack_exchange {
        use serde::Deserialize;

        pub enum Site {
            StackOverflow,
        }

        //
        // NOTE:
        // These fields are from [here](https://api.stackexchange.com/docs/wrapper)
        //

        #[derive(Deserialize, Debug)]
        pub struct Response<I> {
            pub items: Vec<I>,
        }

        #[derive(Deserialize, Debug)]
        pub struct Question {
            pub question_id: u64,
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
                    ("accepted", "true"),
                    ("closed", "false"),
                ];
                let request = client
                    .get("https://api.stackexchange.com/2.2/search/advanced")
                    .query(&params);
                let response = request.send().await?;
                let payload: stack_exchange::Response<stack_exchange::Question> =
                  response.json().await.expect("StackOverflow questions HTTP response");
                let question_ids: Vec<String> = payload.items.iter().map(|q| q.question_id.to_string()).collect();
                let ids = question_ids.join(";");
                let url: String = format!("https://api.stackexchange.com/2.2/questions/{}/answers", ids);
                let params = [
                    ("filter", "withbody"),
                    ("site", "stackoverflow"),
                ];


                let request = client.get(&url).query(&params);
                let response = request.send().await?;
                let answers: stack_exchange::Response<stack_exchange::Answer> = response.json().await?;
                for answer in answers.items {
                    let parser = Parser::default_html();
                    let document = parser.parse_string(answer.body.as_bytes()).expect("HTML document");
                    let mut context = xpath::Context::new(&document).expect("HTML document context");
                    let snippets = context.findnodes("//pre/code", None).expect("XPath selector");
                    let code_blocks: Vec<String> = snippets.iter().map(|s| s.get_content()).collect();
                    println!("{}", code_blocks.join("\n"));
                    println!("--->");
                };

                Ok(())
            }
        }
    }
}
