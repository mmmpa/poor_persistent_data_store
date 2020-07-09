use bytes::Bytes;
use rusoto_dynamodb::AttributeValue;

pub trait CanQueryKey {}

impl<A: Into<Key>, B: Into<Key>> CanQueryKey for (A, B) {}

#[derive(Debug, Clone)]
pub struct Keys(pub Key, pub Option<Key>);

#[derive(Debug, Clone)]
pub struct S(AttributeValue);

#[derive(Debug, Clone)]
pub struct N(AttributeValue);

#[derive(Debug, Clone)]
pub struct B(AttributeValue);

#[derive(Debug, Clone)]
pub struct Key(AttributeValue);

impl<T: Into<String>> From<T> for S {
    fn from(v: T) -> Self {
        Self(AttributeValue {
            s: Some(v.into()),
            ..Default::default()
        })
    }
}

impl<T: Into<f64>> From<T> for N {
    fn from(v: T) -> Self {
        Self(AttributeValue {
            n: Some(v.into().to_string()),
            ..Default::default()
        })
    }
}

impl<T: Into<Bytes>> From<T> for B {
    fn from(v: T) -> Self {
        Self(AttributeValue {
            b: Some(v.into()),
            ..Default::default()
        })
    }
}

impl Into<AttributeValue> for S {
    fn into(self) -> AttributeValue {
        self.0
    }
}

impl Into<AttributeValue> for N {
    fn into(self) -> AttributeValue {
        self.0
    }
}

impl Into<AttributeValue> for B {
    fn into(self) -> AttributeValue {
        self.0
    }
}

impl From<String> for Key {
    fn from(v: String) -> Self {
        Self(S::from(v).0)
    }
}

impl From<&str> for Key {
    fn from(v: &str) -> Self {
        Self(S::from(v).0)
    }
}

impl From<f64> for Key {
    fn from(v: f64) -> Self {
        Self(N::from(v).0)
    }
}

impl From<Bytes> for Key {
    fn from(v: Bytes) -> Self {
        Self(B::from(v).0)
    }
}

impl Into<AttributeValue> for Key {
    fn into(self) -> AttributeValue {
        self.0
    }
}

impl<A: Into<Key>> From<A> for Keys {
    fn from(a: A) -> Self {
        Self(a.into(), None)
    }
}

impl<A: Into<Key>, B: Into<Key>> From<(A, B)> for Keys {
    fn from((a, b): (A, B)) -> Self {
        Self(a.into(), Some(b.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Keys;

    fn gen(keys: impl Into<Keys>) -> Keys {
        keys.into()
    }

    #[test]
    fn test() {
        gen("a");
        gen(1.0);
        gen(("a", 1.0));
    }
}
