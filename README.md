[![CircleCI](https://circleci.com/gh/mmmpa/poor_persistent_data_store.svg?style=shield)](https://circleci.com/gh/mmmpa/poor_persistent_data_store)


# poor_persistent_data_store

This provide persistency for serverless functions like AWS Lambda.

# DynamoSimpleHashMap

We can use `DynamoSimpleHashMap` like HashMaps that have persistency.

```rust
pub struct SimpleClientA {
    pub cli: DynamoDbClient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Content {
    pub content: usize,
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
```

## `type Key`

We can use followings

- `A`
- `(A, B)`

A and B are either `String`, `f64` or `Bytes`. They must be keys for DynamoDB.

### To use other types

We wrap it and impliment `Into<key>` for it.

`Key`'s definition is below

```rust
pub struct Key(AttributeValue)
```

so we implement like below if we have `Foo`.

```rust
struct NewType(Foo)

impl Into<Key> for NewType {
  fn into(self) -> Key {
    Key(AttributeValue {
        s: Some(self.to_string()),
        ..Default::default()
    })
  }
}
```