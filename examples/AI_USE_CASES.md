# AI and Vectorized Use Cases

## Summary
Databend environments are not just for classic BI and reporting. They can also support AI-adjacent or vectorized workloads such as:
- semantic search over enterprise documents
- internal retrieval systems
- AI copilots over structured and semi-structured data
- embedding-backed discovery workflows

## Example enterprise scenario
Imagine a large automotive company with:
- Europe, North America, and APAC data regions
- brand-level reporting across Volvo-, BMW-, Audi-, or Lamborghini-scale product organizations
- dealer support documents
- internal manuals
- finance and sales analytics
- AI retrieval or assistant workloads layered on top

In that world, `databend-admin` helps answer:
- who can access AI-related datasets
- whether vectorized or retrieval-oriented workloads are sharing the wrong warehouses
- whether temporary or contractor accounts still have access
- how to keep governance understandable as AI and analytics converge

## Example output framing
### Questions platform teams care about
```text
- Which roles can access embedding or search-support tables?
- Are AI workloads isolated from finance-critical reporting workloads?
- Which warehouses are serving mixed analytics + retrieval traffic?
- Are regional brand teams properly scoped by country or brand boundary?
```

### Why it is useful
This turns `databend-admin` into more than a traditional admin CLI.
It becomes a governance and operational visibility layer for modern data environments where structured analytics and AI-oriented workloads coexist.
