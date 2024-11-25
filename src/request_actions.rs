use reqwest::blocking;
use url::Url;

use anyhow::Result;

use botifactory_common::project::Project as CommonProject;

pub struct Botifactory {
    url: Url,
}

pub struct Identifier {
    id: Option<i64>,
    name: Option<String>,
}

pub struct Project {
    base: Botifactory,
    name: String,
}

pub struct Channel {
    project: Project,
    identifier: Identifier,
}

pub struct Release {
    channel: Channel,
    identifier: Identifier,
}

pub fn show_project(project: Project) -> Result<CommonProject> {
    let project_url = project.base.url.join(project.name);
    get(project_url)?.json()?
}

pub fn create_project(project: Project) {}
