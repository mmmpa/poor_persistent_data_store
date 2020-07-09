#[cfg(test)]
pub mod test_helpers {
    use crate::DynamoSimpleHashMap;
    use rusoto_core::{HttpClient, Region};
    use rusoto_credential::StaticProvider;
    use rusoto_dynamodb::DynamoDbClient;
    use serde::{Deserialize, Serialize};

    pub struct SimpleClientA {
        pub cli: DynamoDbClient,
    }

    impl DynamoSimpleHashMap for SimpleClientA {
        type DynamoDb = DynamoDbClient;
        type Key = (&'static str, &'static str);
        type Data = Content;

        fn dynamo_db(&self) -> &Self::DynamoDb {
            &self.cli
        }

        fn table_name(&self) -> String {
            "test-a".to_string()
        }
    }

    pub struct SimpleClientB {
        pub cli: DynamoDbClient,
    }

    impl DynamoSimpleHashMap for SimpleClientB {
        type DynamoDb = DynamoDbClient;
        type Key = &'static str;
        type Data = Content;

        fn dynamo_db(&self) -> &Self::DynamoDb {
            &self.cli
        }

        fn table_name(&self) -> String {
            "test-b".to_string()
        }

        fn hash_key(&self) -> String {
            "key".to_string()
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Content {
        pub content: usize,
    }

    pub fn gen_dynamo_client() -> DynamoDbClient {
        let region = Region::Custom {
            name: "us-west-2".to_owned(),
            endpoint: "http://localhost:8000".to_owned(),
        };
        let credential = StaticProvider::new(
            "fakeMyKeyId".to_string(),
            "fakeSecretAccessKey".to_string(),
            None,
            None,
        );
        DynamoDbClient::new_with(HttpClient::new().unwrap(), credential, region)
    }

    #[tokio::test]
    async fn test() {
        let cli = SimpleClientA {
            cli: DynamoDbClient::new(Region::UsEast1),
        };

        let inserted = cli.insert(("a", "b"), Content { content: 123 }).await;
        assert!(inserted.is_ok());

        let got = cli.get(("a", "b")).await;
        assert!(got.is_ok());
        assert_eq!(got.unwrap().content, 123);
    }
}
