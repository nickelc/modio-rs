//! Reports interface
use url::form_urlencoded;

use crate::prelude::*;

pub struct Reports {
    modio: Modio,
}

impl Reports {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// Submit a report for any resource on mod.io. [required: token]
    pub async fn submit(self, report: Report) -> Result<()> {
        self.modio
            .request(Route::SubmitReport)
            .body(report.to_query_string())
            .send::<Message>()
            .await?;
        Ok(())
    }
}

pub struct Report {
    pub name: String,
    pub contact: Option<String>,
    pub summary: String,
    pub kind: ReportType,
    pub resource: Resource,
}

pub enum ReportType {
    Generic,
    DMCA,
}

pub enum Resource {
    Game(u32),
    Mod(u32),
    User(u32),
}

impl Report {
    pub fn new<S: Into<String>>(
        name: S,
        contact: Option<S>,
        summary: S,
        kind: ReportType,
        resource: Resource,
    ) -> Self {
        Self {
            name: name.into(),
            contact: contact.map(Into::into),
            summary: summary.into(),
            kind,
            resource,
        }
    }
}

impl QueryString for Report {
    fn to_query_string(&self) -> String {
        let (resource, id) = match self.resource {
            Resource::Game(id) => ("games", id),
            Resource::Mod(id) => ("mods", id),
            Resource::User(id) => ("users", id),
        };
        let _type = match self.kind {
            ReportType::Generic => 0,
            ReportType::DMCA => 1,
        };
        form_urlencoded::Serializer::new(String::new())
            .extend_pairs(self.contact.as_ref().map(|c| ("contact", c)))
            .append_pair("resource", resource)
            .append_pair("id", &id.to_string())
            .append_pair("type", &_type.to_string())
            .append_pair("name", &self.name)
            .append_pair("summary", &self.summary)
            .finish()
    }
}
