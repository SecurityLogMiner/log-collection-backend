use async_trait::async_trait;
use std::sync::mpsc::Receiver;
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_dynamodb::operation::create_table::{CreateTableOutput,CreateTableError};
use crate::config::Package;

#[async_trait]
pub trait DataHandler {
    async fn check_table(&self) -> bool;
    async fn create_table(&self) -> Result<CreateTableOutput, String>;
    async fn handle_log_data(&self,log_channel: Receiver<(String,String)>);
}

