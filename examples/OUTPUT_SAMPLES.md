# Output Samples

These are illustrative examples showing how `databend-admin` output is intended to look for enterprise workflows.
They are meant to make the product feel tangible even before every live connector path is fully finalized.

## RBAC snapshot, Markdown
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

## Security audit, Text
```text
High: broad privilege on warehouse:global_analytics - Role admin holds ALL privileges on warehouse:global_analytics. Confirm this is intentional.
Low: review contractor account contractor_brand_review - Contractor-style accounts should be reviewed for expiry, least privilege, and ongoing need.
```

## Security audit, JSON
```json
[
  {
    "severity": "high",
    "title": "broad privilege on warehouse:global_analytics",
    "detail": "Role admin holds ALL privileges on warehouse:global_analytics. Confirm this is intentional."
  },
  {
    "severity": "low",
    "title": "review contractor account contractor_brand_review",
    "detail": "Contractor-style accounts should be reviewed for expiry, least privilege, and ongoing need."
  }
]
```

## Warehouse health, Markdown
```markdown
# Warehouse Health

| warehouse | size | running | auto suspend secs | auto resume |
| --- | --- | --- | ---: | --- |
| global_analytics | large | true | 300 | true |
| finance_reporting | medium | true | 120 | true |
| adhoc_exploration | small | false | 60 | true |
```

## Enterprise framing example
Imagine a large automotive manufacturer operating across:
- Europe
- North America
- APAC
- multiple brands
- city-level dealer and sales analytics

In a Volvo-, BMW-, or Lamborghini-scale scenario, these outputs help platform and data teams quickly understand:
- who has access
- which roles are too broad
- whether contractor-style accounts still need access
- which warehouses are active and how they are configured
