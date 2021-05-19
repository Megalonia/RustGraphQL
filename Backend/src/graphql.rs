use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fmt::Formatter;
use std::fmt::LowerExp;
use std::iter::Iterator;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::guard::Guard;
use async_graphql::*;
use bigdecimal::{BigDecimal, ToPrimitive};
use futures::{Stream, StreamExt};
use rdkafka::{producer::FutureProducer, Message};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};



#[Object]
impl Query {
    async fn get_crop(&self, ctx: &Context<'_>) -> Vec<Crop> {
        repos
    }
}


