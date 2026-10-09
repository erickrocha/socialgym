use crate::domain::business_error::BusinessError;
use aws_sdk_sqs::Client;

pub struct SqsGateway {}

impl SqsGateway {
    pub async fn send_fifo_message(
        client: &Client,
        queue_url: &str,
        body: &str,
        group_id: &str,
        deduplication_id: &str,
    ) -> Result<(), BusinessError> {
        client
            .send_message()
            .queue_url(queue_url)
            .message_body(body)
            .message_group_id(group_id)
            .message_deduplication_id(deduplication_id)
            .send()
            .await
            .map_err(|error| {
                BusinessError::new(format!("Failed to publish SQS message: {error}"))
            })?;
        Ok(())
    }

    /// Long-polls the queue and returns up to `max_messages` (≤ 10) messages.
    /// `wait_time_seconds` controls the long-poll duration (1–20 s).
    pub async fn receive_messages(
        client: &Client,
        queue_url: &str,
        max_messages: i32,
        wait_time_seconds: i32,
    ) -> Result<Vec<aws_sdk_sqs::types::Message>, BusinessError> {
        let output = client
            .receive_message()
            .queue_url(queue_url)
            .max_number_of_messages(max_messages)
            .wait_time_seconds(wait_time_seconds)
            .send()
            .await
            .map_err(|e| BusinessError::new(format!("Failed to receive SQS messages: {e:?}")))?;

        Ok(output.messages.unwrap_or_default())
    }

    /// Acknowledges (deletes) a successfully processed message from the queue.
    pub async fn delete_message(
        client: &Client,
        queue_url: &str,
        receipt_handle: &str,
    ) -> Result<(), BusinessError> {
        client
            .delete_message()
            .queue_url(queue_url)
            .receipt_handle(receipt_handle)
            .send()
            .await
            .map_err(|e| BusinessError::new(format!("Failed to delete SQS message: {e:?}")))?;
        Ok(())
    }
}
