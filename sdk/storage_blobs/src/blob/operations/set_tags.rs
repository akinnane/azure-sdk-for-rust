use crate::prelude::*;
use azure_core::prelude::*;
use azure_core::{
    headers::{date_from_headers, request_id_from_headers, Headers},
    Method, RequestId,
};
use chrono::{DateTime, Utc};
use std::convert::{TryFrom, TryInto};

operation! {
    SetTags,
    client: BlobClient,
    tags: Tags,
    ?lease_id: LeaseId
}

impl SetTagsBuilder {
    pub fn into_future(mut self) -> SetTags {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "tags");

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            let body = self.tags.to_xml()?;

            let mut request =
                self.client
                    .finalize_request(url, Method::Put, headers, Some(body.into()))?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            response.headers().try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct SetTagsResponse {
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl TryFrom<&Headers> for SetTagsResponse {
    type Error = crate::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        Ok(SetTagsResponse {
            request_id: request_id_from_headers(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}
