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

## Example command
```bash
databend-admin ai vector-demo --query 0.90,0.10,0.40,0.20 --top 3 --format markdown
```

## Example output
```markdown
# AI Vector Demo

| id | brand | city | similarity | topic |
| --- | --- | --- | ---: | --- |
| doc-eu-bmw-berlin-001 | bmw | berlin | 1.000 | dealer incentive policy |
| doc-eu-volvo-stockholm-001 | volvo | stockholm | 0.986 | ev safety messaging |
| doc-na-audi-newyork-001 | audi | new-york | 0.965 | dealer performance support |
```

## Why it is useful
This turns `databend-admin` into more than a traditional admin CLI.
It becomes a governance and operational visibility layer for modern data environments where structured analytics and AI-oriented workloads coexist.
