# clust

A Rust client for the [Anthropic Claude API](https://docs.anthropic.com/claude/reference).

## Features

- **Full API Coverage**: Complete implementation of the Messages API
- **Type Safety**: Strongly typed request and response structures
- **Async/Await**: Built on top of `tokio` and `reqwest`
- **Streaming Support**: Real-time streaming of responses
- **Error Handling**: Comprehensive error types and handling
- **Builder Pattern**: Fluent API for constructing requests
- **Cache Control**: Support for granular cache control with TTL options
- **1-Hour Caching**: Extended cache TTL support for longer caching periods

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clust = "0.9"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use clust::Client;
use clust::messages::{MessagesRequestBody, ClaudeModel, Message, MaxTokens};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a client
    let client = Client::from_env()?;

    // Create a request
    let model = ClaudeModel::Claude3Sonnet20240229;
    let max_tokens = MaxTokens::new(1024, model)?;
    let request_body = MessagesRequestBody {
        model,
        max_tokens,
        messages: vec![Message::user("Hello, Claude!")],
        ..Default::default()
    };

    // Send the request
    let response = client.create_a_message(request_body).await?;
    println!("Response: {}", response.content[0].text());

    Ok(())
}
```

## Advanced System Prompts with Cache Control

You can create advanced system prompts with granular cache control for individual content blocks. This allows you to cache specific parts of your system prompt while keeping others dynamic.

```rust
use clust::messages::{MessagesRequestBody, ClaudeModel, Message, MaxTokens, SystemPrompt, CacheControl};

// Create an advanced system prompt with cache control
let system_prompt = SystemPrompt::from_text_blocks_with_cache_control(vec![
    ("You are a helpful assistant.", None), // No cache control for the instruction
    (
        "You have access to the following information that should be cached: The weather in New York is currently sunny with a temperature of 72°F.",
        Some(CacheControl::default()), // Cache this content with ephemeral cache control
    ),
]);

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("What's the weather like?")],
    system: Some(system_prompt),
    ..Default::default()
};
```

You can also create content blocks directly with cache control:

```rust
use clust::messages::{ContentBlock, TextContentBlock, CacheControl, SystemPrompt};

let content_block = ContentBlock::Text(
    TextContentBlock::new_with_cache_control(
        "This content will be cached",
        CacheControl::default(),
    ),
);

let system_prompt = SystemPrompt::from_content_blocks(vec![content_block]);
```

This approach allows for fine-grained control over what gets cached, improving performance and reducing costs for repeated requests with the same content.

## 1-Hour Caching Support

The library supports extended cache TTL with 1-hour caching periods. When you specify a 1-hour TTL, the client automatically adds the required beta header.

### Basic 1-Hour Cache Usage

```rust
use clust::messages::{
    CacheControl, CacheTtl, ClaudeModel, ContentBlock, MaxTokens, Message,
    MessagesRequestBody, Role, SystemPrompt, TextContentBlock,
};

// Create a system prompt with 1-hour cache control
let system_prompt = SystemPrompt::from_text_blocks_with_cache_control(vec![
    ("You are a helpful assistant.", None), // No cache control for basic instruction
    (
        "You have access to the following information that should be cached for 1 hour: The weather in New York is currently sunny with a temperature of 72°F.",
        Some(CacheControl {
            _type: clust::messages::CacheControlType::Ephemeral,
            ttl: Some(CacheTtl::OneHour),
        }),
    ),
]);

// Create a message with 1-hour cache control
let message = Message {
    role: Role::User,
    content: vec![ContentBlock::Text(
        TextContentBlock::new_with_cache_control(
            "What's the weather like in New York?",
            CacheControl {
                _type: clust::messages::CacheControlType::Ephemeral,
                ttl: Some(CacheTtl::OneHour),
            },
        ),
    )],
};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![message],
    system: Some(system_prompt),
    ..Default::default()
};

let response = client.create_a_message(request_body).await?;

// The response will include cache creation information
if let Some(cache_creation) = response.usage.cache_creation {
    println!("5m cache input tokens: {}", cache_creation.ephemeral_5m_input_tokens);
    println!("1h cache input tokens: {}", cache_creation.ephemeral_1h_input_tokens);
}
```

### Cache Control Options

The `CacheControl` struct supports the following TTL options:

- **Default (5 minutes)**: `CacheControl::default()` or `ttl: None`
- **5 minutes**: `ttl: Some(CacheTtl::FiveMinutes)`
- **1 hour**: `ttl: Some(CacheTtl::OneHour)`

When using 1-hour TTL, the client automatically adds the `extended-cache-ttl-2025-04-11` beta header to the request.

### Cache Control in Content Blocks

You can apply cache control to individual content blocks:

```rust
use clust::messages::{ContentBlock, TextContentBlock, CacheControl, CacheTtl};

// Content block with 1-hour cache
let cached_block = ContentBlock::Text(
    TextContentBlock::new_with_cache_control(
        "This content will be cached for 1 hour",
        CacheControl {
            _type: clust::messages::CacheControlType::Ephemeral,
            ttl: Some(CacheTtl::OneHour),
        },
    ),
);

// Content block with 5-minute cache (default)
let short_cached_block = ContentBlock::Text(
    TextContentBlock::new_with_cache_control(
        "This content will be cached for 5 minutes",
        CacheControl {
            _type: clust::messages::CacheControlType::Ephemeral,
            ttl: Some(CacheTtl::FiveMinutes),
        },
    ),
);

// Content block with no cache control
let uncached_block = ContentBlock::Text(TextContentBlock::new("This content won't be cached"));
```

## Streaming

For real-time streaming of responses:

```rust
use clust::messages::{MessagesRequestBody, ClaudeModel, Message, MaxTokens, StreamOption};
use tokio_stream::StreamExt;

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Tell me a story")],
    stream: Some(StreamOption::ReturnStream),
    ..Default::default()
};

let mut stream = client.create_a_message_stream(request_body).await?;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(chunk) => {
            // Process the chunk
            println!("Chunk: {:?}", chunk);
        }
        Err(error) => {
            // Handle the error
            eprintln!("Error: {:?}", error);
        }
    }
}
```

## Error Handling

The library provides comprehensive error handling:

```rust
use clust::messages::MessagesError;

match client.create_a_message(request_body).await {
    Ok(response) => {
        println!("Success: {}", response.content[0].text());
    }
    Err(MessagesError::ApiError(api_error)) => {
        eprintln!("API Error: {}", api_error);
    }
    Err(MessagesError::ClientError(client_error)) => {
        eprintln!("Client Error: {}", client_error);
    }
    Err(MessagesError::StreamOptionMismatch) => {
        eprintln!("Stream option mismatch");
    }
}
```

## Builder Pattern

You can use the builder pattern for constructing requests:

```rust
use clust::messages::{MessagesRequestBuilder, ClaudeModel, Message};

let request_body = MessagesRequestBuilder::new(ClaudeModel::Claude3Sonnet20240229)
    .messages(vec![Message::user("Hello, Claude!")])
    .max_tokens(MaxTokens::new(1024, ClaudeModel::Claude3Sonnet20240229)?)
    .temperature(Temperature::new(0.7)?)
    .build();
```

## Examples

See the [examples](examples/) directory for more detailed usage examples:

- [Basic message creation](examples/create_a_message.rs)
- [Streaming messages](examples/streaming_messages.rs)
- [Advanced system prompts with cache control](examples/advanced_system_prompt.rs)
- [1-hour caching](examples/one_hour_cache.rs)
- [Tool use](examples/tool_use.rs)
- [Conversation management](examples/conversation.rs)

## API Reference

The request body is defined by `clust::messages::MessagesRequestBody`.

See also `MessagesRequestBody` for other options.

### Basic Usage

```rust
use clust::messages::MessagesRequestBody;

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    ..Default::default()
};
```

### With System Prompt

```rust
use clust::messages::{MessagesRequestBody, SystemPrompt};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    system: Some(SystemPrompt::new("You are a helpful assistant.")),
    ..Default::default()
};
```

### With Metadata

```rust
use clust::messages::{MessagesRequestBody, Metadata, UserId};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    metadata: Some(Metadata::new(UserId::new("user-123"))),
    ..Default::default()
};
```

### With Stop Sequences

```rust
use clust::messages::{MessagesRequestBody, StopSequence};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    stop_sequences: Some(vec![StopSequence::new("END")]),
    ..Default::default()
};
```

### With Temperature

```rust
use clust::messages::{MessagesRequestBody, Temperature};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    temperature: Some(Temperature::new(0.7)?),
    ..Default::default()
};
```

### With Tools

```rust
use clust::messages::{MessagesRequestBody, ToolDefinition};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    tools: Some(vec![ToolDefinition::new("get_weather", "Get the weather")]),
    ..Default::default()
};
```

### With Top P

```rust
use clust::messages::{MessagesRequestBody, TopP};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    top_p: Some(TopP::new(0.9)?),
    ..Default::default()
};
```

### With Top K

```rust
use clust::messages::{MessagesRequestBody, TopK};

let request_body = MessagesRequestBody {
    model: ClaudeModel::Claude3Sonnet20240229,
    max_tokens: MaxTokens::new(1024, model)?,
    messages: vec![Message::user("Hello, Claude!")],
    top_k: Some(TopK::new(40)?),
    ..Default::default()
};
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
