# databend-admin

Rust-native admin and governance toolkit for Databend environments, focused on RBAC, security posture, and warehouse performance visibility.

## In one sentence
`databend-admin` helps data teams inspect and manage Databend users, roles, grants, security posture, and warehouse/query health from a practical Rust-based control layer.

## Why this exists
Many enterprise data teams end up managing Databend environments through ad hoc SQL, tribal knowledge, and manual review. This project aims to make common administration workflows more consistent, auditable, and automatable.

## Initial scope
- live Databend-backed RBAC inventory for users, roles, and grants
- security posture checks for risky grants and stale or contractor-style accounts
- warehouse health snapshots
- CLI-first workflows with JSON and Markdown reporting

## Workspace
- `databend-admin-core`: admin domain types, policy checks, report generation
- `databend-admin-cli`: CLI entrypoint for inventory and report commands

## Planned command style
```bash
databend-admin rbac snapshot --format markdown
databend-admin security audit --format json
databend-admin warehouse health --format markdown
```

## Current commands
- `databend-admin rbac snapshot`
- `databend-admin security audit`
- `databend-admin warehouse health`

## Enterprise user journey example
A realistic target user is a large European automotive or industrial company onboarding multiple teams into Databend for manufacturing analytics, supply chain reporting, finance, and forecasting.

In that scenario, `databend-admin` gives the platform team a Rust-based admin layer to:
- inventory users, roles, and grants
- run security posture checks
- standardize onboarding reviews
- prepare for future warehouse-performance visibility

See `examples/USER_JOURNEYS.md` for fuller onboarding and enterprise usage examples.

## Rendered output examples

### RBAC snapshot example
```markdown
# RBAC Snapshot

## Users
| name | default role | disabled |
| --- | --- | --- |
| europe_sales_ops | sales_analyst | false |
| finance_exec_reporting | finance_reader | false |
| contractor_brand_review | brand_analyst | false |

## Grants
| role | object | privilege |
| --- | --- | --- |
| sales_analyst | database:vehicle_sales_eu | SELECT |
| finance_reader | database:group_finance | SELECT |
| admin | warehouse:global_analytics | ALL |
```

### Security audit example
```text
High: broad privilege on warehouse:global_analytics - Role admin holds ALL privileges on warehouse:global_analytics. Confirm this is intentional.
Low: review contractor account contractor_brand_review - Contractor-style accounts should be reviewed for expiry, least privilege, and ongoing need.
```

### Warehouse health example
```markdown
# Warehouse Health

| warehouse | size | running | auto suspend secs | auto resume |
| --- | --- | --- | ---: | --- |
| global_analytics | large | true | 300 | true |
| finance_reporting | medium | true | 120 | true |
| adhoc_exploration | small | false | 60 | true |
```

See `examples/OUTPUT_SAMPLES.md` and `examples/USER_JOURNEYS.md` for fuller usage and enterprise framing.

## Principles
- Rust-first and automation-friendly
- reportable before mutable
- safe admin workflows over dashboard theater
- enterprise credibility over hype
