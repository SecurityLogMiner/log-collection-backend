use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::primitives::Blob;
use aws_sdk_dynamodb::{config::Region, meta::PKG_VERSION, Error};
use aws_sdk_dynamodb::operation::create_table::{CreateTableOutput,CreateTableError};
use aws_sdk_dynamodb::operation::put_item::{PutItemOutput, PutItemError};
use aws_sdk_dynamodb::error::BuildError;

use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use async_trait::async_trait;
use std::sync::mpsc::Receiver;
use crate::dynamosdk;
use crate::traits::DataHandler;
use crate::config::{Config, Package};
use crate::producer::start_log_stream;
use aws_sdk_dynamodb::Client as DynamodbClient;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Debug,Clone)]
pub struct DynamodbClientWrapper {
    pub client: DynamodbClient,
    pub table: String,
}

#[async_trait]
impl DataHandler for DynamodbClientWrapper {
    async fn handle_log_data(&self, log_channel: Receiver<(String,String)>) {
        match self.check_table().await {
            true => {
                for log_tuple in log_channel {
                    let (time, data) = log_tuple;
                    println!("{time:?},{data:?},{:?}", self.table);
                    let res = self.client.put_item()
                        .table_name(&self.table)
                        .item("eptestkey", AttributeValue::S(data))
                        .send().await;
                    // dumb error checking for now. eventually, this will need to be 
                    // sent to the status api for the user.
                    println!("{res:?}");
                }
            },
            false => println!("oh no"),
        }
    }

    async fn check_table(&self) -> bool {
        // check if the table listed in the configuration file
        // exists. If it does not, create it. 
        let tables = self.client.list_tables()
                            .into_paginator()
                            .items()
                            .send(); 
        let table_names = tables.collect::<Result<Vec<_>,_>>().await.unwrap();
        for tbl in table_names {
            println!("checking for {:?}", self.table);
            if tbl == self.table {
                println!("found {tbl:?}");
                return true
            }
        } 
        if let Ok(table) = self.create_table().await {
            println!("{table:?}");
            return true
        }
        false
    }

    async fn create_table(&self) -> Result<CreateTableOutput, String> {
        let a_name: String = "eptestkey".into();//String = key.into();
        let table_name = &self.table;

        let ad = AttributeDefinition::builder()
            .attribute_name(&a_name)
            .attribute_type(ScalarAttributeType::S)
            .build().unwrap();

        let ks = KeySchemaElement::builder()
            .attribute_name(&a_name)
            .key_type(KeyType::Hash)
            .build().unwrap();

        let pt = ProvisionedThroughput::builder()
            .read_capacity_units(10)
            .write_capacity_units(5)
            .build().unwrap();

        let create_table_response = 
            self.client.create_table()
            .table_name(table_name)
            .key_schema(ks)
            .attribute_definitions(ad)
            .provisioned_throughput(pt)
            .send()
            .await;

        match create_table_response {
            Ok(out) => {
                println!("Added table {} with key {}", self.table, a_name);
                Ok(out)
            }
            Err(e) => {
                eprintln!("Got an error creating table:{e:?}");
                Err("error".to_string())
            }
        }
    }
}

pub fn show_dynamodb_tables() -> Result<(), Error> {
    todo!();
    // Ok(())
}

pub async fn
create_client(table: String) -> Result<DynamodbClientWrapper, Error> {
    println!("add this table to client struct: {table}");
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    let wrapper = DynamodbClientWrapper {
        client: client,
        table: table,
    };
    Ok(wrapper)
}
