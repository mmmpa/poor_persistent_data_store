use crate::error::DshmError;
use crate::{CanQueryKey, Key, Keys, S};

use async_trait::async_trait;
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, GetItemInput, PutItemInput, QueryInput, QueryOutput,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

#[async_trait]
pub trait DynamoSimpleHashMap {
    type DynamoDb: DynamoDb + Send + Sync;
    type Key: Into<Keys> + Send;
    type Data: Serialize + DeserializeOwned + Send;

    fn dynamo_db(&self) -> &Self::DynamoDb;
    fn table_name(&self) -> String;

    fn hash_key(&self) -> String {
        "hash_key".to_string()
    }

    fn range_key(&self) -> String {
        "range_key".to_string()
    }

    fn data_attribute(&self) -> String {
        "data".to_string()
    }

    fn build_base_item(&self, key: Self::Key) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        match key.into() {
            Keys(hash_key, None) => {
                item.insert(self.hash_key(), hash_key.into());
            }
            Keys(hash_key, Some(range_key)) => {
                item.insert(self.hash_key(), hash_key.into());
                item.insert(self.range_key(), range_key.into());
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }

        item
    }

    fn pick_data(&self, o: Option<HashMap<String, AttributeValue>>) -> Option<String> {
        o?.remove(&self.data_attribute())?.s
    }

    fn retrieve(
        &self,
        o: Option<HashMap<String, AttributeValue>>,
    ) -> Result<Self::Data, DshmError> {
        let raw = self.pick_data(o).ok_or(DshmError::NoItem)?;
        Ok(serde_json::from_str(&raw)?)
    }

    async fn insert(&self, key: Self::Key, data: Self::Data) -> Result<(), DshmError> {
        let mut item = self.build_base_item(key);

        item.insert(
            self.data_attribute(),
            S::from(serde_json::to_string(&data)?).into(),
        );

        let item = PutItemInput {
            table_name: self.table_name(),
            item,
            ..Default::default()
        };
        debug!("{:?}", item);

        self.dynamo_db().put_item(item).await?;

        Ok(())
    }

    async fn get_base(
        &self,
        key: Self::Key,
        consistent_read: Option<bool>,
    ) -> Result<Self::Data, DshmError> {
        let item = GetItemInput {
            table_name: self.table_name(),
            key: self.build_base_item(key),
            attributes_to_get: Some(vec![self.data_attribute()]),
            consistent_read,
            ..Default::default()
        };
        debug!("{:?}", item);

        let o = self.dynamo_db().get_item(item).await?;

        self.retrieve(o.item)
    }

    async fn get_strongly(&self, key: Self::Key) -> Result<Self::Data, DshmError> {
        self.get_base(key, Some(true)).await
    }

    async fn get(&self, key: Self::Key) -> Result<Self::Data, DshmError> {
        self.get_base(key, None).await
    }

    async fn query(&self, key: Key, limit: i64) -> Result<Vec<Self::Data>, DshmError>
    where
        Self::Key: CanQueryKey,
    {
        let h = self.hash_key();

        // hash = :hash
        let condition = format!("{} = :{}", h.as_str(), h.as_str());

        // {":hash":{"S":"<PASSED_KEY>"}}
        let mut values = HashMap::default();
        values.insert(format!(":{}", h.as_str()), key.into());

        let QueryOutput {
            items,
            last_evaluated_key: _,
            ..
        } = self
            .dynamo_db()
            .query(QueryInput {
                table_name: self.table_name(),
                key_condition_expression: Some(condition),
                expression_attribute_values: Some(values),
                limit: Some(limit),
                ..Default::default()
            })
            .await?;

        let data = items
            .ok_or(DshmError::NoItem)?
            .into_iter()
            .filter_map(|item| match self.retrieve(Some(item)) {
                Ok(o) => Some(o),
                Err(_) => None,
            })
            .collect();

        // TODO: limit を超えた時の処理
        //
        // last_evaluated_key

        Ok(data)
    }

    async fn remove(&self, key: Self::Key) -> Result<Self::Data, DshmError> {
        let o = self
            .dynamo_db()
            .delete_item(DeleteItemInput {
                table_name: self.table_name(),
                key: self.build_base_item(key),
                return_values: Some("ALL_OLD".to_string()),
                ..Default::default()
            })
            .await?;

        self.retrieve(o.attributes)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::test_helpers::{gen_dynamo_client, SimpleClientB};

    #[test]
    fn test() {
        let _cli = SimpleClientB {
            cli: gen_dynamo_client(),
        };

        // not implemented CanQueryKey for Key of scalar.
        // cli.query("q".into(), 10).await;
    }
}
