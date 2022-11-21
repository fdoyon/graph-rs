//! ### Microsoft Graph API Client in Rust
//! graph-rs is an API client for Microsoft Graph V1.0 and Graph Beta.
//!
//! Installation and basic usage can be found below and there are extensive examples in the example's directory
//! on GitHub](https://github.com/sreeise/graph-rs).
//!
//! ### What Api's are available
//!
//! The Api's available are generated from Microsoft's msgraph-metadata repository which stores OpenApi configs for the
//! Graph Api. There may be some requests and/or Api's not yet included in this project but in general most of them are
//! implemented.
//!
//! ### Feature requests or Bug reports.
//!
//! For both feature requests and bug reports please file an issue on
//! [GitHub](https://github.com/sreeise/graph-rs)
//! and a response or fix will be done as soon as possible.
//!
//! ### Use
//!
//! The client supports both blocking and async requests.
//!
//! ### Blocking Client
//!
//! To use the blocking client
//! ```rust
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new("ACCESS_TOKEN");
//! ```
//!
//! ### Async Client
//!
//! To use the async client
//! ```rust
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new_async("ACCESS_TOKEN");
//! ```
//!
//! #### The send method and Graph types
//! The send() method is the main method for sending a request. If there are no errors the return value will be wrapped
//! in a response object and the body will be one of:
//!
//! 1. [`serde_json::Value`]
//!
//! 2. NoContent (204 responses that return a content field) This is always represented as a [`serde_json::Value::String`]
//!     and sometimes includes some basic info depending on the request.
//!
//! # Basic Use:
//! ```rust,ignore
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new("ACCESS_TOKEN");
//!
//! let response = client.v1()
//!     .me()
//!     .drive()
//!     .get_drive()
//!     .send()
//!     .unwrap();
//!
//! // Print the value returned in the body of the response
//! println!("{:#?}", response.body());
//! ```
//!
//! # Using the Async Client
//!
//! ```rust,ignore
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new_async("ACCESS_TOKEN");
//!
//! let response = client.v1()
//!     .me()
//!     .drive()
//!     .get_drive()
//!     .send()
//!     .await
//!     .unwrap();
//!
//! println!("{:#?}", response);
//! ```
//!
//! ### Use the Graph version one or beta Api
//! v1() refers to the endpoint for version 1 of the Microsoft graph Api. You can also
//! use the beta() method which uses the Microsoft graph beta Api endpoint.
//!
//! ```rust,ignore
//! use graph_rs_sdk::prelude::*;
//!
//! let client =  Graph::new_async("ACCESS_TOKEN");
//!
//! let _response = client.beta()
//!     .me()
//!     .get_user()
//!     .send()?;
//! ```
//!
//! - For more information and examples please see the repository on
//! [GitHub](https://github.com/sreeise/graph-rs)
//! - If you run into issues related to graph-rs specifically please
//! file an issue on [GitHub](https://github.com/sreeise/graph-rs)

extern crate graph_core;
extern crate graph_error;
extern crate graph_oauth;
extern crate handlebars;
extern crate reqwest;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate serde_yaml;
extern crate strum;

// mod client needs to stay on top of all other
// client mod declarations for macro use.
/// Main Graph client.
#[macro_use]
pub mod client;
/// Access Package Assignment Approvals request client (Identity governance)
pub mod access_package_assignment_approvals;
/// Access Packages request client (Identity Governance)
pub mod access_packages;
/// Access Review Definitions request client (Identity Governance)
pub mod access_review_definitions;
/// Access Reviews request client (Identity Governance).
pub mod access_reviews;
/// Activities request client.
pub mod activities;
/// Admin request client.
pub mod admin;
/// Administrative Units request client (Directory)
pub mod administrative_units;
/// Administrative Units Members (Directory administrativeUnits/members)
pub mod administrative_units_members;
/// Agreement acceptances request client.
pub mod agreement_acceptances;
/// Agreements client
pub mod agreements;
/// App catalogs request client.
pub mod app_catalogs;
/// App Consent request client (Identity Governance).
pub mod app_consent;
/// Applications request client.
pub mod applications;
/// Assignment Policies client (Identity Governance)
pub mod assignment_policies;
/// Assignment Requests client. (Identity Governance)
pub mod assignment_requests;
/// Assignments request client (Identity Governance)
pub mod assignments;
/// Attachments request client.
pub mod attachments;
/// Audit logs request client.
pub mod audit_logs;
/// Authentication Method Configurations request client.
pub mod authentication_method_configurations;
/// Authentication Methods Policy request client.
pub mod authentication_methods_policy;
/// Branding request client.
pub mod branding;
/// Planner buckets request client.
pub mod buckets;
/// Calendar request client.
pub mod calendar;
/// Calendar groups client.
pub mod calendar_groups;
/// Calendar view client.
pub mod calendar_view;
/// Communication call records request client.
pub mod call_records;
/// Communication calls request client.
pub mod calls;
/// Certificate based auth configuration client.
pub mod certificate_based_auth_configuration;
/// Channels request client. (Teams, Sites)
pub mod channels;
/// Chats or Channel Messages request client.
pub mod chats_channels_messages;
/// Mail folders child folders request client.
pub mod child_folders;
/// Communications client.
pub mod communications;
/// Connected Organizations client (Identity Governments - Entitlement Management)
pub mod connected_organizations;
/// Contact folders client (me, users, etc.).
pub mod contact_folders;
/// Contacts request client.
pub mod contacts;
/// Content types request client.
pub mod content_types;
/// Contracts request client.
pub mod contracts;
/// Groups conversations request client.
pub mod conversations;
/// Data policy operations request client.
pub mod data_policy_operations;
/// Definition Instance Stages request client (Identity Governance)
pub mod definition_instance_stages;
/// Definition Instances request client (Identity Governance)
pub mod definition_instances;
/// Device app management request client.
pub mod device_app_management;
/// Device management request client.
pub mod device_management;
/// Directory request client.
pub mod directory;
pub mod directory_deleted_items;
pub mod directory_objects;
pub mod directory_role_templates;
pub mod directory_roles;
/// Domain dns records request client.
pub mod domain_dns_records;
/// Domains request client.
pub mod domains;
/// OneDrive request client.
pub mod drive;
/// Education request client.
pub mod education;
/// Entitlement Management client (Identity Governance)
pub mod entitlement_management;
/// Events request client (Calendars).
pub mod events;
/// Extended properties request client.
pub mod extended_properties;
/// Group lifecycle policies request client.
pub mod group_lifecycle_policies;
/// Groups request client.
pub mod groups;
/// Health Overviews request client
pub mod health_overviews;
/// Identity request client.
pub mod identity;
/// Identity Governance client.
pub mod identity_governance;
/// Inference classification client (me, users, etc.).
pub mod inference_classification;
/// Insights client (me, users, etc.).
pub mod insights;
/// Instances request client (events and calendarView).
pub mod instances;
/// Invitations request client.
pub mod invitations;
/// Items request client.
pub mod items;
/// Lists request client.
pub mod lists;
/// Mail folders request client.
pub mod mail_folders;
/// Managed devices client.
pub mod managed_devices;
/// Me request client.
pub mod me;
// Members request client (teams)
pub mod members;
/// Messages request client.
pub mod messages;
/// Notebooks request client.
pub mod notebooks;
/// OneNote request client.
pub mod onenote;
/// Contacts/Org Contacts request client.
pub mod org_contact;
/// Outlook request client.
pub mod outlook;
/// Onenote pages request client.
pub mod pages;
/// Onenote parent notebook request client.
pub mod parent_notebook;
/// Onenote parent section request client.
pub mod parent_section;
/// Onenote parent section group request client.
pub mod parent_section_group;
/// Places request client.
pub mod places;
/// Planner request client.
pub mod planner;
/// Planner plans request client.
pub mod plans;
/// Policies request client.
pub mod policies;
/// Groups threads posts request client.
pub mod posts;
/// Primary channels request client.
pub mod primary_channel;
/// Reports request client.
pub mod reports;
/// Schedule request client (teams).
pub mod schedule;
/// Schema extensions request client.
pub mod schema_extensions;
/// Onenote section group request client.
pub mod section_groups;
/// Onenote section request client.
pub mod sections;
/// Service announcement request client.
pub mod service_announcement;
/// Service principles request client.
pub mod service_principals;
/// Communications call record sessions request client.
pub mod sessions;
/// Settings request client (me, users, etc.).
pub mod settings;
/// Shared With Teams request client (Teams channels and primary channel path)
pub mod shared_with_teams;
/// Sites request client.
pub mod sites;
/// Subscribed skus request client.
pub mod subscribed_skus;
/// Subscriptions request client.
pub mod subscriptions;
/// Tabs request client.
pub mod tabs;
/// Planner tasks request client.
pub mod tasks;
/// Teams request client.
pub mod teams;
/// Teams templates request client.
pub mod teams_templates;
/// Teamwork request client.
pub mod teamwork;
/// Terms Of Use client (Identity Governance)
pub mod terms_of_use;
/// Groups thread request client.
pub mod threads;
/// Users request client.
pub mod users;

pub static GRAPH_URL: &str = "https://graph.microsoft.com/v1.0";
pub static GRAPH_URL_BETA: &str = "https://graph.microsoft.com/beta";

/// Common structs and traits.
pub mod prelude {
    pub use crate::client::*;
    pub use graph_http::types::Delta;
    pub use graph_http::GraphResponse;
}

/// Reexport of graph-oauth crate.
pub mod oauth {
    pub use graph_oauth::jwt;
    pub use graph_oauth::oauth::*;
}

/// Reexport of graph-http crate.
pub mod http {
    pub use graph_http::*;
}

/// Reexport of graph-error crate.
pub mod error {
    pub use graph_error::*;
}

/// Reexport of reqwest headers for use with API requests.
pub mod header {
    pub use reqwest::header::*;
}

/// Types used across multiple crates.
pub mod core {
    pub use graph_core::resource::*;
}

pub(crate) mod api_default_imports {
    pub use crate::client::Graph;
    pub use crate::core::ResourceIdentity;
    pub use graph_http::IntoResponse;
    pub use graph_http::RequestClient;
    pub use graph_http::{client, ResourceConfig, ResponseHandler};
    pub use handlebars::*;
    pub use reqwest::Method;
}
