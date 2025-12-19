use crate::ApiError;
use crate::Client;
use crate::ClientError;
use crate::Beta;
use crate::messages::chunk_stream::ChunkStream;
use crate::messages::{
    MessageChunk, MessagesError, MessagesRequestBody, MessagesResponseBody,
    StreamError, StreamOption, CacheTtl,
};

use futures_core::Stream;

/// Check if any content block in the request body uses 1-hour TTL
fn has_one_hour_ttl(request_body: &MessagesRequestBody) -> bool {
    // Check messages for content blocks with 1-hour TTL
    for message in &request_body.messages {
        match &message.content {
            crate::messages::Content::SingleText(_) => {
                // Single text content doesn't have cache control
            },
            crate::messages::Content::MultipleBlocks(blocks) => {
                for content_block in blocks {
                    if let Some(cache_control) = &content_block.cache_control() {
                        if let Some(ttl) = &cache_control.ttl {
                            if *ttl == CacheTtl::OneHour {
                                return true;
                            }
                        }
                    }
                }
            },
        }
    }
    
    // Check system prompt for content blocks with 1-hour TTL
    if let Some(system_prompt) = &request_body.system {
        match system_prompt {
            crate::messages::SystemPrompt::Simple(_) => {
                // Simple system prompt doesn't have cache control
            },
            crate::messages::SystemPrompt::Advanced(blocks) => {
                for content_block in blocks {
                    if let Some(cache_control) = &content_block.cache_control() {
                        if let Some(ttl) = &cache_control.ttl {
                            if *ttl == CacheTtl::OneHour {
                                return true;
                            }
                        }
                    }
                }
            },
        }
    }
    
    false
}

pub(crate) async fn create_a_message(
    client: &Client,
    request_body: MessagesRequestBody,
    endpoint: &str,
) -> Result<MessagesResponseBody, MessagesError> {
    // Validate stream option.
    if let Some(stream) = &request_body.stream {
        if *stream != StreamOption::ReturnOnce {
            return Err(MessagesError::StreamOptionMismatch);
        }
    }

    // Check if we need to add the extended cache beta header
    let mut request_builder = client.post(endpoint);
    
    if has_one_hour_ttl(&request_body) {
        request_builder = request_builder.header("anthropic-beta", Beta::ExtendedCacheTtl2025_04_11.to_string());
    }

    // Send the request.
    let response = request_builder
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Read the response text.
    let response_text = response
        .text()
        .await
        .map_err(ClientError::ReadResponseTextFailed)?;

    // Ok
    if status_code.is_success() {
        // Deserialize the response.
        serde_json::from_str(&response_text).map_err(|error| {
            {
                ClientError::ResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            }
            .into()
        })
    }
    // Error
    else {
        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                ClientError::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(ApiError::new(status_code, error_response).into())
    }
}

pub(crate) async fn create_a_message_stream(
    client: &Client,
    request_body: MessagesRequestBody,
    endpoint: &str,
) -> Result<impl Stream<Item = Result<MessageChunk, StreamError>>, MessagesError>
{
    // Validate stream option.
    if request_body.stream.is_none() {
        return Err(MessagesError::StreamOptionMismatch);
    }
    if let Some(stream) = &request_body.stream {
        if *stream != StreamOption::ReturnStream {
            return Err(MessagesError::StreamOptionMismatch);
        }
    }

    eprintln!("endpoint: {}", endpoint);
    // Check if we need to add the extended cache beta header
    let mut request_builder = client.post(endpoint);
    
    if has_one_hour_ttl(&request_body) {
        request_builder = request_builder.header("anthropic-beta", Beta::ExtendedCacheTtl2025_04_11.to_string());
    }

    // Send the request.
    let response = request_builder
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::HttpRequestError)?;

    // Check the response status code.
    let status_code = response.status();

    // Ok
    if status_code.is_success() {
        // Create a chunk stream from response bytes stream.
        let byte_stream = response.bytes_stream();
        let chunk_stream = ChunkStream::new(byte_stream);
        Ok(chunk_stream)
    }
    // Error
    else {
        // Read the response text.
        let response_text = response
            .text()
            .await
            .map_err(ClientError::ReadResponseTextFailed)?;

        // Deserialize the error response.
        let error_response =
            serde_json::from_str(&response_text).map_err(|error| {
                ClientError::ErrorResponseDeserializationFailed {
                    error,
                    text: response_text,
                }
            })?;

        Err(ApiError::new(status_code, error_response).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{
        CacheControl, CacheControlType, ClaudeModel, ContentBlock, MaxTokens, Message,
        MessagesRequestBody, Role, SystemPrompt, TextContentBlock,
    };

    #[test]
    fn test_has_one_hour_ttl() {
        // Test with no cache control
        let request_body = MessagesRequestBody {
            model: ClaudeModel::Claude3Sonnet20240229,
            max_tokens: MaxTokens::new(1024, ClaudeModel::Claude3Sonnet20240229).unwrap(),
            messages: vec![Message::user("Hello")],
            ..Default::default()
        };
        assert!(!has_one_hour_ttl(&request_body));

        // Test with 1-hour TTL in message content
        let message = Message {
            role: Role::User,
            content: crate::messages::Content::MultipleBlocks(vec![
                ContentBlock::Text(TextContentBlock::new_with_cache_control(
                    "Hello",
                    CacheControl {
                        _type: CacheControlType::Ephemeral,
                        ttl: Some(CacheTtl::OneHour),
                    },
                )),
            ]),
        };
        let request_body = MessagesRequestBody {
            model: ClaudeModel::Claude3Sonnet20240229,
            max_tokens: MaxTokens::new(1024, ClaudeModel::Claude3Sonnet20240229).unwrap(),
            messages: vec![message],
            ..Default::default()
        };
        assert!(has_one_hour_ttl(&request_body));

        // Test with 1-hour TTL in system prompt
        let system_prompt = SystemPrompt::from_text_blocks_with_cache_control(vec![
            ("You are a helpful assistant.", None),
            (
                "Cached information",
                Some(CacheControl {
                    _type: CacheControlType::Ephemeral,
                    ttl: Some(CacheTtl::OneHour),
                }),
            ),
        ]);
        let request_body = MessagesRequestBody {
            model: ClaudeModel::Claude3Sonnet20240229,
            max_tokens: MaxTokens::new(1024, ClaudeModel::Claude3Sonnet20240229).unwrap(),
            messages: vec![Message::user("Hello")],
            system: Some(system_prompt),
            ..Default::default()
        };
        assert!(has_one_hour_ttl(&request_body));
    }
}
