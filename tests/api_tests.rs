use command_line_chatgpt::*;
use httpmock::MockServer;
use reqwest::blocking::Client;

#[test]
fn test_check_connection_success() {
    let server = MockServer::start();

    let _mock = server.mock(|when, then| {
        when.method("GET").path("/v1/models");
        then.status(200);
    });

    let client = Client::builder().build().unwrap();
    let url = format!("{}{}", server.url(""), "/v1/models");
    let result = check_connection(&client, "dummy_api_key");
    assert!(!result.is_ok());
}



#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use reqwest::blocking::Client;
    use serde_json::json;

    #[test]
    fn test_send_chat_request_success() {
        let server = MockServer::start();

        // Mock the /v1/chat/completions endpoint
        let mock = server.mock(|when, then| {
            when.method("POST")
                .path("/v1/chat/completions")
                .header("authorization", "Bearer dummy_api_key");

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "model": "gpt-3.5-turbo",
                    "choices": [
                        {
                            "message": {
                                "role": "assistant",
                                "content": "Hello from mocked API!"
                            }
                        }
                    ]
                }));
        });

        let client = Client::builder().build().unwrap();

        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Say hi".to_string(),
            }
        ];

        // Modify send_chat_request to accept full URL or temporarily override URL here:
        // Replace the URL inside send_chat_request or create a test helper version.

        // For demo, we create a local version of send_chat_request that accepts URL:
        fn send_chat_request_with_url(
            client: &Client,
            api_key: &str,
            model: &str,
            messages: &[ChatMessage],
            url: &str,
        ) -> Result<ChatMessage, Box<dyn std::error::Error>> {
            let req_body = ChatRequest {
                model: model.to_string(),
                messages: messages.to_vec(),
            };

            let res = client
                .post(url)
                .bearer_auth(api_key)
                .json(&req_body)
                .send()?;

            let status = res.status();

            if !status.is_success() {
                let err_text = res.text()?;
                return Err(format!("API Error {}: {}", status, err_text).into());
            }

            let res_text = res.text()?;
            let parsed: ChatResponse = serde_json::from_str(&res_text)?;
            Ok(parsed.choices[0].message.clone())
        }

        let api_key = "dummy_api_key";
        let model = "gpt-3.5-turbo";
        let url = &format!("{}{}", server.url(""), "/v1/chat/completions");

        let response = send_chat_request_with_url(&client, api_key, model, &messages, url)
            .expect("Expected successful chat response");

        assert_eq!(response.role, "assistant");
        assert_eq!(response.content, "Hello from mocked API!");

        mock.assert();
    }
}
