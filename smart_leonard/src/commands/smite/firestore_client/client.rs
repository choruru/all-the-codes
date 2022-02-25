use anyhow::Result;
use reqwest;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use std::env;
use titlecase::titlecase;

use crate::commands::smite::api_client::model::god::Ability;
use crate::commands::smite::api_client::model::god::God;

const BASE_URL: &'static str = "https://firestore.googleapis.com/v1/projects/demandy-dog-bot";

async fn get_databases() -> Result<String> {
    let auth_token = env::var("GCP_AUTH_TOKEN").expect("GCP_AUTH_TOKEN not found.");
    let url = format!("{}/databases", BASE_URL);
    println!("{}", url);
    let res = reqwest::Client::new()
        .get(url)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;
    Ok(res.text().await?)
}

async fn create_document<T: Serialize>(
    collection: &str,
    document_id: &str,
    document: &T,
) -> Result<String> {
    // let auth_token = env::var("GCP_AUTH_TOKEN").expect("GCP_AUTH_TOKEN not found.");
    let auth_token = "ya29.A0ARrdaM-KM-ey1ykmo1TTTs7DuItQDuLpcR4daKpQP8vbA1W0Vt46u5R5DMNG_DbLpoM96Z8FXcFwmIptInSpnZ4MURMa3mN_aSpU1dforB1qYoqp52ag5OnB0tUXpKOynwnFQ9ZrMBRlvm6O59s6BmsYFnSnGMWsgVUFF6E";
    let url = format!(
        "{}/databases/(default)/documents/{}/{}",
        BASE_URL, collection, document_id
    );
    let res = reqwest::Client::new()
        .patch(url)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", auth_token))
        .json(document)
        .send()
        .await?;
    Ok(res.text().await?)
}

pub async fn create_god(god: &God) -> Result<String> {
    fn serialize_ability(ability: &Ability) -> Value {
        json!({"mapValue": {
                "fields": {
                    "cooldown": {
                        "stringValue": ability.item_description.cooldown
                    },
                    "cost": {
                        "stringValue": ability.item_description.cost
                    },
                    "cooldown": {
                        "stringValue": ability.item_description.description
                    },
                    "menuitems": {
                        "arrayValue": {
                            "values": ability.item_description.menuitems.iter().map(|item|
                                json!({
                                    "mapValue": {
                                        "fields": {
                                            "description": {
                                                "stringValue": item.description
                                            },
                                            "value": {
                                                "stringValue": item.value
                                            }
                                        }
                                    }
                                })
                            ).collect::<Vec<Value>>()
                        },
                    },
                    "rankitems": {
                        "arrayValue": {
                            "values": ability.item_description.rankitems.iter().map(|item|
                                json!({
                                    "mapValue": {
                                        "fields": {
                                            "description": {
                                                "stringValue": item.description
                                            },
                                            "value": {
                                                "stringValue": item.value
                                            }
                                        }
                                    }
                                })
                            ).collect::<Vec<Value>>()
                        },
                    },
                },
            },
        })
    }

    let doc = json!({
        "fields": {
            "id": {
                "integerValue": god.id
            },
            "name": {
                "stringValue": god.name
            },
            "type_": {
                "stringValue": god.type_
            },
            "role": {
                "stringValue": god.role
            },
            "passive_name": {
                "stringValue": god.passive_name
            },
            "passive": serialize_ability(&god.passive),
            "ability1_name": {
                "stringValue": god.ability1_name
            },
            "ability1": serialize_ability(&god.ability1),
            "ability2_name": {
                "stringValue": god.ability2_name
            },
            "ability2": serialize_ability(&god.ability2),
            "ability3_name": {
                "stringValue": god.ability3_name
            },
            "ability3": serialize_ability(&god.ability3),
            "ability4_name": {
                "stringValue": god.ability4_name
            },
            "ability4": serialize_ability(&god.ability4),
        },
    });
    println!("{:#?}", doc);
    create_document("god", &titlecase(&god.name), &doc).await
}

async fn get_document() -> Result<Value> {
    let auth_token = env::var("GCP_AUTH_TOKEN").expect("GCP_AUTH_TOKEN not found.");
    let url = "https://firestore.googleapis.com/v1/projects/demandy-dog-bot/databases/(default)/documents/test/john";
    println!("{}", url);
    let res = reqwest::Client::new()
        .get(url)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;
    Ok(res.json::<Value>().await?)
}

mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::commands::smite::api_client::{self, client::get_sesssion_id, client::getgods};

    #[tokio::test]
    async fn it_works() {
        let res = super::get_databases().await.unwrap();
        println!("{}", res)
    }

    #[tokio::test]
    async fn create_document_works() {
        let doc = json!({
            "fields": {
                "age": {
                    "integerValue": 42,
                }
            },
        });
        let res = super::create_document("test", "john", &doc).await.unwrap();
        println!("{}", res)
    }

    #[tokio::test]
    async fn create_god_works() {
        let sess_id = get_sesssion_id().await.unwrap();
        let gods = getgods(&sess_id).await.unwrap();
        let res = super::create_god(&gods[0]).await.unwrap();
        println!("{}", res)
    }

    #[tokio::test]
    async fn get_document_works() {
        let res = super::get_document().await.unwrap();
        println!("{}", res)
    }
}
