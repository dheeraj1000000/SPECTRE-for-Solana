# SPECTRE for Solana

**SPECTRE** is an end-to-end security review pipeline for Solana
programs, built by Team Pinpoint. It pairs a fast Rust scanner with a
chain of specialist agents that investigate findings, prove them with
tests, and deliver a polished audit pack — all surfaced through a
dashboard.

This repository hosts **public artifacts** from SPECTRE for Solana:
sample audit packs, a corpus-scan findings report, and bench fixtures
for our Token-2022 detectors. The pipeline itself is proprietary.

## Repository map

```
.
├── audits/                                     # Sample audit packs (full zips)
│   ├── 260511-SPECTRE-AUDIT-JITO-RESTAKING-V1.zip
│   ├── 260511-SPECTRE-AUDIT-METAPLEX-BUBBLEGUM-V1.zip
│   └── 260511-SPECTRE-AUDIT-TENSOR-MARKETPLACE-V1.zip
├── reports/
│   └── spectre-corpus-scan-2026-05-09.md       # 55-protocol corpus scan, full findings
├── fixtures/
│   └── token2022/                              # Single-file Rust bench fixtures
│       ├── fee-transfer-tok023-{vuln,safe}.rs
│       ├── nonce-stale-nonce002-{vuln,safe}.rs
│       └── nonce-stale-multigen-nonce002-vuln.rs
├── LICENSE
└── README.md
```

**For reviewers in a hurry:**

- Want a deliverable example? → [`audits/`](./audits/)
- Want the breadth-of-coverage evidence? → [corpus scan](#corpus-scan-2026-05-09)
- Want to see detectors fire against known-vuln/known-safe code?
  → [`fixtures/token2022/`](./fixtures/token2022/)

## How SPECTRE works

SPECTRE turns a Solana codebase into an auditable deliverable in four
stages:

### 1. Fast scan — Rust CLI

A native Rust command-line tool ingests a target repository and produces
a structured set of candidate findings in seconds to minutes, depending
on corpus size. The CLI is built for throughput: a full 55-protocol
corpus scan fits inside a single working session.

### 2. Triage — dashboard

Scan results land in the SPECTRE dashboard, where customers see findings
grouped by program, rule, and severity. The dashboard is where all
subsequent activity is tracked: agent progress, intermediate artifacts,
and the final deliverable.

### 3. Agent chain — investigate, test, patch

Each promoted finding is handed to a series of specialist agents that
work in sequence:

1. **Investigator** — reviews the candidate in context and decides
   whether it represents a real risk worth reporting.
2. **Test writer** — produces a failing test that demonstrates the issue
   on the live codebase. A finding without a reproducer does not
   advance.
3. **Patch writer** — proposes a minimal fix and shows the test passing
   against the patched program.

The result is a finding backed by a concrete reproducer and a concrete
remediation, not just a description.

### 4. Audit pack — delivered to the customer

When the chain completes, SPECTRE generates a typeset PDF audit pack
covering every confirmed finding, its reproducer, its proposed patch,
and surrounding context. The customer receives an email notifying them
the audit is ready, with a link back to the dashboard to download the
pack and review individual findings.

## Sample audit packs

Three full audit packs from the 2026-05-11 public-corpus run live in
[`audits/`](./audits/) as zip archives. Each zip mirrors exactly what an
engaged customer receives at the end of the pipeline — the typeset PDF
alongside its machine-readable companions (`.md`, `.csv`, `.yml`),
auxiliary materials, and a per-pack README.

| Protocol | Pack |
|---|---|
| Jito — restaking | [`260511-SPECTRE-AUDIT-JITO-RESTAKING-V1.zip`](./audits/260511-SPECTRE-AUDIT-JITO-RESTAKING-V1.zip) |
| Metaplex — Bubblegum | [`260511-SPECTRE-AUDIT-METAPLEX-BUBBLEGUM-V1.zip`](./audits/260511-SPECTRE-AUDIT-METAPLEX-BUBBLEGUM-V1.zip) |
| Tensor — marketplace | [`260511-SPECTRE-AUDIT-TENSOR-MARKETPLACE-V1.zip`](./audits/260511-SPECTRE-AUDIT-TENSOR-MARKETPLACE-V1.zip) |

These are public-corpus deliverables produced without an engagement with
the named protocols — see the note under the corpus-scan section below.

## Corpus scan (2026-05-09)

SPECTRE for Solana findings reported across **55 production Solana
protocols**. Full table:
[`reports/spectre-corpus-scan-2026-05-09.md`](./reports/spectre-corpus-scan-2026-05-09.md).

| Metric | Value |
|---|---:|
| Protocols scanned | **55 of 60** |
| Architectural findings | **3,356** |
| Distinct rules fired | **10** |
| Errored (clone_failed) | 5 |

**Top rules by fire count**

| Rule ID | Count | Description |
|---|---:|---|
| `QUAL-003` | 2,938 | Code-quality smell (high-volume hygiene rule) |
| `ACC-013` | 174 | Account-type / discriminator validation gap |
| `DEPVULN-001` | 61 | Vulnerable dependency declared in `Cargo.toml` |
| `CPI-030` | 50 | CPI to untrusted program without provenance check |
| `GOV-001` | 41 | Privileged governance instruction without timelock |
| `AUTH-001` | 30 | Missing signer / authority constraint |
| `COV-001` | 30 | Test coverage gap on public instruction |
| `AUTH-100` | 20 | Single-step authority transfer (PDA seed binding) |
| `CONFIG-010` | 10 | Mutable config accepted on permissionless path |
| `EVT-001` | 2 | Privileged handler emits via `emit_cpi!` only |

**Top-10 protocols by raw finding count**

| Protocol | Findings | Top rule |
|---|---:|---|
| jito-restaking | 873 | `QUAL-003` ×850 |
| tensor-marketplace | 424 | `QUAL-003` ×405 |
| metaplex-bubblegum | 413 | `QUAL-003` ×403 |
| mango-v4 | 199 | `QUAL-003` ×189 |
| orca-whirlpools | 195 | `QUAL-003` ×155 |
| kamino-lending | 151 | `QUAL-003` ×145 |
| marginfi-v2 | 126 | `QUAL-003` ×119 |
| marinade-anchor | 115 | `QUAL-003` ×57 |
| jito-stake-pool | 90 | `QUAL-003` ×74 |
| kamino-scope | 81 | `QUAL-003` ×80 |

> SPECTRE for Solana findings reported here are produced by the
> automated fast-scan stage over each program's published source code.
> They are **not the output of an authorized manual audit**, and no
> engagement exists with any of the named protocols. Whether each
> pattern represents a real-world risk depends on operational
> considerations the tool cannot observe (admin-key custody, multisig
> thresholds, timelock delays, bug-bounty coverage, incident-response
> runbooks). The full agent chain — investigation, reproducer, patch,
> audit pack — runs only for engaged customers.

## Bench fixtures (Token-2022)

Single-file Rust samples illustrating three Token-2022 detector classes.
Each rule has a `vuln` fixture (rule should fire) and a `safe` fixture
(rule should be silenced); the multi-generation NONCE-002 fixture
exercises the per-(program, field, handler) dedup guard.

| Rule ID | Class | Scenario |
|---|---|---|
| **TOK-023** | `transfer_checked` CPI on a `TransferFeeConfig` mint without fee-bps adjustment of the credited amount | `fee-transfer` |
| **NONCE-002** | User-defined nonce counter written without a strict monotonic floor (`require!(new >= old)`) | `nonce-stale` (single + multi-generation) |

The fixtures are designed to be parsed (not necessarily compiled). Each
file's header comment documents the expected detection trace. They are
suitable for reproducing detector behaviour in a SPECTRE rule-pack test
harness or for benchmarking other Solana / Anchor static-analysis tools.

---

Brought to you by RHINO, GHOST PEPPA, and BRAVEHEART @ Team Pinpoint.

## License

MIT — see [LICENSE](./LICENSE).
