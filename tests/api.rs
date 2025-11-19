use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

fn get_test_base_url() -> String {
    std::env::var("TEST_BASE_URL").unwrap_or_else(|_| "http://localhost:8082".to_string())
}

async fn wait_for_service() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut retries = 30;
    let base_url = get_test_base_url();
    
    while retries > 0 {
        match client.get(&format!("{}/health", base_url)).send().await {
            Ok(response) if response.status().is_success() => {
                return Ok(());
            }
            _ => {
                retries -= 1;
                if retries > 0 {
                    println!("Service not ready, retrying in 1 second... ({} retries left)", retries);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
    
    Err("Service did not become ready in time".into())
}

struct TestContext {
    client: Client,
    base_url: String,
}

impl TestContext {
    async fn new() -> Self {
        wait_for_service().await.unwrap();

        let base_url = get_test_base_url();

        Self {
            client: Client::new(),
            base_url,
        }
    }
}

mod schema_tests {
    use super::*;

    fn valid_schema_payload(name: &str) -> serde_json::Value {
        json!({
            "name": name,
            "version": "1.0.0",
            "schema_definition": {
                "type": "object",
                "properties": {
                    "message": { "type": "string" }
                },
                "required": [ "message" ]
            }
        })
    }

    mod create_schema {
        use reqwest::StatusCode;
        use log_server::{ErrorResponse, Schema};
        use uuid::Uuid;

        use super::*;

        const TEST_SCHEMA_NAME: &str = "test-schema";
        const TEST_SCHEMA_VERSION: &str = "1.0.0";

        #[tokio::test]
        async fn crates_schema_with_valid_data() {
            let ctx = TestContext::new().await;

            let response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload(TEST_SCHEMA_NAME))
                .send()
                .await
                .expect("Failed to send request");

            assert_eq!(response.status(), StatusCode::CREATED);

            let schema: Schema = response.json().await.unwrap();
            assert_eq!(schema.name, TEST_SCHEMA_NAME);
            assert_eq!(schema.version, TEST_SCHEMA_VERSION);
            let uuid_str = schema.id.to_string();
            assert!(Uuid::parse_str(&uuid_str).is_ok(), "Schema ID should be a valid UUID");
            assert!(schema.created_at.timestamp() > 0);
        }

        #[tokio::test]
        async fn returns_201_with_location_header() {
            let ctx = TestContext::new().await;

            let response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("location-test"))
                .send()
                .await
                .unwrap();
            
            assert_eq!(response.status(), StatusCode::CREATED);

            let location = response.headers()
                .get("Location")
                .expect("Location header should be present");

            assert!(location.to_str().unwrap().contains("/schemas/"));
        }

        #[tokio::test]
        async fn rejects_duplicate_schema_name() {
            let ctx = TestContext::new().await;

            ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("duplicate"))
                .send()
                .await
                .unwrap();

            let response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("duplicate"))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::CONFLICT);

            let error: ErrorResponse = response.json().await.unwrap();
            assert!(error.message.contains("already exists"));
        }

        #[tokio::test]
        async fn rejects_missing_required_fields() {
            let ctx = TestContext::new().await;

            let invalid_payload = json!({
                "version": "1.0.0",
            });

            let response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&invalid_payload)
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
            
            let error_text = response.text().await.unwrap();
            assert!(error_text.contains("missing field") || error_text.contains("name"));
        }

        #[tokio::test]
        async fn handles_special_characters_in_name() {
            let ctx = TestContext::new().await;

            let response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("test-schema_123.v2"))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::CREATED);
        }

        #[tokio::test]
        async fn rejects_name_exceeding_max_length() {
            let ctx = TestContext::new().await;
            let long_name = "a".repeat(256);

            let response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload(&long_name))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
    }

    mod get_schema {
        use log_server::Schema;
        use reqwest::StatusCode;

        use super::*;

        #[tokio::test]
        async fn retrieves_existing_schema() {
            let ctx = TestContext::new().await;
            let schema_response = ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("get-test"))
                .send()
                .await
                .unwrap();

            let schema: Schema = schema_response
                .json()
                .await
                .unwrap();

            let response = ctx.client
                .get(&format!("{}/schemas/{}", ctx.base_url, schema.id))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let retrieved: Schema = response.json().await.unwrap();
            assert_eq!(retrieved.id, schema.id);
            assert_eq!(retrieved.name, "get-test");
        }

        #[tokio::test]
        async fn returns_404_for_nonexistent_schema() {
            let ctx = TestContext::new().await;

            let response = ctx.client
                .get(&format!("{}/schemas/{}", ctx.base_url, "7182c4cb-24dc-4142-890c-3c7755ba673e"))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[tokio::test]
        async fn rejects_invalid_uuid_format() {
            let ctx = TestContext::new().await;

            let response = ctx.client
                .get(&format!("{}/schemas/{}", ctx.base_url, "not-a-uuid"))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
    }

    mod list_schemas {
        use reqwest::StatusCode;

        use super::*;

        #[tokio::test]
        async fn lists_all_schemas() {
            let ctx = TestContext::new().await;

            ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("list-1"))
                .send()
                .await
                .unwrap();

            ctx.client
                .post(&format!("{}/schemas", ctx.base_url))
                .json(&valid_schema_payload("list-2"))
                .send()
                .await
                .unwrap();
            
            let response = ctx.client
                .get(&format!("{}/schemas", ctx.base_url))
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);

            let list: SchemaList = response.json().await.unwrap();
            assert!(list.schemas.len() >= 2);
        }
    }
}
