# OPEN_SOURCE_READINESS_AUDIT

> Release-readiness audit of Telemetry Hub, performed by the Release Engineer
> before the first public GitHub release. Scope: repository governance,
> automation, developer experience, release engineering, and open-source
> maintainability. Out of scope: feature implementation, application code,
> business logic.

## 1. Executive Summary

Telemetry Hub's **documentation foundation is strong**; its **repository
infrastructure is not**. The audit found zero CI, zero automation, zero release
engineering artifacts, and all 14 standard Rust tooling files absent. The
remote repository confirms the gap: no workflows, no releases, no milestones,
no branch protection, Dependabot security updates disabled, and default-only
labels.

The codebase itself is healthy at the architectural level (clean layering,
repository traits, one domain crate), but hygiene is unreleased: 23 compiler
warnings, a `todo!()`, panic-based DB mapping, an unused dependency, a public
typo (`Postres`), and four crates shipping `cargo new` boilerplate.

**Verdict: not ready for public release.** The documentation declares a gate
(CONTRIBUTING: fmt + clippy `-D warnings` + test); the repository enforces
none of it. The gap between declared standard and enforced standard is the
central finding. Fixing the P0 set (roughly 1–2 focused days) makes the release
defensible.

## 2. Current Readiness Score: 37 / 100

| Category | Max | Score | Basis |
|---|---|---|---|
| CI & Automation | 20 | 3 | Issue/PR templates exist; zero workflows, zero dependabot, zero CODEOWNERS |
| Security & Governance | 20 | 8 | Secret scanning + push protection ON; CoC/SECURITY docs excellent; Dependabot updates OFF, no branch protection |
| Rust Tooling | 15 | 2 | Nothing beyond stock rustfmt/clippy; no toolchain pin, no deny.toml |
| Developer Experience | 10 | 2 | Docs lead; `scripts/` empty, no justfile/Makefile, no pre-commit |
| Release Engineering | 15 | 4 | CHANGELOG + semver declared; no tags, no releases, no MSRV pin |
| Workspace Quality | 10 | 6 | Clean dependency direction; naming + duplication issues |
| Repository Hygiene | 10 | 3 | 23 warnings, `todo!()`, panic-in-mapping, empty files, unused dep |
| Documentation & Community | 10 | 9 | Verified consistent; 2 minor inconsistencies (see §5) |
| **Total** | **100** | **37** | |

## 3. Missing Repository Infrastructure

Verified absent (local `ls`/`find`, remote GitHub API):

| Item | Status | Evidence |
|---|---|---|
| `.github/workflows/` | ❌ | directory absent; API `workflows.total_count = 0` |
| `.github/dependabot.yml` | ❌ | absent; API `dependabot_security_updates: disabled` |
| `.github/CODEOWNERS` | ❌ | absent |
| `rust-toolchain.toml` | ❌ | absent (README claims "Rust 1.85+" — unpinned) |
| `clippy.toml` | ❌ | absent |
| `deny.toml` | ❌ | absent |
| `rustfmt.toml` | ❌ | absent (stock defaults only) |
| `.editorconfig` | ❌ | absent |
| `.gitattributes` | ❌ | absent |
| `.cargo/config.toml` | ❌ | absent |
| `Makefile` / `justfile` / `Justfile` | ❌ | absent |
| `.pre-commit-config.yaml` | ❌ | absent |
| `scripts/` content | ❌ | directory exists, **empty** |
| Git tags / GitHub Releases | ❌ | `git tag -l` empty; API releases = 0 |

## 4. Missing GitHub Automation

1. **CI workflow** — CONTRIBUTING declares the pre-PR gate; nothing enforces
   it. The #1 release blocker.
2. **Release workflow** — no tag-triggered release, no release-note
   generation.
3. **Dependabot** — no `dependabot.yml`; security updates explicitly disabled.
   Contradicts SECURITY.md's "dependency updates land in dedicated, reviewed
   PRs".
4. **CODEOWNERS** — no default reviewer routing.
5. **Stale/issue automation** — none.
6. **Branch protection** — API: `Branch not protected (404)`; all three merge
   styles enabled, auto-merge off, no PR requirement.

## 5. Missing Rust Tooling

- **rust-toolchain.toml** — edition 2024 requires ≥1.85; nothing pins or
  verifies it. One compiler bump on CI could silently break local/CI parity.
- **deny.toml + cargo-deny** — no license/advisory/bans gate; SECURITY.md
  promises supply-chain care.
- **clippy.toml** — no lint policy; the effective policy is default, and it is
  not clean.
- **rustfmt.toml** — formatting policy implicit.
- **cargo-nextest** — optional at this scale (nice-to-have).
- **taplo** — few TOML files (nice-to-have).
- **.editorconfig / .gitattributes** — absent; cross-platform line-ending
  normalization prevents whitespace churn for mixed-OS contributors.

## 6. Missing Developer Experience

- **`scripts/` is empty** while ROADMAP M2 and the README tree reference
  "development tooling". A single `scripts/dev.sh` (up → migrate → run server
  → run simulator) would honor the North Star ("running in under five
  minutes").
- **No task runner** (justfile or Makefile).
- **No pre-commit** — the fmt/clippy gate exists only as prose.
- **Onboarding** — CONTRIBUTING covers setup; the missing piece is mechanical
  (scripts), not informational.

## 7. Missing Release Engineering

- **Versioning policy** — CHANGELOG links semver.org, crates are `0.1.0`, but
  no document states the policy. ROADMAP M4 schedules the first tag; the
  policy should precede it.
- **Tags & releases** — zero tags, zero releases; no release workflow.
- **MSRV policy** — README: "Rust 1.85+"; no `rust-toolchain.toml`, no CI MSRV
  check, no `MSRV:` note in manifest metadata.
- **Changelog discipline** — CHANGELOG exists (empty `[Unreleased]`); no
  automation backs it — acceptable for a maintainer-run project.
- **Crate metadata** — all 6 Cargo.toml files lack `license`, `repository`,
  `description`; remote license detection reports `null`.

## 8. Missing Repository Governance

- **Milestones** — none; ROADMAP M0–M5 map naturally onto milestones.
- **Label taxonomy** — 8 default labels only; issue templates reference
  `["bug","triage"]`/`["feature","triage"]` — the `triage` label does not
  exist.
- **Discussions** — API: `has_discussions: false`; SUPPORT.md handles this
  honestly with a conditional; config.yml's Discussions URL is currently dead.
- **Branch protection & merge hygiene** — unprotected `main`, three merge
  styles, `delete_branch_on_merge: false`.
- **Private vulnerability reporting** — status unverifiable via API; SECURITY.md
  points at the Security tab; should be confirmed enabled.

## 9. Technical Debt

| # | Item | Location | Notes |
|---|---|---|---|
| 1 | 23 compiler warnings | `apps/server` | unused imports/structs from incomplete layering |
| 2 | `.expect()` in DB→domain mapping | `repositories/postgres/models/device_record.rs:25,27` | panics on corrupt row; `main.rs` expects acceptable as entry-point errors |
| 3 | `todo!()` | `services/device_service.rs:24` | `create_device` unimplemented |
| 4 | Empty source files | `services/telemetry_service.rs`, `repositories/postgres/models/telemetry_record.rs` | half-finished work committed as structure |
| 5 | Unused dependency `serde_json` | `apps/server/Cargo.toml:10` | zero usages in `src/` |
| 6 | Typo `Postres` | `device_repository.rs` + re-export `postgres/mod.rs:4` | 4 occurrences; cheap now, expensive after release |
| 7 | Boilerplate `add()` | `crates/{core,common,telemetry,transport}/src/lib.rs` | 4 copies of `cargo new` template code |
| 8 | No `workspace.dependencies` / `workspace.package` | root `Cargo.toml` | deps & `0.1.0` duplicated across 6 manifests |
| 9 | No crate metadata | all Cargo.toml | GitHub license widget shows null |
| 10 | `updated_at` never refreshed; no `(device_id, created_at)` index | migrations | tracked in ROADMAP M1 |

## 10. Priority Matrix

### P0 — Required before public release

| Item | Why | Impact |
|---|---|---|
| CI workflow (fmt + clippy `-D warnings` + test) | CONTRIBUTING declares the gate; nothing enforces it | Declared standard becomes enforced standard |
| Resolve 23 warnings → clippy-clean | CI with `-D warnings` cannot be enabled before this | Unblocks CI |
| `rust-toolchain.toml` (pin 1.85.x) | README claims 1.85+; reproducibility | Deterministic builds |
| Cargo metadata in all 6 manifests | License detection null; community signal | Legal clarity + license widget |
| Enable Dependabot security updates + `dependabot.yml` | SECURITY.md promises dedicated dependency PRs | Supply-chain visibility |
| Fix dead Discussions link (enable **or** remove) | Active doc inconsistency | Docs fully truthful |
| Add `triage` label (+ optional phase labels) | Issue templates already reference `triage` | Template labels resolve |

### P1 — Strongly recommended

| Item | Why | Impact |
|---|---|---|
| Branch protection on `main` | Unguarded main + merge commits | Protected default branch |
| `deny.toml` + cargo-deny CI job | Matches SECURITY.md posture | License/advisory gate |
| Release workflow: tag → notes → GitHub Release | Zero releases today | First tag v0.1.0 automation |
| `scripts/dev.sh` | scripts/ empty; North Star onboarding | One-command stack |
| CODEOWNERS (default to maintainer) | Standard governance | Review routing |
| `.editorconfig` + `.gitattributes` | Mixed-OS contributors | Clean diffs |
| Milestones M0–M5 + phase labels | ROADMAP is prose only | Visible progress |
| Remove `Postres` typo | Cost grows after release | API cleanliness |
| Remove empty files / `add()` boilerplate; drop `serde_json` | Hygiene debt | Clean tree |
| `Result`-based enum mapping | Panic on corrupt data | No request-thread crashes |

### P2 — Nice to have

| Item | Why |
|---|---|
| `justfile` (or Makefile) | Command ergonomics |
| `rustfmt.toml` + `clippy.toml` explicit policy | Codified style |
| `cargo-nextest` | Faster test UX (low value at this scale) |
| `cargo-machete` CI step | Unused-dep detection |
| Stale-bot / auto-labeler | Issue triage automation |
| `workspace.dependencies` / `workspace.package` | Single-source manifests |
| pre-commit hook bundle | Local gate mirror of CI |

### P3 — Future

| Item | Why |
|---|---|
| Release automation (release-please / cargo-release) | Once cadence exists |
| `cargo-semver-checks` | Public-API stability (post-1.0) |
| MSRV matrix CI | Once a support window is committed |
| MIRI / fuzz / benchmarks | Post-M4 hardening |
| Docs link-checker + spellcheck CI | Doc drift prevention |

## 11. Action Plan

**Phase 0 — Foundation pins (≈ half a day)**: toolchain pin; manifest
metadata; dependabot + label taxonomy; editorconfig/gitattributes; Discussions
consistency fix.

**Phase 1 — Enforcement (≈ 1–1.5 days)**: fix 23 warnings; CI workflow;
branch protection; hygiene sweep (add()/empty files/serde_json/Postres);
`scripts/dev.sh`.

**Phase 2 — Release machinery (≈ half a day)**: version policy; release
workflow; CODEOWNERS; first tag.

**Phase 3 — Future (post-release, per ROADMAP M4+)**: P2/P3 items.

*(Execution order follows the constraint chain: repository health → toolchain
→ developer experience → CI → governance → release engineering. The ordered,
sprint-structured execution of this plan lives in `docs/IMPLEMENTATION_PLAN.md`.)*
