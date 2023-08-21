use reqwest; // web requests
             // use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use serde::{Deserialize, Serialize};
use serde_json;
use dotenv;

use crate::parse_cdn::CdnValues;
//use parse_cdn::CdnValues;

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    client_id: &'a String,
    client_secret: &'a String,
    //scope: &'a str,
    resource: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    #[serde(deserialize_with = "string_to_i32")]
    expires_in: i32,
}

fn string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}


pub async fn get_oauth_token() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let token_request = TokenRequest {
        grant_type: "client_credentials",
        client_id: &std::env::var("CLIENT_ID").expect(".env must contain CLIENT_ID for the app."), //App id "Verizon-cdn-whitelist-retrieval"
        client_secret: &std::env::var("CLIENT_SECRET").expect(".env must contain CLIENT_SECRET for the app."),
        resource: "https://management.azure.com/",
        //scope: "https://graph.microsoft.com/.default",
    };

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://login.microsoftonline.com/{}/oauth2/token", &std::env::var("TENANT_ID").expect(".env must contain TENNANT_ID for the app.")
        ))
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        )
        .form(&token_request)
        .send()
        .await
        .expect("Failed to send request");

    let status = response.status();
    println!("Debug {}", status);

    let response_decode = response
        .json::<TokenResponse>()
        .await
        .expect("Failed to decode token response");
    Ok(response_decode.access_token)
}

// async fn get_edgenode_ips() -> Result<String, Box<dyn std::error::Error>> {
pub async fn get_edgenode_ips(token: &str) -> Result<CdnValues, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://management.azure.com/providers/Microsoft.Cdn/edgenodes?api-version=2023-05-01"
        ))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to fetch cdn ips");

    let status = response.status();
    println!("Debug {}", status);

    let body_text = response.text().await.expect("Failed to get response text");

    println!("Debug_response {:?}", body_text);
    println!();

    //let response_text = response.text().await?;
    let cdn: CdnValues = serde_json::from_str(&body_text).expect("Failed to decode cdn response");
    Ok(cdn)
}