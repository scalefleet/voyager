#[derive(serde::Deserialize)]
pub struct OrganizationListResponse {
    #[serde(rename = "type")]
    _type: String,
    current_page: usize,
    next_page: Option<usize>,
    next_page_url: Option<String>,
    prev_page: Option<usize>,
    prev_page_url: Option<String>,
    data: Vec<Organization>,
}

#[derive(serde::Deserialize)]
pub struct Organization {
    #[serde(rename = "type")]
    _type: String,
    id: String,
    name: String,
    created_at: String,
    updated_at: String,
    billing_email: String,
    sso: bool,
    sso_directory: bool,
    can_create_databases: bool,
    free_databases_remaining: usize,
    single_tenancy: bool,
    plan: String,
    valid_billing_info: bool,
    admin_only_production_access: bool,
    has_past_due_invoices: bool,
    flags: OrganizationFlag,
    features: OrganizationFeatures,
}

#[derive(serde::Deserialize)]
pub struct OrganizationFlag {
    connect_limited_beta_open: String,
    new_org_integrations_ui: String,
    branch_read_only_region: String,
}

#[derive(serde::Deserialize)]
pub struct OrganizationFeatures {
    insights: bool,
    sso: bool,
    single_tenancy: bool,
}
