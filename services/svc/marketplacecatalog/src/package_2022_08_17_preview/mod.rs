#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://catalogapi.azure.com";
impl ClientBuilder {
    #[doc = "Create a new instance of `ClientBuilder`."]
    #[must_use]
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
            options: azure_core::ClientOptions::default(),
        }
    }
    #[doc = "Set the endpoint."]
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    #[doc = "Set the scopes."]
    #[must_use]
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    #[doc = "Set the retry options."]
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }
    #[doc = "Set the transport options."]
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
    #[doc = "Convert the builder into a `Client` instance."]
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{endpoint}/")]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let context = azure_core::Context::default();
        self.pipeline.send(&context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn search_client(&self) -> search::Client {
        search::Client(self.clone())
    }
}
pub mod search {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of azure marketplace catalog offers and total count and facets"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `language`: Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'"]
        #[doc = "* `select`: Array of fields to return. Such as 'pricingTypes'. \n- `All`: All fields.\n- `Popularity`: Popularity.\n- `ApplicableProducts`: Applicable Products.\n- `CategoryIds`: Category Ids.\n- `Market`: Market.\n- `LinkedAddIns`: Linked AddIns.\n- `SupportedProducts`: Supported Products.\n- `HideKeys`: Hide Keys.\n- `PublisherId`: Publisher Id.\n- `CspStates`: Csp States.\n- `DisplayName`: Display Name.\n- `AzureBenefit`: Azure Benefit.\n- `Badges`: Badges.\n- `SmallIconUri`: Small Icon Uri.\n- `MediumIconUri`: Medium Icon Uri.\n- `LargeIconUri`: Large Icon Uri.\n- `WideIconUri`: Wide Icon Uri.\n- `IndustryCloud`: Industry Cloud.\n- `PublisherType`: Publisher Type.\n- `PublishingState`: Publishing State.\n- `Language`: Language.\n- `UniqueProductId`: Unique Product Id.\n- `ProductType`: Product Type.\n- `Plans`: Plans.\n- `OperatingSystems`: Operating Systems.\n- `PricingTypes`: Pricing Types.\n- `PublisherDisplayName`: Publisher Display Name.\n- `Summary`: Summary.\n- `VmImageGenerations`: Vm Image Generations.\n- `VmSecurityTypes`: Vm Security Types.\n- `VmArchitectureTypes`: Vm Architecture Types.\n- `Description`: Description.\n- `RatingBuckets`: Rating Buckets.\n- `RatingAverage`: Rating Average."]
        #[doc = "* `market`: Product sold market, Possible values - https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies. Such as 'US'"]
        #[doc = "* `x_ms_app`: The Application named property has the name of the client application that makes the request, used for tracing. Such as x-ms-app=appsource"]
        pub fn get(
            &self,
            language: impl Into<String>,
            select: Vec<String>,
            market: impl Into<String>,
            x_ms_app: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                language: language.into(),
                select,
                market: market.into(),
                x_ms_app: x_ms_app.into(),
                search_query: None,
                publisher_display_name: None,
                azure_benefit: None,
                publisher_types: Vec::new(),
                badges: Vec::new(),
                industry_cloud: None,
                gallery: None,
                orderby: Vec::new(),
                product_types: Vec::new(),
                pricing_types: Vec::new(),
                industries: Vec::new(),
                operating_systems: Vec::new(),
                hide_keys: Vec::new(),
                category_ids: Vec::new(),
                linked_add_ins: Vec::new(),
                supported_products: Vec::new(),
                applicable_products: Vec::new(),
                publisher_ids: Vec::new(),
                rating_buckets: Vec::new(),
                vm_image_generations: Vec::new(),
                vm_architecture_types: Vec::new(),
                vm_security_types: Vec::new(),
                publishing_stage: None,
                facets: Vec::new(),
                skip: None,
                top: None,
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SearchResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SearchResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) language: String,
            pub(crate) select: Vec<String>,
            pub(crate) market: String,
            pub(crate) x_ms_app: String,
            pub(crate) search_query: Option<String>,
            pub(crate) publisher_display_name: Option<String>,
            pub(crate) azure_benefit: Option<String>,
            pub(crate) publisher_types: Vec<String>,
            pub(crate) badges: Vec<String>,
            pub(crate) industry_cloud: Option<String>,
            pub(crate) gallery: Option<String>,
            pub(crate) orderby: Vec<String>,
            pub(crate) product_types: Vec<String>,
            pub(crate) pricing_types: Vec<String>,
            pub(crate) industries: Vec<String>,
            pub(crate) operating_systems: Vec<String>,
            pub(crate) hide_keys: Vec<String>,
            pub(crate) category_ids: Vec<String>,
            pub(crate) linked_add_ins: Vec<String>,
            pub(crate) supported_products: Vec<String>,
            pub(crate) applicable_products: Vec<String>,
            pub(crate) publisher_ids: Vec<String>,
            pub(crate) rating_buckets: Vec<String>,
            pub(crate) vm_image_generations: Vec<String>,
            pub(crate) vm_architecture_types: Vec<String>,
            pub(crate) vm_security_types: Vec<String>,
            pub(crate) publishing_stage: Option<String>,
            pub(crate) facets: Vec<String>,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The search text. Such as searchQuery=red hat"]
            pub fn search_query(mut self, search_query: impl Into<String>) -> Self {
                self.search_query = Some(search_query.into());
                self
            }
            #[doc = "The search publisher name. Such as publisherDisplayName=red hat"]
            pub fn publisher_display_name(mut self, publisher_display_name: impl Into<String>) -> Self {
                self.publisher_display_name = Some(publisher_display_name.into());
                self
            }
            #[doc = "Define the search for only azure benefit eligible offers, if no value selected, this filter is ignored. Default: null. \n- `Eligible`: Eligible.\n- `NotEligible`: Not Eligible."]
            pub fn azure_benefit(mut self, azure_benefit: impl Into<String>) -> Self {
                self.azure_benefit = Some(azure_benefit.into());
                self
            }
            #[doc = "Define the search publisher types, Possible values: Microsoft, ThirdParty. Default: Microsoft. \n- `Microsoft`: Microsoft.\n- `ThirdParty`: Third Party."]
            pub fn publisher_types(mut self, publisher_types: Vec<String>) -> Self {
                self.publisher_types = publisher_types;
                self
            }
            #[doc = "The following product badges are available: \n- `PreferredSolution`: Preferred Solution.\n- `PowerBICertified`: power BI Certified.\n- `AdditionalPurchaseRequirement`: Additional Purchase Requirement."]
            pub fn badges(mut self, badges: Vec<String>) -> Self {
                self.badges = badges;
                self
            }
            #[doc = "Define the search for IndustryCloud offers. Default: NotApplicable. \n- `NotApplicable`: Not Applicable.\n- `True`: True.\n- `False`: False."]
            pub fn industry_cloud(mut self, industry_cloud: impl Into<String>) -> Self {
                self.industry_cloud = Some(industry_cloud.into());
                self
            }
            #[doc = "Gallery to search by. Such as Azure"]
            pub fn gallery(mut self, gallery: impl Into<String>) -> Self {
                self.gallery = Some(gallery.into());
                self
            }
            #[doc = "Array of sort by fields to order by, if none selected then ignored. For example, orderby=Popularity"]
            pub fn orderby(mut self, orderby: Vec<String>) -> Self {
                self.orderby = orderby;
                self
            }
            #[doc = "Array of product types to search by. Such as 'VirtualMachine'.\n- `None`: None.\n- `DevService`: Dev Service.\n- `ManagedApplication`: Managed Application.\n- `VirtualMachine`: Virtual Machine.\n- `AzureApplication`: Azure Application.\n- `Container`: Container.\n- `SaaS`: SaaS.\n- `SolutionTemplate`: Solution Template.\n- `IotEdgeModules`: Iot Edge Modules.\n- `ManagedServices`: Managed Services.\n- `ContainerApps`: Container Apps.\n- `VisualStudioExtension`: Visual Studio Extension.\n- `DynamicsOps`: Dynamics Ops.\n- `DynamicsCE`: Dynamics CE.\n- `DynamicsBC`: Dynamics BC.\n- `PowerBI`: PowerBI.\n- `ConsultingServices`: Consulting Services.\n- `CosellOnly`: Cosell Only.\n- `CoreVirtualMachine`: Core Virtual Machine.\n- `PowerBIVisuals`: PowerBI Visuals.\n- `Office365`: Office365.\n- `AADApps`: Azure Active Directory Apps.\n- `AzureServices`: Azure Services"]
            pub fn product_types(mut self, product_types: Vec<String>) -> Self {
                self.product_types = product_types;
                self
            }
            #[doc = "Array of pricing types to search by. \n- `Free`: The product has at least one plan that is free of charge.\n- `FreeTrial`: The product has at least one plan that is free trial.\n- `Byol`: The product has at least one plan that is bring your own license.\n- `Payg`: The product has at least one plan that is Pay as you go, usage based billing model.\n- `RI`: The product has at least one plan that is Reserved Instance billing model."]
            pub fn pricing_types(mut self, pricing_types: Vec<String>) -> Self {
                self.pricing_types = pricing_types;
                self
            }
            #[doc = "Array of Product industries, Possible values - https://docs.microsoft.com/en-us/azure/marketplace/marketplace-categories-industries#industries. Such as 'retailers'"]
            pub fn industries(mut self, industries: Vec<String>) -> Self {
                self.industries = industries;
                self
            }
            #[doc = "Array of operating systems to search by, if none selected then filter is ignored, this is relevant for Virtual Machine product type only. Such as operatingSystems=windows,linux"]
            pub fn operating_systems(mut self, operating_systems: Vec<String>) -> Self {
                self.operating_systems = operating_systems;
                self
            }
            #[doc = "Array of hideKeys for preview search by. Such as '22c6b3ae-1111-1111-1111-e7cbdc8569dd'"]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "Array of product categories, https://docs.microsoft.com/en-us/azure/marketplace/marketplace-categories-industries#categories. Such as 'Productivity'"]
            pub fn category_ids(mut self, category_ids: Vec<String>) -> Self {
                self.category_ids = category_ids;
                self
            }
            #[doc = "Array of linkedAddIns to search by, For SaaS products only, https://docs.microsoft.com/en-us/azure/marketplace/monetize-addins-through-microsoft-commercial-marketplace."]
            pub fn linked_add_ins(mut self, linked_add_ins: Vec<String>) -> Self {
                self.linked_add_ins = linked_add_ins;
                self
            }
            #[doc = "Array of supported products to search by. Such as 'CRM'"]
            pub fn supported_products(mut self, supported_products: Vec<String>) -> Self {
                self.supported_products = supported_products;
                self
            }
            #[doc = "Array of M365 products applicable. Such as 'Office365'"]
            pub fn applicable_products(mut self, applicable_products: Vec<String>) -> Self {
                self.applicable_products = applicable_products;
                self
            }
            #[doc = "Array of publisher ids to return. Such as 'Microsoft'"]
            pub fn publisher_ids(mut self, publisher_ids: Vec<String>) -> Self {
                self.publisher_ids = publisher_ids;
                self
            }
            #[doc = "Array of azure portal rating buckets to search by. Such as 'Above1'.\n- `AboveOne`: Above One.\n- `AboveTwo`: Above Two.\n- `AboveThree`: Above Three.\n- `AboveFour`: Above Four"]
            pub fn rating_buckets(mut self, rating_buckets: Vec<String>) -> Self {
                self.rating_buckets = rating_buckets;
                self
            }
            #[doc = "Array of vm generations to search by, See Azure support for generation 2 VMs - Azure Virtual Machines | Microsoft Docs, Possible values, 1,2. Such as '1'"]
            pub fn vm_image_generations(mut self, vm_image_generations: Vec<String>) -> Self {
                self.vm_image_generations = vm_image_generations;
                self
            }
            #[doc = "Array of Virtual Machine image architecture types to search by, 1-x64, 2-ARM. Such as '1'"]
            pub fn vm_architecture_types(mut self, vm_architecture_types: Vec<String>) -> Self {
                self.vm_architecture_types = vm_architecture_types;
                self
            }
            #[doc = "Array of vm security types to search by. Such as 'Trusted'.\n- `None`: None.\n- `Trusted`: Trusted.\n- `Confidential`: Confidential"]
            pub fn vm_security_types(mut self, vm_security_types: Vec<String>) -> Self {
                self.vm_security_types = vm_security_types;
                self
            }
            #[doc = "Audience. Default: Preview.\n- `Preview`: Preview.\n- `Public`: Public"]
            pub fn publishing_stage(mut self, publishing_stage: impl Into<String>) -> Self {
                self.publishing_stage = Some(publishing_stage.into());
                self
            }
            #[doc = "Array facets to facet by, if none selected then no facets will return. Such as facets=pricingTypes,operatingSystems. \n- `All`: All fields.\n- `Popularity`: Popularity.\n- `ApplicableProducts`: Applicable Products.\n- `CategoryIds`: Category Ids.\n- `Market`: Market.\n- `LinkedAddIns`: Linked AddIns.\n- `SupportedProducts`: Supported Products.\n- `HideKeys`: Hide Keys.\n- `PublisherId`: Publisher Id.\n- `CspStates`: Csp States.\n- `DisplayName`: Display Name.\n- `AzureBenefit`: Azure Benefit.\n- `Badges`: Badges.\n- `SmallIconUri`: Small Icon Uri.\n- `MediumIconUri`: Medium Icon Uri.\n- `LargeIconUri`: Large Icon Uri.\n- `WideIconUri`: Wide Icon Uri.\n- `IndustryCloud`: Industry Cloud.\n- `PublisherType`: Publisher Type.\n- `PublishingState`: Publishing State.\n- `Language`: Language.\n- `UniqueProductId`: Unique Product Id.\n- `ProductType`: Product Type.\n- `Plans`: Plans.\n- `OperatingSystems`: Operating Systems.\n- `PricingTypes`: Pricing Types.\n- `PublisherDisplayName`: Publisher Display Name.\n- `Summary`: Summary.\n- `VmImageGenerations`: Vm Image Generations.\n- `VmSecurityTypes`: Vm Security Types.\n- `VmArchitectureTypes`: Vm Architecture Types.\n- `Description`: Description.\n- `RatingBuckets`: Rating Buckets.\n- `RatingAverage`: Rating Average."]
            pub fn facets(mut self, facets: Vec<String>) -> Self {
                self.facets = facets;
                self
            }
            #[doc = "Number of items to skip. Minimum: 0"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "Number of items to return. Minimum: 0, Maximum: 50000, Default: 20."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(search_query) = &this.search_query {
                            req.url_mut().query_pairs_mut().append_pair("searchQuery", search_query);
                        }
                        if let Some(publisher_display_name) = &this.publisher_display_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherDisplayName", publisher_display_name);
                        }
                        if let Some(azure_benefit) = &this.azure_benefit {
                            req.url_mut().query_pairs_mut().append_pair("azureBenefit", azure_benefit);
                        }
                        if let Some(industry_cloud) = &this.industry_cloud {
                            req.url_mut().query_pairs_mut().append_pair("industryCloud", industry_cloud);
                        }
                        let language = &this.language;
                        req.url_mut().query_pairs_mut().append_pair("language", language);
                        if let Some(gallery) = &this.gallery {
                            req.url_mut().query_pairs_mut().append_pair("gallery", gallery);
                        }
                        let market = &this.market;
                        req.url_mut().query_pairs_mut().append_pair("market", market);
                        if let Some(publishing_stage) = &this.publishing_stage {
                            req.url_mut().query_pairs_mut().append_pair("publishingStage", publishing_stage);
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut().query_pairs_mut().append_pair("skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("top", &top.to_string());
                        }
                        req.insert_header("x-ms-app", &this.x_ms_app);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/search", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2022-08-17-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SearchResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SearchResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
