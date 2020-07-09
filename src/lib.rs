#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod dynamo_simple_hash_map;
mod error;
mod keys;
mod test_helpers;

pub use self::dynamo_simple_hash_map::*;
pub use error::*;
pub use keys::*;
pub use test_helpers::*;

pub type DshmResult<T> = Result<T, DshmError>;

#[cfg(test)]
mod tests {
    use crate::test_helpers::test_helpers::{
        gen_dynamo_client, Content, SimpleClientA, SimpleClientB,
    };
    use crate::DynamoSimpleHashMap;

    #[tokio::test]
    async fn test() {
        pretty_env_logger::init();
        let cli = SimpleClientA {
            cli: gen_dynamo_client(),
        };

        let inserted = cli.insert(("a", "b"), Content { content: 123 }).await;
        assert!(inserted.is_ok(), "can insert data {:?}", inserted);

        let got = cli.get(("a", "b")).await;
        assert!(got.is_ok(), "can get data {:?}", got);
        assert_eq!(got.unwrap().content, 123);

        let got = cli.get_strongly(("a", "b")).await;
        assert!(got.is_ok(), "can get data {:?}", got);
        assert_eq!(got.unwrap().content, 123);

        let updated = cli.insert(("a", "b"), Content { content: 456 }).await;
        assert!(updated.is_ok(), "can update same key data {:?}", updated);

        let got = cli.get(("a", "b")).await;
        assert!(got.is_ok(), "can get updated data {:?}", got);
        assert_eq!(got.unwrap().content, 456);

        let got = cli.get(("a", "c")).await;
        assert!(got.is_err(), "cannot get data not exist {:?}", got);

        let removed = cli.remove(("a", "b")).await;
        assert!(removed.is_ok(), "can remove data {:?}", removed);
        assert_eq!(removed.unwrap().content, 456);

        let removed = cli.remove(("a", "c")).await;
        assert!(
            removed.is_err(),
            "cannot remove data not exist {:?}",
            removed
        );

        let removed = cli.remove(("a", "b")).await;
        assert!(removed.is_err(), "cannot remove twice {:?}", removed);
    }

    #[tokio::test]
    async fn test_query() {
        let cli = SimpleClientA {
            cli: gen_dynamo_client(),
        };

        cli.insert(("q", "a"), Content { content: 1 })
            .await
            .unwrap();
        cli.insert(("q", "b"), Content { content: 2 })
            .await
            .unwrap();
        cli.insert(("q", "c"), Content { content: 3 })
            .await
            .unwrap();
        cli.insert(("q", "d"), Content { content: 4 })
            .await
            .unwrap();

        cli.insert(("r", "a"), Content { content: 24 })
            .await
            .unwrap();

        let queried = cli.query("q".into(), 10).await;
        assert!(queried.is_ok(), "can query {:?}", queried);

        let queried = queried.unwrap();
        assert_eq!(queried.len(), 4, "can query all inserted {:?}", queried)
    }

    #[tokio::test]
    async fn test_diff() {
        let cli = SimpleClientB {
            cli: gen_dynamo_client(),
        };

        let inserted = cli.insert("a", Content { content: 123 }).await;
        assert!(inserted.is_ok(), "can insert data {:?}", inserted);

        let got = cli.get("a").await;
        assert!(got.is_ok(), "can get data {:?}", got);
        assert_eq!(got.unwrap().content, 123);
    }
}
