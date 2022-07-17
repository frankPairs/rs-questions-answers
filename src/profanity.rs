use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadWord {
    pub original: String,
    pub word: String,
    pub deviations: i64,
    pub info: i64,
    #[serde(rename = "replacedLen")]
    pub replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BadWordsResponse {
    pub content: String,
    pub bad_words_total: i64,
    pub bad_words_list: Vec<BadWord>,
    pub censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, handle_errors::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "H1WemIqKY1WNOTICqeIWV3YgH7XgwTrY")
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    let res_status = res.status();

    if !res_status.is_success() {
        let err = transform_error(res).await;

        if res_status.is_client_error() {
            return Err(handle_errors::Error::ClientError(err));
        }

        return Err(handle_errors::Error::ServerError(err));
    }

    let json_response = res.json::<BadWordsResponse>().await;

    match json_response {
        Ok(res) => Ok(res.censored_content),
        Err(err) => Err(handle_errors::Error::ExternalAPIError(err)),
    }
}

async fn transform_error(res: reqwest::Response) -> handle_errors::ApiLayerError {
    handle_errors::ApiLayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
