# SPECTRE Solana Corpus Scan — Full Findings

_Scanned at: 2026-05-09T05:27:53+00:00_

_Profile: `balanced` · min-confidence: `0.78`_


## Topline

- **Protocols scanned:** 55 of 60
- **Program crates analysed:** 55
- **Architectural findings:** 3,356
- **Errored:** 5

## Findings by rule (top 25)

| Rule ID | Count |
|---|---:|
| `QUAL-003` | 2,938 |
| `ACC-013` | 174 |
| `DEPVULN-001` | 61 |
| `CPI-030` | 50 |
| `GOV-001` | 41 |
| `AUTH-001` | 30 |
| `COV-001` | 30 |
| `AUTH-100` | 20 |
| `CONFIG-010` | 10 |
| `EVT-001` | 2 |

## Per-protocol summary

| Protocol | Category | Status | Findings | Top Rule |
|---|---|---|---:|---|
| jito-restaking | admin | ✓ | 873 | `QUAL-003` ×850 |
| tensor-marketplace | admin | ✓ | 424 | `QUAL-003` ×405 |
| metaplex-bubblegum | admin | ✓ | 413 | `QUAL-003` ×403 |
| mango-v4 | admin | ✓ | 199 | `QUAL-003` ×189 |
| orca-whirlpools | admin | ✓ | 195 | `QUAL-003` ×155 |
| kamino-lending | admin | ✓ | 151 | `QUAL-003` ×145 |
| marginfi-v2 | admin | ✓ | 126 | `QUAL-003` ×119 |
| marinade-anchor | admin | ✓ | 115 | `QUAL-003` ×57 |
| jito-stake-pool | dao | ✓ | 90 | `QUAL-003` ×74 |
| kamino-scope | admin | ✓ | 81 | `QUAL-003` ×80 |
| metaplex-program-library | admin | ✓ | 73 | `QUAL-003` ×58 |
| marinade-validator-bonds | admin | ✓ | 68 | `QUAL-003` ×49 |
| meteora-dlmm | admin | ✓ | 64 | `AUTH-001` ×29 |
| phoenix-v1 | admin | ✓ | 54 | `QUAL-003` ×54 |
| sanctum-unstake | admin | ✓ | 48 | `QUAL-003` ×40 |
| openbook-v2 | admin | ✓ | 47 | `QUAL-003` ×46 |
| serum-dex-v3 | admin | ✓ | 36 | `QUAL-003` ×35 |
| wormhole-core-bridge | timelock | ✓ | 35 | `QUAL-003` ×35 |
| wormhole-token-bridge | timelock | ✓ | 35 | `QUAL-003` ×34 |
| drift-protocol-v2 | insurance | ✓ | 34 | `QUAL-003` ×23 |
| drift-vaults | admin | ✓ | 28 | `QUAL-003` ×27 |
| tribeca-core | dao | ✓ | 26 | `ACC-013` ×26 |
| quarry-mine | admin | ✓ | 25 | `ACC-013` ×16 |
| raydium-clmm | admin | ✓ | 22 | `QUAL-003` ×14 |
| pyth-governance | dao | ✓ | 20 | `QUAL-003` ×13 |
| helium-sub-daos | dao | ✓ | 19 | `QUAL-003` ×12 |
| pyth-solana-receiver | admin | ✓ | 12 | `QUAL-003` ×10 |
| magicblock-bolt | admin | ✓ | 7 | `ACC-013` ×4 |
| tribeca-locked-voter | dao | ✓ | 7 | `ACC-013` ×7 |
| magicblock-bolt-engine | admin | ✓ | 7 | `ACC-013` ×4 |
| raydium-amm | admin | ✓ | 6 | `GOV-001` ×2 |
| squads-v4 | multisig | ✓ | 4 | `ACC-013` ×3 |
| saber-stable-swap | admin | ✓ | 3 | `CPI-030` ×2 |
| pyth-oracle-client | admin | ✓ | 3 | `DEPVULN-001` ×1 |
| goki-smart-wallet | multisig | ✓ | 2 | `AUTH-100` ×1 |
| squads-mpl-v3 | multisig | ✓ | 1 | `QUAL-003` ×1 |
| spl-governance | dao | ✓ | 1 | `QUAL-003` ×1 |
| sanctum-spl-stake-pool | admin | ✓ | 1 | `DEPVULN-001` ×1 |
| helium-data-credits | admin | ✓ | 1 | `CONFIG-010` ×1 |
| spl-stake-pool | admin | ✓ | 0 | - |
| spl-feature-proposal | dao | ✓ | 0 | - |
| spl-single-pool | admin | ✓ | 0 | - |
| drift-insurance-fund-staking | insurance | ✓ | 0 | - |
| jito-tip-distribution | dao | ✓ | 0 | - |
| tensorswap | admin | ✓ | 0 | - |
| metaplex-mpl | admin | ✓ | 0 | - |
| cardinal-token-manager | admin | ✓ | 0 | - |
| solend-program | admin | ✓ | 0 | - |
| port-finance | admin | ✓ | 0 | - |
| jet-margin | dao | ✓ | 0 | - |
| mango-governance | dao | ✓ | 0 | - |
| goki-token-signer | multisig | ✓ | 0 | - |
| marinade-anchor-legacy | admin | ⚠ clone_failed | — | — |
| kamino-vault-program | admin | ⚠ clone_failed | — | — |
| pyth-publisher-caps | dao | ⚠ clone_failed | — | — |
| jet-engine | dao | ⚠ clone_failed | — | — |
| hubbleprotocol-borrowing | admin | ⚠ clone_failed | — | — |
| solend-lending | admin | ✓ | 0 | - |
| staratlas-factory | admin | ✓ | 0 | - |
| drift-jit-proxy | admin | ✓ | 0 | - |

## Errors

- **marinade-anchor-legacy** (clone_failed): repository not found at upstream URL `https://github.com/marinade-finance/marinade-anchor/`.
- **kamino-vault-program** (clone_failed): repository not found at upstream URL `https://github.com/Kamino-Finance/kamino-lending-on-chain-rewards-sdk/`.
- **pyth-publisher-caps** (clone_failed): repository not found at upstream URL `https://github.com/pyth-network/publisher-caps/`.
- **jet-engine** (clone_failed): repository not found at upstream URL `https://github.com/jet-lab/jet-engine/`.
- **hubbleprotocol-borrowing** (clone_failed): repository not found at upstream URL `https://github.com/hubbleprotocol/hubble-public-api/`.
