use langdb_clust::messages::{
    CacheControl, ClaudeModel, ContentBlock, MaxTokens, Message, MessagesRequestBody,
    SystemPrompt, TextContentBlock,
};
use langdb_clust::ApiKey;
use langdb_clust::ClientBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client with your API key
    let api_key = ApiKey::new("your-api-key-here");
    let client = ClientBuilder::new(api_key).build();

    let model = ClaudeModel::Claude3Sonnet20240229;
    let max_tokens = MaxTokens::new(1024, model)?;

    // Create an advanced system prompt with cache control
    // This matches the curl example you provided
    let system_prompt = SystemPrompt::from_text_blocks_with_cache_control(vec![
        (
            "You are an AI assistant tasked with analyzing literary works. Your goal is to provide insightful commentary on themes, characters, and writing style.\n",
            None, // No cache control for the instruction
        ),
        (
            "<the entire contents of Pride and Prejudice>",
            Some(CacheControl::default()), // Cache this content with ephemeral cache control
        ),
    ]);

    // Create the request body
    let request_body = MessagesRequestBody {
        model,
        messages: vec![Message::user("Analyze the major themes in Pride and Prejudice.")],
        max_tokens,
        system: Some(system_prompt),
        ..Default::default()
    };

    println!("Sending request with advanced system prompt and cache control...");
    
    // Send the request
    let response = client
        .create_a_message(request_body)
        .await?;

    println!("Response: {}", response.content);

    // Alternative way to create the same system prompt using content blocks directly
    let system_prompt_alt = SystemPrompt::from_content_blocks(vec![
        ContentBlock::Text(TextContentBlock::new(
            "You are an AI assistant tasked with analyzing literary works. Your goal is to provide insightful commentary on themes, characters, and writing style.\n"
        )),
        ContentBlock::Text(TextContentBlock::new_with_cache_control(
            "<the entire contents of Pride and Prejudice>",
            CacheControl::default(),
        )),
    ]);

    let request_body_alt = MessagesRequestBody {
        model,
        messages: vec![Message::user("Analyze the major themes in Pride and Prejudice.")],
        max_tokens,
        system: Some(system_prompt_alt),
        ..Default::default()
    };

    println!("\nSending request with alternative system prompt construction...");
    
    let response_alt = client
        .create_a_message(request_body_alt)
        .await?;

    println!("Alternative Response: {}", response_alt.content);

    Ok(())
}