# SPECTRE for Solana — Hackathon Progress Report

_Reporting window: September 14–October 12, 2026_  
_Benchmark snapshot: September 18, 2026_  
_Publication scope: public results and methodology only_

## Summary

During the hackathon reporting window, Team Pinpoint shipped SPECTRE CLI
1.6.9 and completed a benchmark-quality pass over the Solana detector suite.
The work focused on making results measurable and honest: regenerating the
corpus, correcting path and corpus-selection errors, recording every rule in
the scorecard, and moving a noisy rule out of the production-facing strict
set until it can meet the quality bar.

This report is synchronized from committed engineering milestones in the
private product repository. It contains no proprietary implementation source,
customer code, credentials, internal infrastructure, local filesystem paths,
private issue links, or personal contribution data.

## Dated milestones

| Date | Public milestone |
|---|---|
| 2026-09-14 | Released SPECTRE CLI 1.6.9 and completed a Windows build compatibility correction. |
| 2026-09-17 | Regenerated the Solana benchmark and added recorded build metadata plus a generated scorecard. |
| 2026-09-17 | Corrected source-path resolution across the Solana rule pack, including the handling of scan-root-relative files. |
| 2026-09-17 | Re-evaluated the strict profile and moved `ACC-013` to the experimental profile after the corpus showed unacceptable false-positive volume. |
| 2026-09-18 | Corrected corpus subpaths for six public benchmark entries and regenerated the 72-entry scorecard. |
| 2026-09-18 | Updated benchmark and CI accounting so a measurement that did not run is not reported as a failed measurement. |
| 2026-09-23 | Completed integration of the hackathon-window benchmark corrections into the main development line. |
| 2026-09-24–2026-10-12 | No additional committed Solana benchmark milestone was recorded for this reporting window; this report does not invent activity for those dates. |

## Benchmark snapshot

The September 18 snapshot used SPECTRE CLI 1.6.9 against **72 corpus
entries**: **60 Anchor** and **12 native Solana** programs or fixtures.
Ground truth is expressed per rule as a source file and expected line range.
An emitted finding counts as a true positive only when the rule and location
match that ground truth within the benchmark tolerance.

### Production-facing strict set

The strict set contained 11 rules. The figures below are derived from the
per-rule scorecard after the September 18 corpus-subpath correction.

| Metric | Result |
|---|---:|
| True positives | 53 |
| False positives | 68 |
| False negatives | 14 |
| Precision | 0.438 |
| Recall | 0.791 |
| F1 | 0.564 |

The benchmark exposed meaningful remaining work. In particular, two strict
rules had no true-positive signal in this corpus snapshot and require either
better positive coverage, detector tuning, or removal from the strict set.
The reported numbers are therefore an engineering baseline, not a claim that
the strict profile has reached its target quality.

### Full research profile

The `all` profile includes experimental and surface-coverage rules that are
not part of the production-facing strict set.

| Metric | Result |
|---|---:|
| True positives | 67 |
| False positives | 2,939 |
| False negatives | 55 |
| Precision | 0.022 |
| Recall | 0.549 |
| F1 | 0.043 |

Publishing this baseline makes the tuning backlog visible. The high
false-positive count is dominated by experimental rules and must not be read
as the quality of the strict profile.

## What changed in the measurement process

- The benchmark began writing build metadata so results could be tied to the
  executable that produced them.
- The rule inventory was brought into the generated scorecard so unmeasured
  rules could not disappear silently.
- Source paths were resolved relative to the scan root instead of the process
  working directory.
- Six corpus manifests were corrected so each scan covered the intended
  program subtree rather than an entire upstream repository.
- A rule with zero measured true positives and excessive false positives was
  removed from the strict profile and kept available for experimental tuning.

## Limitations

- The snapshot is a static-analysis benchmark, not a manual audit or a claim
  that every emitted pattern is exploitable.
- Corpus coverage is uneven across rule classes. A zero does not prove that a
  detector or vulnerability class is absent from the wider Solana ecosystem.
- The internal run metadata recorded an uncommitted working-tree state. The
  figures are useful hackathon evidence, but they are not presented as a
  release certification or independently reproduced result.
- Protocol and fixture names in the underlying benchmark refer to public
  source material. Their inclusion does not imply an audit engagement or an
  endorsement by the named project.

## Public/private boundary

This public repository contains results, methodology summaries, selected
fixtures, and sample deliverables. SPECTRE's scanner implementation, private
test harness, operational infrastructure, customer material, and internal
development history remain proprietary and are intentionally not mirrored
here.
