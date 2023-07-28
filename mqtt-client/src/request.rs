use serde::Deserialize;

#[derive(Deserialize, Debug)]
// Struct to hold the fetched data
pub struct Fact {
    fact: String,
}

pub async fn request()->Result<String, Box<dyn std::error::Error>>{

    // Get data from the website
    let response = reqwest::get("https://catfact.ninja/fact").await?;

    // Deserialize the JSON response into the Fact struct
    let fact:Fact = response.json().await?;

    Ok(fact.fact)
}
