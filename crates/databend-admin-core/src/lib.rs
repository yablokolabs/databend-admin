use databend_driver::{Client, Connection, Row, Value};
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAccount {
    pub name: String,
    pub default_role: Option<String>,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoleGrant {
    pub role: String,
    pub object: String,
    pub privilege: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WarehouseHealth {
    pub warehouse: String,
    pub size: String,
    pub running: bool,
    pub auto_suspend_secs: Option<u64>,
    pub auto_resume: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityFinding {
    pub severity: Severity,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
}

#[derive(Debug, Error)]
pub enum AdminError {
    #[error("missing DATABEND_DSN environment variable")]
    MissingDsn,
    #[error("databend query failed: {0}")]
    Databend(String),
}

pub async fn load_users() -> Result<Vec<UserAccount>, AdminError> {
    let conn = connect().await?;
    let rows = conn
        .query_all("SHOW USERS")
        .await
        .map_err(|e| AdminError::Databend(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|row| UserAccount {
            name: value_to_string(row.values().first()),
            default_role: non_empty(value_to_string(row.values().get(2))),
            disabled: value_to_bool(row.values().get(4)),
        })
        .collect())
}

pub async fn load_grants() -> Result<Vec<RoleGrant>, AdminError> {
    let conn = connect().await?;
    let rows = conn
        .query_all("SHOW GRANTS")
        .await
        .map_err(|e| AdminError::Databend(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|row| RoleGrant {
            privilege: value_to_string(row.values().first()),
            object: value_to_string(row.values().get(1)),
            role: value_to_string(row.values().get(3)),
        })
        .collect())
}

pub async fn load_warehouses() -> Result<Vec<WarehouseHealth>, AdminError> {
    let conn = connect().await?;
    let rows = conn
        .query_all("SHOW WAREHOUSES")
        .await
        .map_err(|e| AdminError::Databend(e.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|row| WarehouseHealth {
            warehouse: value_to_string(row.values().first()),
            size: value_to_string(row.values().get(1)),
            running: value_to_bool(row.values().get(2)),
            auto_suspend_secs: value_to_u64(row.values().get(5)),
            auto_resume: value_to_bool(row.values().get(6)),
        })
        .collect())
}

pub fn run_security_audit(users: &[UserAccount], grants: &[RoleGrant]) -> Vec<SecurityFinding> {
    let mut findings = Vec::new();

    for user in users {
        if !user.disabled && user.default_role.is_none() {
            findings.push(SecurityFinding {
                severity: Severity::Medium,
                title: format!("user {} has no default role", user.name),
                detail: "Review whether this account is partially configured or relying on unexpected access paths.".to_string(),
            });
        }

        if !user.disabled && user.name.contains("contractor") {
            findings.push(SecurityFinding {
                severity: Severity::Low,
                title: format!("review contractor account {}", user.name),
                detail: "Contractor-style accounts should be reviewed for expiry, least privilege, and ongoing need.".to_string(),
            });
        }
    }

    for grant in grants {
        if grant.privilege.eq_ignore_ascii_case("ALL") {
            findings.push(SecurityFinding {
                severity: Severity::High,
                title: format!("broad privilege on {}", grant.object),
                detail: format!(
                    "Role {} holds ALL privileges on {}. Confirm this is intentional.",
                    grant.role, grant.object
                ),
            });
        }
    }

    findings
}

async fn connect() -> Result<Connection, AdminError> {
    let dsn = env::var("DATABEND_DSN").map_err(|_| AdminError::MissingDsn)?;
    Client::new(dsn)
        .get_conn()
        .await
        .map_err(|e| AdminError::Databend(e.to_string()))
}

fn value_to_string(value: Option<&Value>) -> String {
    value.map(ToString::to_string).unwrap_or_default()
}

fn non_empty(value: String) -> Option<String> {
    if value.is_empty() || value.eq_ignore_ascii_case("null") {
        None
    } else {
        Some(value)
    }
}

fn value_to_bool(value: Option<&Value>) -> bool {
    let s = value_to_string(value);
    s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("running") || s == "1"
}

fn value_to_u64(value: Option<&Value>) -> Option<u64> {
    value_to_string(value).parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risky_grants_are_flagged() {
        let users = vec![UserAccount {
            name: "contractor_finance".to_string(),
            default_role: Some("analyst".to_string()),
            disabled: false,
        }];
        let grants = vec![RoleGrant {
            role: "admin".to_string(),
            object: "warehouse:default".to_string(),
            privilege: "ALL".to_string(),
        }];
        let findings = run_security_audit(&users, &grants);
        assert!(!findings.is_empty());
    }
}
