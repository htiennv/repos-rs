use std::vec;

use anyhow::Ok;
use async_trait::async_trait;

use crate::endpoint::Endpoint;

#[derive(Debug, Clone)]
pub struct Version {
    pub opaque: String,
}

#[async_trait]
pub trait Resolver {
    fn is_constant(&self) -> bool;

    async fn resolver(&self, version: Version)
        -> anyhow::Result<(Vec<Box<dyn Endpoint>>, Version)>;
}

pub struct ConstantResolver {
    pub endpoints: Vec<Box<dyn Endpoint>>,
}

impl ConstantResolver {
    pub fn new() -> Self {
        Self {
            endpoints: Vec::new(),
        }
    }
}

#[async_trait]
impl Resolver for ConstantResolver {
    fn is_constant(&self) -> bool {
        todo!()
    }

    async fn resolver(
        &self,
        version: Version,
    ) -> anyhow::Result<(Vec<Box<dyn Endpoint>>, Version)> {
        let endpoints = vec![];

        Ok((endpoints, version))
    }
}
