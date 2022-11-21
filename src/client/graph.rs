use crate::admin::{AdminRequest, AdminRequest2};
use crate::agreement_acceptances::{AgreementAcceptancesIdRequest, AgreementAcceptancesRequest};
use crate::agreements::{AgreementsIdRequest, AgreementsRequest};
use crate::app_catalogs::AppCatalogsRequest;
use crate::applications::{ApplicationsIdRequest, ApplicationsRequest};
use crate::audit_logs::AuditLogsRequest;
use crate::authentication_method_configurations::{
    AuthenticationMethodConfigurationsIdRequest, AuthenticationMethodConfigurationsRequest,
};
use crate::authentication_methods_policy::AuthenticationMethodsPolicyRequest;
use crate::branding::BrandingRequest;
use crate::certificate_based_auth_configuration::CertificateBasedAuthConfigurationRequest;
use crate::communications::CommunicationsRequest;
use crate::contracts::ContractsRequest;
use crate::data_policy_operations::DataPolicyOperationsRequest;
use crate::device_app_management::DeviceAppManagementRequest;
use crate::device_management::DeviceManagementRequest;
use crate::directory::DirectoryRequest;
use crate::directory_objects::{DirectoryObjectsIdRequest, DirectoryObjectsRequest};
use crate::directory_role_templates::{
    DirectoryRoleTemplatesIdRequest, DirectoryRoleTemplatesRequest,
};
use crate::directory_roles::{DirectoryRolesIdRequest, DirectoryRolesRequest};
use crate::domain_dns_records::DomainDnsRecordsRequest;
use crate::domains::{DomainRequest, DomainsRequest};
use crate::drive::{DriveRequest, DrivesRequest};
use crate::education::EducationRequest;
use crate::group_lifecycle_policies::GroupLifecyclePoliciesRequest;
use crate::groups::{GroupRequest, GroupsRequest};
use crate::identity::IdentityRequest;
use crate::identity_governance::IdentityGovernanceRequest;
use crate::invitations::InvitationsRequest;
use crate::me::MeRequest;
use crate::org_contact::{OrgContactRequest, OrgContactsRequest};
use crate::places::PlacesRequest;
use crate::planner::PlannerRequest;
use crate::policies::PoliciesRequest;
use crate::reports::ReportsRequest;
use crate::schema_extensions::SchemaExtensionsRequest;
use crate::service_principals::ServicePrincipalsRequest;
use crate::sites::{SiteRequest, SitesRequest};
use crate::subscribed_skus::SubscribedSkusRequest;
use crate::subscriptions::SubscriptionsRequest;
use crate::teams::{TeamsIdRequest, TeamsRequest};
use crate::teams_templates::{TeamsTemplatesIdRequest, TeamsTemplatesRequest};
use crate::teamwork::TeamworkRequest;
use crate::users::{UserRequest, UsersRequest};
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_core::resource::ResourceIdentity;
use graph_error::GraphFailure;
use graph_http::client::Client;
use graph_http::url::GraphUrl;
use graph_http::ResourceConfigBuilder;
use graph_http::{
    types::DeltaPhantom, AsyncHttpClient, BlockingHttpClient, IntoResponse, RequestClient,
    ResourceConfig,
};
use graph_oauth::oauth::{AccessToken, OAuth};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::Method;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;

fn resource_config(resource_identity: ResourceIdentity) -> ResourceConfig {
    ResourceConfigBuilder::default()
        .resource_identity(resource_identity)
        .url(GraphUrl::parse(GRAPH_URL).unwrap())
        .build()
        .unwrap()
}

fn registry_with_id<ID: ToString>(id: ID) -> Handlebars {
    let mut registry = Handlebars::new();
    let id_owned = id.to_string();
    registry.register_helper(
        "RID",
        Box::new(
            move |_: &Helper,
                  _: &Handlebars,
                  _: &Context,
                  _: &mut RenderContext,
                  out: &mut dyn Output|
                  -> HelperResult {
                out.write(&id_owned)?;
                Ok(())
            },
        ),
    );
    registry
}

pub struct GraphV2 {
    client: Client,
}

impl GraphV2 {
    pub fn new(access_token: &str) -> GraphV2 {
        GraphV2 {
            client: Client::new(access_token),
        }
    }

    pub fn admin(&self) -> AdminRequest2 {
        AdminRequest2::new(
            self.client.clone(),
            resource_config(ResourceIdentity::Admin),
            Handlebars::new(),
        )
    }

    pub fn admin_id(&self, id: &str) -> AdminRequest2 {
        AdminRequest2::new(
            self.client.clone(),
            resource_config(ResourceIdentity::Admin),
            registry_with_id(id),
        )
    }
}

/*
pub struct Identify2<'a> {
    client: &'a Client,
    url: GraphUrl
}

impl<'a> Identify2<'a> {
    pub(crate) fn new(client: &'a Client, url: GraphUrl) -> Identify2 {
        Identify2 {
            client,
            url
        }
    }

    pub fn admin(&self) -> AdminRequest2 {
        let request_config = ResourceConfigBuilder::default()
            .resource_identity(ResourceIdentity::Admin)
            .url(self.url.clone())
            .build()
            .unwrap();
        AdminRequest2::new(self.request_handler.clone(), request_config)
    }

}

 */

/// The graph client.
///
/// The graph client supports blocking and asynchronous requests but
/// you must declare which type of client you want to use beforehand.
///
/// # Blocking - Most simple and easy to use
/// ```
/// # use graph_rs_sdk::client::Graph;
/// let _client = Graph::new("ACCESS_TOKEN");
/// ```
///
/// # Async
/// ```
/// # use graph_rs_sdk::client::Graph;
/// let _client = Graph::new_async("ACCESS_TOKEN");
/// ```
pub struct Graph<Client> {
    pub(crate) request: Client,
}

impl<'a, Client> Graph<Client>
where
    Client: graph_http::RequestClient,
{
    /// Use the V1.0 api.
    pub fn v1(&'a self) -> Identify<'a, Client> {
        self.request.set_url(GraphUrl::from_str(GRAPH_URL).unwrap());
        Identify { client: self }
    }

    /// Use the beta API.
    pub fn beta(&'a self) -> Identify<'a, Client> {
        self.request
            .set_url(GraphUrl::from_str(GRAPH_URL_BETA).unwrap());
        Identify { client: self }
    }

    pub fn custom_endpoint(&'a self, custom_endpoint: &str) -> Identify<'a, Client> {
        self.request
            .set_url(GraphUrl::from_str(custom_endpoint).unwrap());
        Identify { client: self }
    }

    /// Check if the current host is v1.0.
    pub fn is_v1(&self) -> bool {
        self.request.url().as_str().starts_with(GRAPH_URL)
    }

    /// Check if the current host is beta.
    pub fn is_beta(&self) -> bool {
        self.request.url().as_str().starts_with(GRAPH_URL_BETA)
    }

    /// Set the access token used for requests.
    pub fn set_token(&self, token: &str) {
        self.request.set_token(token);
    }

    pub fn ident(&self) -> ResourceIdentity {
        self.request.ident()
    }

    pub(crate) fn set_ident(&self, ident: ResourceIdentity) {
        self.request.set_ident(ident);
    }

    pub(crate) fn request(&self) -> &Client {
        &self.request
    }

    pub fn set_timeout(&self, duration: Duration) -> &Self {
        self.request.set_timeout(duration);
        self
    }
}

impl<Client> Debug for Graph<Client>
where
    Client: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.request.fmt(f)
    }
}

type GraphBlocking = Graph<BlockingHttpClient>;
type GraphAsync = Graph<AsyncHttpClient>;

impl GraphBlocking {
    /// Create a new client with an access token.
    ///
    /// # Example
    /// ```
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let client = Graph::new("ACCESS_TOKEN");
    /// ```
    /// ```rust,ignore
    /// // Use the v1.0 API
    ///
    /// // Returns a response object with the body
    /// // converted to serde_json::Value.
    /// let response = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .send()?;
    ///
    /// println!("{:#?}", response.body());
    ///
    /// // Use a custom data structure. The json method
    /// // will convert anything that implements serde deserialize.
    /// let drive_items: serde_json::Value = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .json()?;
    /// ```
    pub fn new(token: &str) -> GraphBlocking {
        let request = BlockingHttpClient::new(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph { request }
    }

    pub fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl),
    {
        self.request.inner_url_ref(f)
    }
}

impl From<&str> for GraphBlocking {
    fn from(token: &str) -> Self {
        Graph::new(token)
    }
}

impl From<String> for GraphBlocking {
    fn from(token: String) -> Self {
        Graph::new(token.as_str())
    }
}

impl From<&AccessToken> for GraphBlocking {
    fn from(token: &AccessToken) -> Self {
        Graph::new(token.bearer_token())
    }
}

impl TryFrom<&OAuth> for GraphBlocking {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth
            .get_access_token()
            .ok_or_else(|| GraphFailure::not_found("no access token"))?;
        Ok(Graph::from(&access_token))
    }
}

impl GraphAsync {
    /// Create a new client with an access token.
    ///
    /// # Example
    /// ```
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let client = Graph::new_async("ACCESS_TOKEN");
    /// ```
    /// ```rust,ignore
    /// // Use the v1.0 API
    ///
    /// // Returns a response object with the body
    /// // converted to serde_json::Value.
    /// let response = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .await
    ///     .send()?;
    ///
    /// println!("{:#?}", response.body());
    ///
    /// // Use a custom data structure. The json method
    /// // will convert anything that implements serde deserialize.
    /// let drive_items: serde_json::Value = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .await
    ///     .json()?;
    /// ```
    pub fn new_async(token: &str) -> GraphAsync {
        let request = AsyncHttpClient::new(GraphUrl::parse(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph { request }
    }

    pub fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl) + Sync,
    {
        self.request.url_ref(f)
    }
}

impl From<&str> for GraphAsync {
    fn from(token: &str) -> Self {
        Graph::new_async(token)
    }
}

impl From<String> for GraphAsync {
    fn from(token: String) -> Self {
        Graph::new_async(token.as_str())
    }
}

impl From<&AccessToken> for GraphAsync {
    fn from(token: &AccessToken) -> Self {
        Graph::new_async(token.bearer_token())
    }
}

impl TryFrom<&OAuth> for GraphAsync {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth
            .get_access_token()
            .ok_or_else(|| GraphFailure::not_found("no access token"))?;
        Ok(Graph::from(&access_token))
    }
}

pub struct Identify<'a, Client> {
    client: &'a Graph<Client>,
}

impl<'a, Client> Identify<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn admin(&self) -> AdminRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Admin);
        AdminRequest::new(self.client)
    }

    pub fn app_catalogs(&self) -> AppCatalogsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::AppCatalogs);
        AppCatalogsRequest::new(self.client)
    }

    pub fn agreement_acceptances(&self) -> AgreementAcceptancesRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::AgreementAcceptances);
        AgreementAcceptancesRequest::new(self.client)
    }

    pub fn agreement_acceptance<S: AsRef<str>>(
        &self,
        id: S,
    ) -> AgreementAcceptancesIdRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::AgreementAcceptances);
        AgreementAcceptancesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn agreements(&self) -> AgreementsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Agreements);
        AgreementsRequest::new(self.client)
    }

    pub fn agreement<S: AsRef<str>>(&self, id: S) -> AgreementsIdRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Agreements);
        AgreementsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn application<S: AsRef<str>>(&self, id: S) -> ApplicationsIdRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Applications);
        ApplicationsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn applications(&self) -> ApplicationsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Applications);
        ApplicationsRequest::new(self.client)
    }

    pub fn audit_logs(&self) -> AuditLogsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::AuditLogs);
        AuditLogsRequest::new(self.client)
    }

    pub fn authentication_method_configurations(
        &self,
    ) -> AuthenticationMethodConfigurationsRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::AuthenticationMethodConfigurations);
        AuthenticationMethodConfigurationsRequest::new(self.client)
    }

    pub fn authentication_method_configuration<S: AsRef<str>>(
        &self,
        id: S,
    ) -> AuthenticationMethodConfigurationsIdRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::AuthenticationMethodConfigurations);
        AuthenticationMethodConfigurationsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn authentication_methods_policy(&self) -> AuthenticationMethodsPolicyRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::AuthenticationMethodsPolicy);
        AuthenticationMethodsPolicyRequest::new(self.client)
    }

    pub fn branding(&self) -> BrandingRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Branding);
        BrandingRequest::new(self.client)
    }

    pub fn certificate_based_auth_configuration(
        &self,
    ) -> CertificateBasedAuthConfigurationRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::CertificateBasedAuthConfiguration);
        CertificateBasedAuthConfigurationRequest::new(self.client)
    }

    pub fn communications(&self) -> CommunicationsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Communications);
        CommunicationsRequest::new(self.client)
    }

    pub fn contracts(&self) -> ContractsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Contracts);
        ContractsRequest::new(self.client)
    }

    pub fn data_policy_operations(&self) -> DataPolicyOperationsRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::DataPolicyOperations);
        DataPolicyOperationsRequest::new(self.client)
    }

    pub fn device_app_management(&self) -> DeviceAppManagementRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::DeviceAppManagement);
        DeviceAppManagementRequest::new(self.client)
    }

    pub fn device_management(&self) -> DeviceManagementRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::DeviceManagement);
        DeviceManagementRequest::new(self.client)
    }

    pub fn directory(&self) -> DirectoryRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Directory);
        DirectoryRequest::new(self.client)
    }

    pub fn directory_object<S: AsRef<str>>(&self, id: S) -> DirectoryObjectsIdRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::DirectoryObjects);
        DirectoryObjectsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn directory_objects(&self) -> DirectoryObjectsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::DirectoryObjects);
        DirectoryObjectsRequest::new(self.client)
    }

    pub fn directory_role_template<S: AsRef<str>>(
        &self,
        id: S,
    ) -> DirectoryRoleTemplatesIdRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::DirectoryRoleTemplates);
        DirectoryRoleTemplatesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn directory_role_templates(&self) -> DirectoryRoleTemplatesRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::DirectoryRoleTemplates);
        DirectoryRoleTemplatesRequest::new(self.client)
    }

    pub fn directory_role<S: AsRef<str>>(&self, id: S) -> DirectoryRolesIdRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::DirectoryRoleTemplates);
        DirectoryRolesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn directory_roles(&self) -> DirectoryRolesRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::DirectoryRoleTemplates);
        DirectoryRolesRequest::new(self.client)
    }

    pub fn domain_dns_records(&self) -> DomainDnsRecordsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::DomainDnsRecords);
        DomainDnsRecordsRequest::new(self.client)
    }

    pub fn domains(&self) -> DomainRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Domains);
        DomainRequest::new(self.client)
    }

    pub fn domain<S: AsRef<str>>(&self, id: S) -> DomainsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Domains);
        DomainsRequest::new(id.as_ref(), self.client)
    }

    pub fn drives(&self) -> DriveRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Drive);
        DriveRequest::new(self.client)
    }

    pub fn drive<S: AsRef<str>>(&self, id: S) -> DrivesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Drives);
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), id.as_ref()]);
        DrivesRequest::new(id.as_ref(), self.client)
    }

    pub fn education(&self) -> EducationRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Education);
        EducationRequest::new(self.client)
    }

    pub fn groups(&self) -> GroupRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Groups);
        GroupRequest::new(self.client)
    }

    pub fn group<S: AsRef<str>>(&self, id: S) -> GroupsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Groups);
        GroupsRequest::new(id.as_ref(), self.client)
    }

    pub fn group_lifecycle_policies(&self) -> GroupLifecyclePoliciesRequest<'a, Client> {
        self.client
            .set_ident(ResourceIdentity::GroupLifecyclePolicies);
        GroupLifecyclePoliciesRequest::new(self.client)
    }

    pub fn identity(&self) -> IdentityRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Identity);
        IdentityRequest::new(self.client)
    }

    pub fn identity_governance(&self) -> IdentityGovernanceRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::IdentityGovernance);
        IdentityGovernanceRequest::new(self.client)
    }

    pub fn invitations(&self) -> InvitationsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Invitations);
        InvitationsRequest::new(self.client)
    }

    pub fn me(&self) -> MeRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Me);
        MeRequest::new(self.client)
    }

    pub fn org_contact<S: AsRef<str>>(&self, id: S) -> OrgContactsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::OrgContact);
        OrgContactsRequest::new(id.as_ref(), self.client)
    }

    pub fn org_contacts(&self) -> OrgContactRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::OrgContact);
        OrgContactRequest::new(self.client)
    }

    pub fn places(&self) -> PlacesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Places);
        PlacesRequest::new(self.client)
    }

    pub fn planner(&self) -> PlannerRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Planner);
        PlannerRequest::new(self.client)
    }

    pub fn policies(&self) -> PoliciesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Policies);
        PoliciesRequest::new(self.client)
    }

    pub fn reports(&self) -> ReportsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Reports);
        ReportsRequest::new(self.client)
    }

    pub fn schema_extensions(&self) -> SchemaExtensionsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::SchemaExtensions);
        SchemaExtensionsRequest::new(self.client)
    }

    pub fn service_principals(&self) -> ServicePrincipalsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::ServicePrincipals);
        ServicePrincipalsRequest::new(self.client)
    }

    pub fn subscribed_skus(&self) -> SubscribedSkusRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::SubscribedSkus);
        SubscribedSkusRequest::new(self.client)
    }

    pub fn subscriptions(&self) -> SubscriptionsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Subscriptions);
        SubscriptionsRequest::new(self.client)
    }

    pub fn site<S: AsRef<str>>(&self, id: S) -> SitesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Sites);
        SitesRequest::new(id.as_ref(), self.client)
    }

    pub fn sites(&self) -> SiteRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Sites);
        SiteRequest::new(self.client)
    }

    pub fn teamwork(&self) -> TeamworkRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Teamwork);
        TeamworkRequest::new(self.client)
    }

    pub fn team<S: AsRef<str>>(&self, id: S) -> TeamsIdRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Teams);
        TeamsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn teams(&self) -> TeamsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Teams);
        TeamsRequest::new(self.client)
    }

    pub fn teams_template<S: AsRef<str>>(&self, id: S) -> TeamsTemplatesIdRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::TeamsTemplates);
        TeamsTemplatesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn teams_templates(&self) -> TeamsTemplatesRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::TeamsTemplates);
        TeamsTemplatesRequest::new(self.client)
    }

    pub fn user<S: AsRef<str>>(&self, id: S) -> UsersRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Users);
        UsersRequest::new(id.as_ref(), self.client)
    }

    pub fn users(&self) -> UserRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Users);
        UserRequest::new(self.client)
    }

    /// Perform a batch requests which can store multiple requests
    /// in the request body.
    pub fn batch<B: serde::Serialize>(
        &self,
        batch: &B,
    ) -> IntoResponse<'a, DeltaPhantom<serde_json::Value>, Client> {
        let client = self.client.request();
        client.set_method(Method::POST);
        client.header(ACCEPT, HeaderValue::from_static("application/json"));
        let body = serde_json::to_string(batch).map_err(GraphFailure::from);
        if let Err(err) = body {
            return IntoResponse::new_error(self.client.request(), err);
        } else if let Ok(body) = body {
            client.set_body(body);
        }
        render_path!(self.client, "$batch", &serde_json::json!({}));
        IntoResponse::new(&self.client.request)
    }
}
