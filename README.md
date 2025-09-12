# Obliquery

**Working name:** `Obliquery`

This document is a step‑by‑step blueprint to implement a Rust framework that executes *public queries over private inputs from multiple data owners* using pluggable MPC protocols. It covers design goals, threat model, architecture, IRs, optimization, operator circuits, protocol abstraction, deployment, and a TPC‑H benchmarking plan across 2–3 (and N) computing nodes.

---

## 0) Roadmap

1. **Repo & Crates**  → traits, types, message bus, configs, proto def... 
2. **IRs**: Logical Algebra DAG (QIR) → Optimized Plan → Circuit IR (CIR). 
3. **Protocol Abstraction Layer (PAL)** with **3PC (replicated sharing)** shims. Other protocols will be extended later. 
4. **Operator library**: `scan`, `project`, `filter`, `join`, `aggregate`, `group by`, `sort/limit` as circuits.
5. **Runtime & scheduler**: topological execution, batched ops, query optimization ([Alchemy](https://www.vldb.org/pvldb/vol18/p3021-sohn.pdf)).
6. **TPC‑H ingestion** + data owner adapters, e2e for TPC-H queries.
7. **Distributed deploy**: docker‑compose (2PC/3PC). 
8. **Benchmark harness** + metrics + plots.
