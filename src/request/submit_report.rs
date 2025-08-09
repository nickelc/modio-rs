use std::future::IntoFuture;

use serde_derive::Serialize;

use crate::client::Client;
use crate::request::{Output, RequestBuilder, Route};
use crate::response::ResponseFuture;
use crate::types::id::ResourceId;
use crate::types::Message;

/// Report a game, guide, mod or user.
pub struct SubmitReport<'a> {
    http: &'a Client,
    fields: SubmitReportFields<'a>,
}

#[derive(Serialize)]
struct SubmitReportFields<'a> {
    resource: &'a str,
    id: ResourceId,
    #[serde(rename = "type")]
    kind: u8,
    summary: &'a str,
    name: Option<&'a str>,
    contact: Option<&'a str>,
}

impl<'a> SubmitReport<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        resource: &'a str,
        id: ResourceId,
        kind: u8,
        summary: &'a str,
    ) -> Self {
        Self {
            http,
            fields: SubmitReportFields {
                resource,
                id,
                kind,
                summary,
                name: None,
                contact: None,
            },
        }
    }

    /// Name of the user submitting the report.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);
        self
    }

    /// Contact details of the user submitting the report.
    pub const fn contact(mut self, contact: &'a str) -> Self {
        self.fields.contact = Some(contact);
        self
    }
}

impl IntoFuture for SubmitReport<'_> {
    type Output = Output<Message>;
    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let route = Route::SubmitReport;
        match RequestBuilder::from_route(&route).form(&self.fields) {
            Ok(req) => self.http.request(req),
            Err(err) => ResponseFuture::failed(err),
        }
    }
}
