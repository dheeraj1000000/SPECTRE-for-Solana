# SPECTRE for Solana

**Findings reports** and bench fixtures from **SPECTRE for Solana**.

## What's in this repo

```
.
├── reports/
│   └── spectre-corpus-scan-2026-05-09.md   # 55-protocol corpus scan, full findings
├── fixtures/
│   └── token2022/                          # Single-file Rust bench fixtures
│       ├── fee-transfer-tok023-{vuln,safe}.rs
│       ├── nonce-stale-nonce002-{vuln,safe}.rs
│       └── nonce-stale-multigen-nonce002-vuln.rs
├── LICENSE
└── README.md
```

## Latest corpus scan (2026-05-09)

SPECTRE for Solana findings reported across **55 production Solana protocols**.

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

**Top-10 protocols by raw finding count** (full table in
[`reports/spectre-corpus-scan-2026-05-09.md`](./reports/spectre-corpus-scan-2026-05-09.md)):

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

> SPECTRE for Solana findings reported here are produced by automated
> pattern detection over the abstract syntax tree of each program's
> published source code. They are **not the output of an authorized manual
> audit**, and no engagement exists with any of the named protocols.
> Whether each pattern represents a real-world risk depends on operational
> considerations the tool cannot observe (admin-key custody, multisig
> thresholds, timelock delays, bug-bounty coverage, incident-response
> runbooks).

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

Brought to you by RHINO, GHOST PEPPA, and BRAVEHEART @ Team Pinpoint.

## License

MIT — see [LICENSE](./LICENSE).
