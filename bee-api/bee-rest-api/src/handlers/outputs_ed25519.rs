// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    filters::CustomRejection::ServiceUnavailable,
    handlers::{EnvelopeContent, SuccessEnvelope},
    storage::Backend,
};

use bee_common_pt2::node::ResHandle;
use bee_message::prelude::*;
use bee_storage::access::Fetch;

use serde::Serialize;
use warp::{reject, Rejection, Reply};

use std::ops::Deref;

pub(crate) async fn outputs_ed25519<B: Backend>(
    addr: Ed25519Address,
    storage: ResHandle<B>,
) -> Result<impl Reply, Rejection> {
    let mut fetched = match Fetch::<Ed25519Address, Vec<OutputId>>::fetch(storage.deref(), &addr)
        .await
        .map_err(|_| reject::custom(ServiceUnavailable("can not fetch from storage".to_string())))?
    {
        Some(ids) => ids,
        None => vec![],
    };

    let count = fetched.len();
    let max_results = 1000;
    fetched.truncate(max_results);

    Ok(warp::reply::json(&SuccessEnvelope::new(GetOutputsForAddressResponse {
        address: addr.to_string(),
        max_results,
        count,
        output_ids: fetched.iter().map(|id| id.to_string()).collect(),
    })))
}

/// Response of GET /api/v1/addresses/{address}/outputs
#[derive(Clone, Debug, Serialize)]
pub struct GetOutputsForAddressResponse {
    pub address: String,
    #[serde(rename = "maxResults")]
    pub max_results: usize,
    pub count: usize,
    #[serde(rename = "outputIds")]
    pub output_ids: Vec<String>,
}

impl EnvelopeContent for GetOutputsForAddressResponse {}