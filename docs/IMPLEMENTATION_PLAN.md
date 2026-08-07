# IMPLEMENTATION_PLAN

> Engineering implementation plan derived from
> `docs/OPEN_SOURCE_READINESS_AUDIT.md`. Converts the audit's findings into
> independently executable sprints. Each sprint: can be completed alone,
> produces immediate value, keeps the repository buildable, minimizes merge
> conflicts, and prepares the next sprint.
>
> Execution order follows the constraint chain:
> **repository health → toolchain → developer experience → CI → governance →
> release engineering.**

## How to execute

- One sprint = one or two PRs, merged in order. Do not start the next sprint
  before the previous one's acceptance criteria pass.
- Branch names per CONTRIBUTING: `chore/` or `refactor/` for S1–S3, `feat/`
  for S4–S6 where they add capability.
- Commit style per CONTRIBUTING (imperative, scoped).
- Nothing here generates code: each task describes the *change* and the
  *decision* the maintainer makes when implementing it.
- Every sprint keeps `cargo build --workspace` and `cargo test --workspace`
  green. Sprint 1 is the only one that intentionally *touches* the tree
  heavily; later sprints are additive.

---

## Sprint 1 — Compiler health & workspace hygiene

**Objective:** the workspace builds with **zero warnings**, the tree contains
no placeholder code, and the public typo is gone. This is the prerequisite for
every strict-lint step that follows.

**Tasks**

1.1 **Classify and fix the 23 `apps/server` warnings.** Each warning is one of:
    - *Unused import* → remove it.
    - *Unused variable in `DeviceService::create_device`* (code/name/device_type)
      → prefix `_` or restructure the signature (decide: the parameters are
      the intended API; keep the signature, mark unused).
    - *Never-constructed structs / never-used traits / unused `new` fns*
      (`TelemetryRequest`, `MetricRequest`, `TelemetryResponse`,
      `DeviceRepository`, `TelemetryRepository`, `Postgres*Repository`,
      `DeviceService`) → **keep**, annotate `#[allow(dead_code)]` with a
      `// ROADMAP M1` reference comment. These are the designed API surface,
      not dead weight; deleting them would contradict the architecture.
    - *Unused fields `config`/`db` in `AppState`, `Config` fields* → keep the
      fields, allow the lint, reference M1. They are wired state, not mistakes.
    - *Unused imports from wildcard re-exports* (`dto::*`,
      `repositories::*`) → narrow the re-exports in `dto/mod.rs` and
      `repositories/mod.rs` to explicit items.

    *Reason:* CI with `-D warnings` (Sprint 4) cannot be enabled otherwise.
    *Impact:* clean build; the warning list stops masking real issues.
    *Effort:* M (30–60 min of classification + edits).

1.2 **Remove the `add()` boilerplate** from `crates/core/src/lib.rs`,
    `crates/common/src/lib.rs`, `crates/telemetry/src/lib.rs`,
    `crates/transport/src/lib.rs` (function + its test).
    *Reason:* 4 copies of `cargo new` template code pollute every public API
    surface. *Impact:* honest crates. *Effort:* S.

1.3 **Delete the empty source files** `apps/server/src/services/telemetry_service.rs`
    and `apps/server/src/repositories/postgres/models/telemetry_record.rs`;
    remove their `mod` declarations and re-exports. They are half-finished M1
    work committed as structure; M1 will recreate them with content.
    *Reason:* debt §9.4. *Impact:* no dead files. *Effort:* S.

1.4 **Remove the unused `serde_json` dependency** from
    `apps/server/Cargo.toml` (zero usages in `src/`; verified by audit).
    *Reason:* debt §9.5. *Impact:* smaller dependency surface. *Effort:* S.

1.5 **Rename `PostresDeviceRepository` → `PostgresDeviceRepository`**
    (struct, `impl` block, re-export in `postgres/mod.rs`).
    *Reason:* debt §9.6; cost grows after external consumers exist.
    *Impact:* correct public name. *Effort:* S (mechanical).

1.6 **Add `.editorconfig` and `.gitattributes`** (LF normalization for text
    files; sensible defaults for Rust, YAML, Markdown, TOML).
    *Reason:* mixed-OS contributor hygiene. *Impact:* clean diffs.
    *Effort:* S.

**Dependencies:** none — first sprint.

**Estimated effort:** ~1 day.

**Expected repository improvements:** zero-warning build; no placeholder
code; no empty files; no unused dependency; no public typo; cross-platform
line-ending hygiene.

**Acceptance criteria**
- `cargo build --workspace` reports zero warnings.
- `cargo test --workspace` green.
- `rg "Postres" apps/` → no matches.
- No empty `.rs` files remain in `apps/` or `crates/`.
- `serde_json` absent from `apps/server/Cargo.toml`.
- `cargo fmt --all -- --check` green.

**Merge strategy:** single PR (`chore/workspace-hygiene`). No other branches
in flight, so no conflicts. Everything here is tree-wide edits — doing it as
one PR prevents it colliding with later additive PRs.

**Prepares:** Sprint 2 (clippy `-D warnings` needs the zero-warning base).

---

## Sprint 2 — Toolchain & manifest baseline

**Objective:** deterministic toolchain, explicit lint/format policy, complete
manifest metadata, and a supply-chain policy file — all without touching CI.

**Tasks**

2.1 **Add `rust-toolchain.toml`** — channel `1.85.x` (latest patch), components
    `rustfmt`, `clippy`.
    *Reason:* README claims Rust 1.85+; unpinned toolchains make local/CI
    parity fragile. *Impact:* reproducible builds for every contributor.
    *Effort:* S.

2.2 **Add `rustfmt.toml`** — edition 2024, explicit style decisions
    (defaults are fine; the file makes the policy visible and reviewable).
    *Reason:* audit §5. *Impact:* codified formatting. *Effort:* S.

2.3 **Add `clippy.toml`** — documents lint policy; expected content is empty
    or minimal (the policy is: `-D warnings`, period).
    *Reason:* audit §5. *Impact:* lint policy visibility. *Effort:* S.

2.4 **Run `cargo clippy --workspace --all-targets -- -D warnings` and fix or
    justify every new lint** surfaced beyond rustc's list (Sprint 1 only
    cleared rustc warnings; clippy adds its own).
    *Reason:* this is the exact command CI will run. *Impact:* strict-lint
    green, locally, before automation exists. *Effort:* M.

2.5 **Add `license = "MIT"`, `repository`, `description` to all 6 Cargo.toml
    manifests.**
    *Reason:* debt §9.9; GitHub license detection currently reports `null`.
    *Impact:* license widget works; crates.io-ready metadata. *Effort:* S.

2.6 **Add `deny.toml`** — license allowlist (MIT + permitted), advisory and
    ban sections. File only; the CI job lands in Sprint 4.
    *Reason:* SECURITY.md promises supply-chain care; audit P1.
    *Impact:* policy ready for automation. *Effort:* S.

**Dependencies:** Sprint 1 (clippy-clean base).

**Estimated effort:** ~0.5 day.

**Expected repository improvements:** pinned toolchain; explicit
format/lint policy; complete manifests; license detection fixed; deny policy
ready.

**Acceptance criteria**
- `rustup show` resolves the pinned channel.
- `cargo clippy --workspace --all-targets -- -D warnings` green.
- `cargo fmt --all -- --check` green.
- `cargo metadata` shows `license`/`repository`/`description` on all packages.
- `cargo deny check` passes locally (if cargo-deny installed; otherwise the
  file is validated by its first CI run in S4).

**Merge strategy:** one PR (`chore/toolchain-baseline`). Additive files only;
no overlap with Sprint 1's deletions.

**Prepares:** Sprint 4 — CI reproduces this exact toolchain and runs this
exact clippy command.

---

## Sprint 3 — Developer experience

**Objective:** a one-command local stack and a verified onboarding path.
Developer experience precedes automation: CI should mirror what developers
already run.

**Tasks**

3.1 **Add `scripts/dev.sh`** — one command that: starts PostgreSQL via
    `docker compose -f docker/docker-compose.yml up -d`, waits for
    readiness, runs the server (`cargo run -p server`), with a `--simulator`
    flag to also run the simulator; support `dev.sh stop` for teardown.
    *Reason:* `scripts/` is empty while README/ROADMAP reference "development
    tooling"; North Star = running in under five minutes.
    *Impact:* one-command stack. *Effort:* M.

3.2 **Verify the README quickstart end-to-end on a fresh clone** (compose up
    → migrate → `GET /health` → simulator). Fix any gap found — in
    documentation wording, or in the script from 3.1.
    *Reason:* the README is the product's first experience; audit found no
    verification evidence. *Impact:* onboarding actually works.
    *Effort:* S.

3.3 **Update README/CONTRIBUTING to reference `scripts/dev.sh`** as the
    primary run path (one or two sentences; keep existing manual commands).
    *Reason:* docs must describe what exists (vision P6). *Impact:* doc
    truth. *Effort:* S.

3.4 *(Optional, deferrable to post-0.1.0)* **justfile wrapper** around the
    script. Not required for release; skip if time-boxed.

**Dependencies:** Sprints 1–2 (the commands in the script must be clean to
run).

**Estimated effort:** ~0.5 day.

**Expected repository improvements:** working `scripts/`; verified onboarding;
docs referencing real tooling.

**Acceptance criteria**
- Fresh checkout → `scripts/dev.sh` → server healthy at `APP_PORT` →
  simulator emits.
- `scripts/dev.sh stop` tears the stack down cleanly.
- README/CONTRIBUTING reference the script.

**Merge strategy:** one PR (`feat/dev-scripts`). Additive; no conflicts.

**Prepares:** Sprint 4 — the CI smoke test reuses the exact commands from
`dev.sh`.

---

## Sprint 4 — Automation: CI & dependency hygiene

**Objective:** the gate declared in CONTRIBUTING becomes enforced, and the
supply chain becomes visible. **CI before branch protection** — protection
(Sprint 5) requires status checks to exist.

**Tasks**

4.1 **Add the CI workflow** (`.github/workflows/ci.yml`): runs on PR + push to
    `main`, uses the pinned toolchain (Sprint 2.1), with a job sequence:
    1. `cargo fmt --all -- --check`
    2. `cargo clippy --workspace --all-targets -- -D warnings`
    3. `cargo test --workspace`
    4. `cargo deny check` (Sprint 2.6)
    5. **Smoke test**: start PostgreSQL (service container), run the server,
       curl `/health`, assert `status: ok`.

    Caching for cargo registry/target to keep runs fast.
    *Reason:* the #1 release blocker identified by the audit.
    *Impact:* every PR verified; CONTRIBUTING's gate enforced.
    *Effort:* M (1–2 h).

4.2 **Add `.github/dependabot.yml`** — ecosystems: `cargo` and
    `github-actions`; weekly interval; prefix labels.
    **Enable `dependabot_security_updates` in repository settings** (currently
    disabled per API).
    *Reason:* SECURITY.md promises dedicated dependency PRs; supply-chain
    visibility. *Impact:* automated dependency review. *Effort:* S.

**Dependencies:** Sprints 1–3 (warnings fixed, toolchain pinned, dev.sh
commands proven locally).

**Estimated effort:** ~1 day.

**Expected repository improvements:** enforced gate; dependency automation;
end-to-end smoke coverage in CI.

**Acceptance criteria**
- A test PR with a clippy violation or failing test shows a red check.
- A clean PR shows all green including smoke test.
- Dependabot opens its first PR within the configured interval.
- `dependabot_security_updates` reports enabled.

**Merge strategy:** one PR for the workflow + dependabot file
(`feat/ci-foundation`). Additive.

**Prepares:** Sprint 5 — branch protection requires CI checks to exist.

---

## Sprint 5 — Governance

**Objective:** protected `main`, review routing, triage taxonomy, and honest
community channels. **Automation before governance** — protection rules
reference the CI from Sprint 4.

**Tasks**

5.1 **Branch protection on `main`** (settings): require status checks
    (CI workflow), require PR review (1 reviewer — the maintainer), squash
    merge only, delete branch on merge. Disable direct pushes.
    *Reason:* audit §8; three merge styles + unguarded main.
    *Impact:* release integrity; linear history. *Effort:* S.

5.2 **Add `.github/CODEOWNERS`** — default owner: the maintainer
    (`@Adibayuluthfiansyah`), so review routing is explicit and future
    maintainers can be added.
    *Reason:* audit P1. *Impact:* review routing. *Effort:* S.

5.3 **Label taxonomy** — add `triage`, `good-first-issue` follow-up, and
    phase labels (`M0…M5`) aligned with ROADMAP. Issue templates already
    reference `triage` — currently nonexistent.
    *Reason:* audit §8. *Impact:* template labels resolve; triage works.
    *Effort:* S.

5.4 **Milestones `M0…M5`** created from ROADMAP phases; assign Sprint 1–6
    PRs to their milestone.
    *Reason:* ROADMAP is prose only. *Impact:* visible progress tracking.
    *Effort:* S.

5.5 **Discussions decision** — either enable Discussions on the repository
    (matches SUPPORT.md's intent) **or** remove the dead link from
    `.github/ISSUE_TEMPLATE/config.yml`. Pick one; do not leave the
    inconsistency.
    *Reason:* audit-found active doc inconsistency.
    *Impact:* no dead links. *Effort:* S.

5.6 *(Optional)* **pre-commit hook bundle** mirroring the CI gate locally.
    Deferrable to post-0.1.0; CI already covers the gate.

**Dependencies:** Sprint 4 (protection rules require CI).

**Estimated effort:** ~0.5 day.

**Expected repository improvements:** protected default branch; explicit
review routing; working label/milestone taxonomy; consistent community
channels.

**Acceptance criteria**
- Direct push to `main` is blocked; PRs require green CI + review.
- CODEOWNERS shows the maintainer as default reviewer.
- `triage` label exists; issue templates resolve.
- Milestones M0–M5 exist with at least Sprint 1–6 PRs assigned.
- Discussions either enabled or the config.yml link removed.

**Merge strategy:** one PR for file-based items (CODEOWNERS, config.yml);
settings changes applied via the GitHub UI in the same session.

**Prepares:** Sprint 6 — a clean, protected `main` is the release base.

---

## Sprint 6 — Release engineering & first release

**Objective:** repeatable release mechanics and the first tagged release
`v0.1.0`. **Governance before release engineering** — the release lands on a
protected, CI-verified base.

**Tasks**

6.1 **Document the versioning policy** — one short section in CONTRIBUTING
    (and a line in ROADMAP): 0.x allows breaking changes; 1.0 commits to
    API stability; tags follow `vX.Y.Z`.
    *Reason:* audit §7; the policy must precede the first tag.
    *Impact:* release semantics understood by users. *Effort:* S.

6.2 **Populate the CHANGELOG** — audit Sprint 1–5 PRs into the `[Unreleased]`
    section (Keep a Changelog categories). Everything user-visible since
    project start qualifies (e.g., bind-address fix, compose fix, docs
    suite, CI).
    *Reason:* CHANGELOG is currently empty by design; the release makes it
    the release notes. *Impact:* truthful release notes. *Effort:* S.

6.3 **Add the release workflow** (`.github/workflows/release.yml`):
    tag-triggered (`v*`); `cargo build --release`; creates a GitHub Release
    (draft-first) with notes from the CHANGELOG.
    *Reason:* audit §4.2/§7. *Impact:* repeatable release mechanics.
    *Effort:* M.

6.4 **Cut `v0.1.0`** — tag `main` at the release commit; publish the draft
    release; verify: license widget shows MIT; SECURITY.md's supported-versions
    table becomes truthful (0.x line active); private vulnerability reporting
    enabled in settings.
    *Reason:* the audit's P0/P1 set is done; release is the milestone.
    *Impact:* first public release. *Effort:* S.

6.5 **Push the full docs suite** if not already on the remote (README,
    LICENSE, community files, `docs/`).
    *Reason:* remote is behind local; the release must carry the docs.
    *Impact:* repo matches audit state. *Effort:* S.

**Dependencies:** Sprints 1–5.

**Estimated effort:** ~1 day.

**Expected repository improvements:** version policy; populated CHANGELOG;
tag-triggered release workflow; `v0.1.0` published; license detected.

**Acceptance criteria**
- Tag `v0.1.0` exists and GitHub Release published with CHANGELOG-derived
  notes.
- Release workflow file present and dry-runnable (or a `v0.1.1`-style test
  tag confirms it).
- Remote repo shows MIT license; private vulnerability reporting enabled.
- SECURITY.md's supported-versions table matches reality.

**Merge strategy:** release commit on `main` after the release workflow PR;
the tag is the deliverable.

**Prepares:** post-0.1.0 backlog (below).

---

## Tasks that can safely wait until after v0.1.0

These are real improvements with real value — none of them gate the first
public release:

| Task | Source | Notes |
|---|---|---|
| `workspace.dependencies` / `workspace.package` consolidation | Debt §9.8 | Refactor; better done when dependency set stabilizes |
| justfile (task-runner wrapper) | Audit P2 | Optional ergonomics on top of `dev.sh` |
| `cargo-nextest` adoption | Audit P2 | Test UX; low value at current test count |
| `cargo-machete` CI step | Audit P2 | Unused-dep automation (Sprint 1 removed the known one) |
| Stale-bot / auto-labeler | Audit P2 | Issue triage automation |
| pre-commit hook bundle | Audit P2 | Local mirror of CI; CI already enforces |
| Release automation (release-please / cargo-release) | Audit P3 | Once release cadence exists |
| `cargo-semver-checks` | Audit P3 | Meaningful after public API exists |
| MSRV matrix CI | Audit P3 | Once a support window is committed |
| MIRI / fuzz / benchmarks | Audit P3 | Post-M4 hardening |
| Docs link-checker + spellcheck CI | Audit P3 | Doc drift prevention |
| CODEOWNERS expansion | Audit P3 | When maintainers multiply |
| `taplo` TOML formatting | Audit P2 | Few TOML files today |

---

## Dependency graph (summary)

```
S1 hygiene ──▶ S2 toolchain ──▶ S3 DX ──▶ S4 CI ──▶ S5 governance ──▶ S6 release
```

Each sprint's output is the next sprint's prerequisite; no sprint depends on a
later one. If a sprint is skipped, the constraint chain breaks (e.g., S4
without S1 = CI is permanently red).

**Total estimated effort to `v0.1.0`: 4.5–5 focused days.**
