use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub number: u32,
    pub title: String,
    pub head_ref_name: String,
    pub state: String,
    pub author: Author,
    pub updated_at: String,
    #[serde(default)]
    pub status_check_rollup: Vec<CheckStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Author {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckStatus {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub gh: bool,
    pub claude: bool,
}

pub fn check_gh_auth() -> bool {
    Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn check_claude_auth() -> bool {
    Command::new("claude")
        .args(["--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn fetch_prs(repo: &str) -> Result<Vec<PullRequest>, String> {
    let output = Command::new("gh")
        .args([
            "pr",
            "list",
            "--repo",
            repo,
            "--json",
            "number,title,headRefName,state,statusCheckRollup,author,updatedAt",
            "--limit",
            "50",
        ])
        .output()
        .map_err(|e| format!("Failed to run gh: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gh pr list failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse PR list: {}", e))
}

pub fn fetch_pr_detail(repo: &str, number: u32) -> Result<PullRequest, String> {
    let output = Command::new("gh")
        .args([
            "pr",
            "view",
            &number.to_string(),
            "--repo",
            repo,
            "--json",
            "number,title,headRefName,state,statusCheckRollup,author,updatedAt",
        ])
        .output()
        .map_err(|e| format!("Failed to run gh: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gh pr view failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse PR detail: {}", e))
}
