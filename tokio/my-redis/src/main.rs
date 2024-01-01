use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("Dylan", "Bard".into()).await?;

    let character_name = "Dylan";
    let result = client.get(character_name).await?;

    println!("{character_name} is {:?}", result);

    Ok(())
}
