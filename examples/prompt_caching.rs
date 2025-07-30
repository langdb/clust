use langdb_clust::messages::ClaudeModel;
use langdb_clust::messages::MaxTokens;
use langdb_clust::messages::Message;
use langdb_clust::messages::MessagesRequestBody;
use langdb_clust::messages::MessagesRequestBuilder;
use langdb_clust::messages::SystemPrompt;
use langdb_clust::ApiKey;
use langdb_clust::ClientBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client with your API key
    let api_key = ApiKey::new("your-api-key-here");
    let client = ClientBuilder::new(api_key).build();

    let model = ClaudeModel::Claude3Sonnet20240229;
    let max_tokens = MaxTokens::new(1024, model)?;

    // Create a request body with prompt caching enabled
    let request_body = MessagesRequestBody {
        model,
        messages: vec![Message::user("What is the capital of France?")],
        max_tokens,
        system: Some(SystemPrompt::new("You are a helpful assistant.")),
        prompt_cache: Some(true), // Enable prompt caching
        ..Default::default()
    };

    println!("Sending request with prompt caching enabled...");
    
    // Send the request
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Response: {}", response.content);

    // Send the same request again - this should use the cached response
    let request_body_cached = MessagesRequestBody {
        model,
        messages: vec![Message::user("What is the capital of France?")],
        max_tokens,
        system: Some(SystemPrompt::new("You are a helpful assistant.")),
        prompt_cache: Some(true), // Enable prompt caching
        ..Default::default()
    };

    println!("\nSending the same request again (should use cache)...");
    
    let response_cached = client
        .create_a_message(request_body_cached)
        .await?;

    println!("Cached Response: {}", response_cached.content);

    // You can also use the builder pattern
    let request_body_builder = MessagesRequestBuilder::new(model)
        .messages(vec![Message::user("What is the capital of France?")])
        .system(SystemPrompt::new("You are a helpful assistant."))
        .max_tokens(max_tokens)
        .prompt_cache(true) // Enable prompt caching using builder
        .build();

    println!("\nSending request using builder pattern with prompt caching...");
    
    let response_builder = client
        .create_a_message(request_body_builder)
        .await?;

    println!("Builder Response: {}", response_builder.content);

    Ok(())
}