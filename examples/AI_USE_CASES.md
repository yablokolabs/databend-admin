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

## More realistic sample dataset shape
The sample vector records now model realistic enterprise metadata such as:
- region, country, city
- brand
- business unit
- document type
- sensitivity level
- owner team
- warehouse serving the workload
- topic text and vector embedding

## Example command
```bash
databend-admin ai vector-demo --query 0.92,0.10,0.42,0.20,0.74,0.19 --top 3 --format markdown
```

## Example Markdown output
```markdown
# AI Vector Demo

| id | region | country | brand | city | business unit | sensitivity | warehouse | similarity | topic |
| --- | --- | --- | --- | --- | --- | --- | --- | ---: | --- |
| doc-eu-de-berlin-bmw-incentives-q1 | europe | germany | bmw | berlin | dealer-operations | internal | global_analytics | 1.000 | Q1 dealer incentive escalation policy for premium sedan campaigns |
| doc-eu-se-stockholm-volvo-ev-safety-playbook | europe | sweden | volvo | stockholm | aftersales-support | internal | dealer_support_ai | 0.991 | EV safety messaging and service-advisor escalation guidance |
| doc-na-us-newyork-audi-dealer-support | north-america | united-states | audi | new-york | dealer-performance | internal | dealer_support_ai | 0.979 | Dealer performance support guide for regional sales and service follow-up |
```

## Example JSON output
```json
[
  {
    "id": "doc-eu-de-berlin-bmw-incentives-q1",
    "region": "europe",
    "country": "germany",
    "city": "berlin",
    "brand": "bmw",
    "business_unit": "dealer-operations",
    "document_type": "sales_policy",
    "sensitivity": "internal",
    "owner_team": "europe-sales-ops",
    "warehouse": "global_analytics",
    "topic": "Q1 dealer incentive escalation policy for premium sedan campaigns",
    "similarity": 0.99964243
  },
  {
    "id": "doc-eu-se-stockholm-volvo-ev-safety-playbook",
    "region": "europe",
    "country": "sweden",
    "city": "stockholm",
    "brand": "volvo",
    "business_unit": "aftersales-support",
    "document_type": "knowledge_base",
    "sensitivity": "internal",
    "owner_team": "nordics-service-ops",
    "warehouse": "dealer_support_ai",
    "topic": "EV safety messaging and service-advisor escalation guidance",
    "similarity": 0.99145013
  }
]
```

## Why it is useful
This turns `databend-admin` into more than a traditional admin CLI.
It becomes a governance and operational visibility layer for modern data environments where structured analytics and AI-oriented workloads coexist.
