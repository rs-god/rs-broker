use async_trait::async_trait;
use std::future::Future;

#[async_trait]
pub trait Broker: Send + Sync + 'static {
    // publish message to topic
    async fn publish(&self, topic: &str, msg: &[u8]) -> Result<(), String>;

    // subscribe message from topic consumer group by async handler fn
    async fn subscribe<F, Fut>(&self, topic: &str, group: &str, handler: F) -> Result<(), String>
    where
        F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), String>> + Send + 'static;

    async fn shutdown(&self) -> Result<(), String>;
}
