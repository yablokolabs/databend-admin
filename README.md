# databend-admin

Rust-native admin and governance toolkit for Databend environments, focused on RBAC, security posture, and warehouse performance visibility.

## In one sentence
`databend-admin` helps data teams inspect and manage Databend users, roles, grants, security posture, and warehouse/query health from a practical Rust-based control layer.

## Why this exists
Many enterprise data teams end up managing Databend environments through ad hoc SQL, tribal knowledge, and manual review. This project aims to make common administration workflows more consistent, auditable, and automatable.

## Initial scope
- RBAC inventory for users, roles, and grants
- security posture checks for risky or stale access
- warehouse and query health snapshots
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

## Enterprise user journey example
A realistic target user is a large European automotive or industrial company onboarding multiple teams into Databend for manufacturing analytics, supply chain reporting, finance, and forecasting.

In that scenario, `databend-admin` gives the platform team a Rust-based admin layer to:
- inventory users, roles, and grants
- run security posture checks
- standardize onboarding reviews
- prepare for future warehouse-performance visibility

See `examples/USER_JOURNEYS.md` for fuller onboarding and enterprise usage examples.

## Principles
- Rust-first and automation-friendly
- reportable before mutable
- safe admin workflows over dashboard theater
- enterprise credibility over hype
