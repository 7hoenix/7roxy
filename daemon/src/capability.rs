pub mod http {
    use libxml::parser::Parser;
    use libxml::xpath;
    use std::error::Error;
    use std::io;
    use std::io::Write;
    use std::process::Command;

    pub mod stack_exchange {
        use serde::Deserialize;

        pub enum Site {
            StackOverflow,
        }

        //
        // NOTE:
        // These types are from [here](https://api.stackexchange.com/docs/wrapper).
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

    fn process_answers(
        answers: stack_exchange::Response<stack_exchange::Answer>,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let stdin = io::stdin();
        let mut results: Vec<String> = Vec::new();
        for answer in answers.items {
            let parser = Parser::default_html();
            let document = parser
                .parse_string(answer.body.as_bytes())
                .expect("HTML document");
            let mut context = xpath::Context::new(&document).expect("HTML document context");

            for snippet in context
                .findnodes("//pre/code", None)
                .expect("XPath selector")
            {
                let content = snippet.get_content();
                println!("Found some code!");
                for (index, line) in content.lines().enumerate() {
                    println!("{: >3}: {}", index /* padded with spaces */, line);
                }
                print!("\n\nSearch GitHub for a line? Legend: s for next answer. Enter to skip snippet. ctrl-d to finish: ");
                io::stdout().flush().unwrap();
                let input = &mut String::new();
                stdin.read_line(input)?;
                if input.is_empty() {
                    return Ok(results); // Ctrl-D
                } else if input == "s\n" {
                    break;
                }
                match input.trim().parse::<usize>() {
                    Err(_) => {} // Just continue processing more answers!
                    Ok(i) => {
                        // Queue to be looked up
                        let line = content.lines().nth(i).expect("Line of code");
                        let url = format!("https://github.com/search?q={}&type=Code", line);
                        results.push(url);
                    }
                }
            }
        }
        Ok(results)
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
                let payload: stack_exchange::Response<stack_exchange::Question> = response
                    .json()
                    .await
                    .expect("StackOverflow questions HTTP response");
                let question_ids: Vec<String> = payload
                    .items
                    .iter()
                    .map(|q| q.question_id.to_string())
                    .collect();
                let ids = question_ids.join(";");
                let url: String = format!(
                    "https://api.stackexchange.com/2.2/questions/{}/answers",
                    ids
                );
                let params = [("filter", "withbody"), ("site", "stackoverflow")];

                let request = client.get(&url).query(&params);
                let response = request.send().await?;
                let answers: stack_exchange::Response<stack_exchange::Answer> =
                    response.json().await?;
                let results = process_answers(answers)?;

                //
                // NOTE:
                // The "open" command is likely MacOS specific.
                //
                Command::new("open").args(results).output()?;

                Ok(())
            }
        }
    }
}
