use langdb_clust::messages::{
    CacheControl, CacheTtl, ClaudeModel, ContentBlock, MaxTokens, Message,
    MessagesRequestBody, Role, SystemPrompt, TextContentBlock,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load API key from environment
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY environment variable must be set");

    // Create client
    let client = langdb_clust::Client::from_api_key(langdb_clust::ApiKey::new(&api_key));

    // Create a system prompt with 1-hour cache control
    let system_prompt = SystemPrompt::from_text_blocks_with_cache_control(vec![
        ("You are a helpful assistant.", None), // No cache control for basic instruction
        (
            "You have access to the following information that should be cached for 1 hour: The weather in New York is currently sunny with a temperature of 72°F.",
            Some(CacheControl {
                _type: langdb_clust::messages::CacheControlType::Ephemeral,
                ttl: Some(CacheTtl::OneHour),
            }),
        ),
    ]);

    // Create a message with cache control
    let message = Message {
        role: Role::User,
        content: vec![ContentBlock::Text(
            TextContentBlock::new_with_cache_control(
                "What's the weather like in New York?",
                CacheControl {
                    _type: langdb_clust::messages::CacheControlType::Ephemeral,
                    ttl: Some(CacheTtl::OneHour),
                },
            ),
        )],
    };

    // Create request body
    let model = ClaudeModel::Claude3Sonnet20240229;
    let max_tokens = MaxTokens::new(1024, model)?;
    let request_body = MessagesRequestBody {
        model,
        max_tokens,
        messages: vec![message],
        system: Some(system_prompt),
        ..Default::default()
    };

    println!("Sending request with 1-hour cache control...");

    // Send the request
    let response = client.create_a_message(request_body).await?;

    println!("Response: {}", response.content[0].text());
    println!("Usage: {:?}", response.usage);

    // The response will include cache creation information if the extended cache TTL feature is used
    if let Some(cache_creation) = response.usage.cache_creation {
        println!("Cache creation details:");
        println!("  5m cache input tokens: {}", cache_creation.ephemeral_5m_input_tokens);
        println!("  1h cache input tokens: {}", cache_creation.ephemeral_1h_input_tokens);
    }

    Ok(())
}