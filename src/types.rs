use chrono;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub owner: User,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequest {
    pub number: u64,
    pub html_url: String,
    pub state: String,
    pub title: String,
    pub user: User,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub closed_at: Option<String>,

    pub base: Branch,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Branch {
    pub label: String,
    pub sha: String,
    pub user: User,
    pub repo: Repository,
}
