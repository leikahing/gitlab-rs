extern crate serde_json;

use self::super::{Gitlab, Error, ErrorKind, Result};

use url::Url;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use url::form_urlencoded;

use std::vec::Vec;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Internal
}

/// A client for the [Projects API](https://docs.gitlab.com/ee/api/projects.html)
pub struct Projects<'a> {
    gitlab: &'a Gitlab,
}

pub struct UserProjects<'a> {
    gitlab: &'a Gitlab,
    user: String,
}

#[derive(Debug, Deserialize)]
pub struct Permission {
    pub access_level: u8,
    pub notification_level: u8,
}

#[derive(Debug, Deserialize)]
pub struct PermissionsWrapper {
    pub project_access: Option<Permission>,
    pub group_access: Option<Permission>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    //pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Namespace {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub full_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Statistic {
    pub commit_count: u32,
    pub storage_size: u64,
    pub repository_size: u64,
    pub lfs_objects_size: u64,
    pub job_artifacts_size: u64,
}

#[derive(Debug, Deserialize)]
pub struct ProjectLinks {
    #[serde(rename="self")]
    pub self_link: String,
    pub issues: String,
    pub merge_requests: String,
    pub repo_branches: String,
    pub labels: String,
    pub events: String,
    pub members: String,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub group_id: u64,
    pub group_name: String,
    pub group_access_level: u16,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: u64,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub visibility: String,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub web_url: String,
    pub tag_list: Vec<String>,
    pub owner: Option<User>,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
    pub issues_enabled: bool,
    pub open_issues_count: u32,
    pub merge_requests_enabled: bool,
    pub jobs_enabled: Option<bool>,
    pub wiki_enabled: Option<bool>,
    pub snippets_enabled: bool,
    pub container_registry_enabled: Option<bool>,
    pub created_at: String,
    pub last_activity_at: String,
    pub creator_id: u32,
    pub namespace: Namespace,
    pub import_status: String,
    pub import_error: Option<String>,
    pub permissions: Option<PermissionsWrapper>,
    pub archived: bool,
    pub avatar_url: Option<String>,
    pub shared_runners_enabled: bool,
    pub forks_count: u32,
    pub star_count: u32,
    pub ci_config_path: Option<String>,
    #[serde(default)]
    pub runners_token: Option<String>,
    pub public_jobs: bool,
    pub shared_with_groups: Vec<Group>,
    pub repository_storage: Option<String>,
    pub only_allow_merge_if_pipeline_succeeds: bool,
    pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    pub printing_merge_requests_link_enabled: Option<bool>,
    pub request_access_enabled: bool,
    pub approvals_before_merge: u8,
    pub statistics: Option<Statistic>,
    #[serde(rename="_links")]
    pub links: Option<ProjectLinks>,
}

#[derive(Default)]
pub struct SingleProjectOptions {
    pub id: String,
    pub params: HashMap<&'static str, String>,
}

impl SingleProjectOptions {
    pub fn builder<T: Into<String>>(id: T) -> SingleProjectOptionsBuilder {
        SingleProjectOptionsBuilder::new(id)
    }

    pub fn to_query_string(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded_qs: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded_qs)
        }
    }
}

pub struct SingleProjectOptionsBuilder(SingleProjectOptions);

impl SingleProjectOptionsBuilder {
    pub fn new<T>(id: T) -> Self
        where T: Into<String>,
    {
        SingleProjectOptionsBuilder(SingleProjectOptions {
            id: id.into(),
            ..Default::default()
        })
    }

    pub fn statistics(&mut self, statistics: bool) -> &mut Self {
        self.0.params.insert("statistics", statistics.to_string());
        self
    }

    pub fn build(&self) -> SingleProjectOptions {
        SingleProjectOptions {
            id: self.0.id.clone(),
            params: self.0.params.clone(),
        }
    }
}

#[derive(Default)]
pub struct GetProjectUsersOptions {
    pub id: String,
    pub params: HashMap<&'static str, String>,
}

impl GetProjectUsersOptions {
    pub fn builder<T: Into<String>>(id: T) -> GetProjectUsersOptionsBuilder {
        GetProjectUsersOptionsBuilder::new(id)
    }

    pub fn to_query_string(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let encoded_qs: String = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(encoded_qs)
        }
    }
}

pub struct GetProjectUsersOptionsBuilder(GetProjectUsersOptions);

impl GetProjectUsersOptionsBuilder {
    pub fn new<T>(id: T) -> Self
        where T: Into<String>,
    {
        GetProjectUsersOptionsBuilder(GetProjectUsersOptions {
            id: id.into(),
            ..Default::default()
        })
    }

    pub fn search_for_user<T>(&mut self, user: T) -> &mut Self
        where T: Into<String>
    {
        self.0.params.insert("search", user.into());
        self
    }

    pub fn build(&self) -> GetProjectUsersOptions {
        GetProjectUsersOptions {
            id: self.0.id.clone(),
            params: self.0.params.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ProjectParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_enabled : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippets_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_registry_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_runners_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_jobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfs_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_access_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printing_merge_requests_link_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_config_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvals_before_merge: Option<u8>,
}

impl ProjectParams {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        ProjectParams {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn builder<T: Into<String>>(name: T) -> ProjectParamsBuilder
    {
        ProjectParamsBuilder::new(name)
    }
}

pub struct ProjectParamsBuilder(ProjectParams);

impl ProjectParamsBuilder {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        ProjectParamsBuilder(ProjectParams {
            name: name.into(),
            ..Default::default()
        })
    }

    pub fn path<T>(&mut self, path: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.0.path = Some(path.into());
        self
    }

    pub fn namespace_id(&mut self, id: u64) -> &mut Self {
        self.0.namespace_id = Some(id);
        self
    }

    pub fn default_branch<T>(&mut self, branch: T) -> &mut Self
    where
        T: Into<String>
    {
        self.0.default_branch = Some(branch.into());
        self
    }

    pub fn build(&self) -> ProjectParams {
        ProjectParams {
            name: self.0.name.clone(),
            ..Default::default()
        }
    }
}

impl<'a> Projects<'a> {
    #[doc(hidden)]
    pub fn new(gitlab: &'a Gitlab) -> Projects<'a> {
        Projects {
            gitlab: gitlab,
        }
    }

    fn resource(&self, id: &str, more: &str) -> String {
        format!("/projects/{}{}", id, more)
    }

    pub fn project(&self, options: &SingleProjectOptions) -> Result<Project> {
        let encoded_id = utf8_percent_encode(&options.id, PATH_SEGMENT_ENCODE_SET).to_string();
        let mut uri = vec![self.resource(&encoded_id, "")];
        if let Some(query) = options.to_query_string() {
            uri.push(query);
        }
        self.gitlab.get::<Project>(&uri.join("?"))
    }

    pub fn users(&self, options: &GetProjectUsersOptions) -> Result<Vec<User>> {
        let encoded_id = utf8_percent_encode(&options.id, PATH_SEGMENT_ENCODE_SET).to_string();
        let mut uri = vec![self.resource(&encoded_id, "/users")];
        if let Some(query) = options.to_query_string() {
            uri.push(query)
        }
        self.gitlab.get::<Vec<User>>(&uri.join("?"))
    }

    pub fn create(&self, params: &ProjectParams) -> Result<Project> {
        let json = serde_json::to_string(&params)?;
        self.gitlab.post::<Project>(&self.resource("", ""), json.into_bytes())
    }

    pub fn delete<T>(&self, id: T)
    where
        T: Into<String>
    {
        let encoded_id = utf8_percent_encode(&id.into(), PATH_SEGMENT_ENCODE_SET).to_string();
        self.gitlab.delete(&self.resource(&encoded_id, ""));
    }
}