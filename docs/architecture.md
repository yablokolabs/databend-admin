# Architecture Notes

## Product direction
Start with a Rust admin engine and CLI before introducing any web UI.

## Core problem areas
- RBAC visibility
- security posture checks
- warehouse usage and performance snapshots
- policy drift detection

## Recommended build sequence
1. domain models for users, roles, grants, warehouses, and findings
2. inventory and reporting commands
3. policy/risk checks
4. optional safe mutation layer
5. optional React + TypeScript UI later
