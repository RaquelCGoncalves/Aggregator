mod request;

#[tokio::main]
async fn main() {
    let result = request::request().await;
    match result {
            Ok(fact) => {
                println!("{:?}", fact);
            }
            Err(err) => {
                println!("{:?}", err);

            }
    }

}