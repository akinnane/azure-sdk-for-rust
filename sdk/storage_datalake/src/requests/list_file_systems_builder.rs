use crate::clients::DataLakeClient;
use crate::responses::*;
use azure_core::prelude::*;
use azure_core::{headers::add_optional_header, AppendToUrlQuery};
use futures::stream::{unfold, Stream};
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListFileSystemsBuilder<'a> {
    data_lake_client: &'a DataLakeClient,
    prefix: Option<Prefix>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> ListFileSystemsBuilder<'a> {
    pub(crate) fn new(data_lake_client: &'a DataLakeClient) -> Self {
        Self {
            data_lake_client,
            prefix: None,
            next_marker: None,
            max_results: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<ListFileSystemsResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = url::Url::parse(self.data_lake_client.url())?;

        url.query_pairs_mut().append_pair("resource", "account");

        self.prefix.append_to_url_query(&mut url);
        self.max_results.append_to_url_query(&mut url);
        if let Some(nm) = self.next_marker.as_ref() {
            nm.append_to_url_query_as_continuation(&mut url)
        }
        self.timeout.append_to_url_query(&mut url);

        debug!("list filesystems url = {}", url);

        let request = self.data_lake_client.prepare_request(
            url.as_str(),
            &Method::GET,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        trace!("request == {:?}", request);

        let response = self
            .data_lake_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        Ok((&response).try_into()?)
    }

    pub fn stream(
        self,
    ) -> impl Stream<Item = Result<ListFileSystemsResponse, Box<dyn std::error::Error + Sync + Send>>> + 'a
    {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(NextMarker),
        }

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                debug!("next_marker == {:?}", &next_marker);
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::NextMarker(next_marker)) => {
                        req.next_marker(next_marker).execute().await
                    }
                    None => return None,
                };

                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = response.next_marker.clone().map(States::NextMarker);

                Some((Ok(response), next_marker))
            }
        })
    }
}
