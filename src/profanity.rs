use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use std::env;

const REQWEST_MAX_RETRIES: u32 = 5;

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
    let api_key =
        env::var("BAD_WORDS_API_KEY").map_err(|_| handle_errors::Error::EnvVariableError)?;
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(REQWEST_MAX_RETRIES);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body(content)
        .send()
        .await
        .map_err(|err| handle_errors::Error::MiddlewareReqwestAPIError(err))?;
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
        Err(err) => Err(handle_errors::Error::ReqwestAPIError(err)),
    }
}

async fn transform_error(res: reqwest::Response) -> handle_errors::ApiLayerError {
    handle_errors::ApiLayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
