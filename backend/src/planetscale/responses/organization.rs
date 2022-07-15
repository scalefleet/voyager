#[derive(Debug, serde::Deserialize)]
pub struct OrganizationListResponse {
    #[serde(rename = "type")]
    _type: String,
    pub current_page: usize,
    pub next_page: Option<usize>,
    pub next_page_url: Option<String>,
    pub prev_page: Option<usize>,
    pub prev_page_url: Option<String>,
    pub data: Vec<Organization>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Organization {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub billing_email: String,
    pub sso: bool,
    pub sso_directory: bool,
    pub can_create_databases: bool,
    pub free_databases_remaining: usize,
    pub single_tenancy: bool,
    pub plan: String,
    pub valid_billing_info: bool,
    pub admin_only_production_access: bool,
    pub has_past_due_invoices: bool,
    pub flags: OrganizationFlag,
    pub features: OrganizationFeatures,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrganizationFlag {
    pub connect_limited_beta_open: String,
    pub new_org_integrations_ui: String,
    pub branch_read_only_region: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct OrganizationFeatures {
    pub insights: bool,
    pub sso: bool,
    pub single_tenancy: bool,
}
