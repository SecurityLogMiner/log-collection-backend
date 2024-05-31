use aws_sdk_dynamodb::{Client as DynamodbClient, Error};
use aws_sdk_dynamodb::operation::create_table::CreateTableOutput;
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType, AttributeValue};
use async_trait::async_trait;
use std::sync::mpsc::Receiver;
use crate::traits::DataHandler;

#[derive(Debug, Clone)]
pub struct DynamodbClientWrapper {
    pub client: DynamodbClient,
    pub table: String,
}

#[async_trait]
impl DataHandler for DynamodbClientWrapper {
    async fn handle_log_data(&self, log_channel: Receiver<(String, String)>) {
        let table_name = self.table.clone();
        let client = self.client.clone();
        
        if self.check_table().await {
            tokio::spawn(async move {
                for log_tuple in log_channel {
                    let (_time, data) = log_tuple;
                    // println!("{:?}, {:?}, {:?}", time, data, table_name);
                    let res = client.put_item()
                        .table_name(&table_name)
                        .item("eptestkey", AttributeValue::S(data))
                        .send().await;
                    println!("{:?}", res);
                }
            });
        } else {
            println!("Table check failed.");
        }
    }

    async fn check_table(&self) -> bool {
        let tables = self.client.list_tables()
        .into_paginator()
        .items()
        .send(); 
        let table_names = tables.collect::<Result<Vec<_>,_>>().await.unwrap();
        for tbl in table_names {
            // println!("checking for {:?}", self.table);
            if tbl == self.table {
                // println!("found {tbl:?}");
                return true
            }
        } 
        if let Ok(_table) = self.create_table().await {
            // println!("{table:?}");
            return true
        }
        false
    }
    async fn create_table(&self) -> Result<CreateTableOutput, String> {
        let table_name = &self.table;
        let a_name: String = "eptestkey".into();

        let ad = AttributeDefinition::builder()
            .attribute_name(&a_name)
            .attribute_type(ScalarAttributeType::S)
            .build()
            .expect("Failed to build AttributeDefinition");

        let ks = KeySchemaElement::builder()
            .attribute_name(&a_name)
            .key_type(KeyType::Hash)
            .build()
            .expect("Failed to build KeySchemaElement");

        let pt = ProvisionedThroughput::builder()
            .read_capacity_units(10)
            .write_capacity_units(5)
            .build()
            .expect("Failed to build ProvisionedThroughput");

        let create_table_response = self.client.create_table()
            .table_name(table_name)
            .key_schema(ks)
            .attribute_definitions(ad)
            .provisioned_throughput(pt)
            .send()
            .await;

        match create_table_response {
            Ok(out) => Ok(out),
            Err(e) => Err(format!("Got an error creating table: {:?}", e)),
        }
    }
}

pub async fn create_client(table: String) -> Result<DynamodbClientWrapper, Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    Ok(DynamodbClientWrapper { client, table })
}
