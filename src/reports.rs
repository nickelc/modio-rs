//! Reports interface

use url::form_urlencoded;

use crate::prelude::*;

pub struct Reports<C>
where
    C: Clone + Connect + 'static,
{
    modio: Modio<C>,
}

impl<C: Clone + Connect + 'static> Reports<C> {
    pub(crate) fn new(modio: Modio<C>) -> Self {
        Self { modio }
    }

    pub fn submit(&self, report: &Report) -> Future<ModioMessage> {
        self.modio.post("/report", report.to_query_params())
    }
}

pub struct Report {
    pub name: String,
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
    pub fn new<S: Into<String>>(name: S, summary: S, kind: ReportType, resource: Resource) -> Self {
        Self {
            name: name.into(),
            summary: summary.into(),
            kind,
            resource,
        }
    }
}

impl QueryParams for Report {
    fn to_query_params(&self) -> String {
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
            .append_pair("resource", resource)
            .append_pair("id", &id.to_string())
            .append_pair("type", &_type.to_string())
            .append_pair("name", &self.name)
            .append_pair("summary", &self.summary)
            .finish()
    }
}
