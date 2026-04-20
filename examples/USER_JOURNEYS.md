# User Journeys

These examples are intentionally written as realistic enterprise scenarios.
They are illustrative only and do not imply any relationship with BMW, Audi, or any other company.

## 1. Onboarding a large European automotive manufacturer
Imagine a large automotive manufacturer with multiple business units:
- manufacturing analytics
- supply chain operations
- finance reporting
- dealer performance analytics
- battery and EV forecasting teams

They want to standardize Databend administration across environments without relying on ad hoc SQL and tribal knowledge.

### How `databend-admin` helps
The platform team can use `databend-admin` to:
- inventory current users, roles, and grants
- review broad or risky access before go-live
- generate repeatable RBAC snapshots for security review
- document warehouse-level access for each department
- create a baseline for future policy drift checks

### Example flow
#### Step 1: take an RBAC inventory snapshot
```bash
databend-admin rbac snapshot --format markdown
```

This gives the platform/security team a readable report of:
- users
- default roles
- warehouse and database grants

#### Step 2: run a security audit
```bash
databend-admin security audit --format json
```

This produces findings that can be reviewed by:
- platform engineering
- security reviewers
- data governance stakeholders

#### Step 3: hand off the report internally
The organization can circulate the Markdown or JSON output during onboarding reviews to validate:
- least-privilege posture
- separation between finance, supply chain, and analytics teams
- readiness for production access

## 2. Audit readiness before a quarterly review
A large enterprise wants to prepare for an internal audit or a security review.

### Goal
Answer questions like:
- who has broad access?
- which roles are over-privileged?
- which warehouses are exposed to too many users?
- where do access patterns need review?

### Example commands
```bash
databend-admin rbac snapshot --format markdown
databend-admin security audit --format markdown
```

### Outcome
Instead of manually assembling SQL extracts, the team gets repeatable operational outputs they can attach to review workflows.

## 3. Warehouse operations visibility for a multi-team environment
An enterprise data platform team wants a future workflow where warehouse health is visible across multiple analytics groups.

### Target use case
- manufacturing dashboards start slowing down
- supply chain forecasting queries spike
- finance month-end loads hit the same shared warehouse

The long-term goal for `databend-admin` is to expose warehouse-level operational visibility so teams can quickly identify:
- heavy workloads
- slow or expensive query patterns
- role-to-warehouse usage overlap
- operational bottlenecks

## 4. Why this matters for enterprise buyers
For a large automotive or industrial company, the pain is usually not “how do we run one SQL query.”
It is:
- how do we govern access consistently
- how do we onboard teams safely
- how do we review security posture quickly
- how do we make warehouse usage operationally visible

That is the problem `databend-admin` is trying to solve.
