use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Fact {
    fact: String,
}

pub async fn request()->Result<Fact, Box<dyn Error>>   {
    let response = reqwest::get("https://catfact.ninja/fact").await?;
    let fact:Fact = response.json().await?;

    Ok(fact)
}