use gloo::net::http::Request;
use serde_json::Value;

pub async fn fetch_words(
    language: &str,
    capitalization: bool,
    punctuation: bool,
) -> Result<Vec<String>, ()> {
    let body = format!(
        "{{
            \"language\": \"{}\",
            \"count\": 50,
            \"with_uppercase\": {},
            \"with_punctuation\": {}
        }}",
        language, capitalization, punctuation
    );
    let res = Request::post("http://127.0.0.1:3000/words");

    let body = res
        .body(body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("Error connecting to the server.") // TODO: handle this unwrap
        .text()
        .await
        .unwrap(); // TODO: this one as well

    // TODO: handle these unwraps
    let body: Value = serde_json::from_str(&body).unwrap();
    let arr = body.as_array().unwrap();
    let arr = arr
        .iter()
        .map(|val| val.as_str().unwrap().to_string())
        .collect::<Vec<_>>();

    Ok(arr)
}
