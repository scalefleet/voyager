use crate::planetscale::responses;

pub type DatabaseList = responses::List<Database>;

#[derive(Debug, serde::Deserialize)]
pub struct Database {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub actor: DatabaseActor,
    pub url: String,
    pub branches_url: String,
    pub branches_count: usize,
    pub production_branches_count: usize,
    pub development_branches_count: usize,
    pub deploy_requests_count: usize,
    pub insights_raw_queries: bool,
    pub multiple_admins_required_for_deletion: bool,
    pub ready: bool,
    pub sharded: bool,
    pub at_development_branch_limit: bool,
    pub default_branch_shard_count: usize,
    pub default_branch_read_only_regions_count: usize,
    pub issues_count: usize,
    pub region: DatabaseRegion,
    pub default_branch: String,
    pub data_import: Option<usize>,
    pub state: String,
    pub name: String,
    pub require_approval_for_deploy: bool,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub migration_table_name: Option<String>,
    pub migration_framework: Option<String>,
    pub automatic_migrations: Option<String>,
    pub plan: String,
    pub production_branch_web_console: bool,
    pub restrict_branch_region: bool,
    pub allow_data_branching: bool,
    pub html_url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseActor {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub display_name: String,
    pub avatar_url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DatabaseRegion {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub provider: String,
    pub enabled: bool,
    pub display_name: String,
    pub location: String,
    pub slug: String,
}
