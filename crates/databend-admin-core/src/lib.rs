use databend_driver::{Client, Connection, Value};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VectorDocument {
    pub id: String,
    pub region: String,
    pub country: String,
    pub city: String,
    pub brand: String,
    pub business_unit: String,
    pub document_type: String,
    pub sensitivity: String,
    pub owner_team: String,
    pub warehouse: String,
    pub topic: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VectorMatch {
    pub id: String,
    pub region: String,
    pub country: String,
    pub city: String,
    pub brand: String,
    pub business_unit: String,
    pub document_type: String,
    pub sensitivity: String,
    pub owner_team: String,
    pub warehouse: String,
    pub topic: String,
    pub similarity: f32,
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

pub fn sample_vector_documents() -> Vec<VectorDocument> {
    vec![
        VectorDocument {
            id: "doc-eu-de-berlin-bmw-incentives-q1".to_string(),
            region: "europe".to_string(),
            country: "germany".to_string(),
            city: "berlin".to_string(),
            brand: "bmw".to_string(),
            business_unit: "dealer-operations".to_string(),
            document_type: "sales_policy".to_string(),
            sensitivity: "internal".to_string(),
            owner_team: "europe-sales-ops".to_string(),
            warehouse: "global_analytics".to_string(),
            topic: "Q1 dealer incentive escalation policy for premium sedan campaigns".to_string(),
            embedding: vec![0.93, 0.11, 0.44, 0.21, 0.72, 0.18],
        },
        VectorDocument {
            id: "doc-eu-se-stockholm-volvo-ev-safety-playbook".to_string(),
            region: "europe".to_string(),
            country: "sweden".to_string(),
            city: "stockholm".to_string(),
            brand: "volvo".to_string(),
            business_unit: "aftersales-support".to_string(),
            document_type: "knowledge_base".to_string(),
            sensitivity: "internal".to_string(),
            owner_team: "nordics-service-ops".to_string(),
            warehouse: "dealer_support_ai".to_string(),
            topic: "EV safety messaging and service-advisor escalation guidance".to_string(),
            embedding: vec![0.87, 0.16, 0.38, 0.31, 0.69, 0.27],
        },
        VectorDocument {
            id: "doc-eu-it-milan-lamborghini-launch-playbook".to_string(),
            region: "europe".to_string(),
            country: "italy".to_string(),
            city: "milan".to_string(),
            brand: "lamborghini".to_string(),
            business_unit: "brand-marketing".to_string(),
            document_type: "launch_playbook".to_string(),
            sensitivity: "confidential".to_string(),
            owner_team: "emea-brand-strategy".to_string(),
            warehouse: "executive_brand_intelligence".to_string(),
            topic: "Premium launch coordination playbook for flagship performance campaigns"
                .to_string(),
            embedding: vec![0.81, 0.28, 0.56, 0.47, 0.78, 0.22],
        },
        VectorDocument {
            id: "doc-na-us-newyork-audi-dealer-support".to_string(),
            region: "north-america".to_string(),
            country: "united-states".to_string(),
            city: "new-york".to_string(),
            brand: "audi".to_string(),
            business_unit: "dealer-performance".to_string(),
            document_type: "support_guide".to_string(),
            sensitivity: "internal".to_string(),
            owner_team: "na-retail-performance".to_string(),
            warehouse: "dealer_support_ai".to_string(),
            topic: "Dealer performance support guide for regional sales and service follow-up"
                .to_string(),
            embedding: vec![0.73, 0.20, 0.51, 0.29, 0.63, 0.24],
        },
        VectorDocument {
            id: "doc-apac-jp-tokyo-volvo-warranty-qa".to_string(),
            region: "apac".to_string(),
            country: "japan".to_string(),
            city: "tokyo".to_string(),
            brand: "volvo".to_string(),
            business_unit: "warranty-operations".to_string(),
            document_type: "qa_manual".to_string(),
            sensitivity: "restricted".to_string(),
            owner_team: "apac-quality-and-service".to_string(),
            warehouse: "warranty_ops".to_string(),
            topic: "Warranty claim QA manual for EV component escalation cases".to_string(),
            embedding: vec![0.76, 0.14, 0.35, 0.40, 0.66, 0.30],
        },
    ]
}

pub fn vector_similarity_search(
    query: &[f32],
    docs: &[VectorDocument],
    top_k: usize,
) -> Vec<VectorMatch> {
    let mut matches: Vec<_> = docs
        .iter()
        .map(|doc| VectorMatch {
            id: doc.id.clone(),
            region: doc.region.clone(),
            country: doc.country.clone(),
            city: doc.city.clone(),
            brand: doc.brand.clone(),
            business_unit: doc.business_unit.clone(),
            document_type: doc.document_type.clone(),
            sensitivity: doc.sensitivity.clone(),
            owner_team: doc.owner_team.clone(),
            warehouse: doc.warehouse.clone(),
            topic: doc.topic.clone(),
            similarity: cosine_similarity(query, &doc.embedding),
        })
        .collect();

    matches.sort_by(|a, b| b.similarity.total_cmp(&a.similarity));
    matches.truncate(top_k);
    matches
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

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
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

    #[test]
    fn vector_search_returns_matches() {
        let docs = sample_vector_documents();
        let matches = vector_similarity_search(&[0.92, 0.10, 0.42, 0.20, 0.74, 0.19], &docs, 3);
        assert_eq!(matches.len(), 3);
        assert!(matches[0].similarity >= matches[1].similarity);
    }
}
