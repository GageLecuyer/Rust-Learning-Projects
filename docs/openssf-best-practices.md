# OpenSSF Best Practices: Passing criteria evidence

Project entry: <https://www.bestpractices.dev/en/projects/16313>
Repository: <https://github.com/GageLecuyer/Rust-Learning-Projects>

This file maps each Passing criterion to evidence in this repository. Status values:

- **Met**: the repository satisfies it once this documentation branch is merged into `Main`.
- **Met after manual step**: you must change a GitHub setting, or do something on GitHub, first. See the [manual steps](#manual-steps).
- **N/A**: the criterion doesn't apply to this project, for the reason given.
- **Your call**: depends on facts only the maintainer knows. Answer honestly.
- **Unmet**: not satisfied. Only SUGGESTED criteria are left Unmet, which doesn't block Passing.

`R` = `https://github.com/GageLecuyer/Rust-Learning-Projects`

## Basics

| Criterion | Level | Status | Evidence / justification to paste |
| --- | --- | --- | --- |
| description_good | MUST | Met | `R/blob/Main/README.md`: first section describes what the repository is and what each program does. |
| interact | MUST | Met | `R/blob/Main/README.md#reporting-bugs-and-requesting-enhancements` covers how to get it, give feedback, and contribute. |
| contribution | MUST | Met | `R/blob/Main/CONTRIBUTING.md` |
| contribution_requirements | SHOULD | Met | `R/blob/Main/CONTRIBUTING.md#rust-coding-conventions`: rustfmt, clippy, naming, test policy. |
| floss_license | MUST | Met | MIT. `R/blob/Main/LICENSE` |
| floss_license_osi | SUGGESTED | Met | MIT is OSI-approved: <https://opensource.org/license/mit> |
| license_location | MUST | Met | `R/blob/Main/LICENSE` (standard location, top level) |
| documentation_basics | MUST | Met | `R/blob/Main/README.md` (install, run, test) and each project's README. |
| documentation_interface | MUST | Met | `R/blob/Main/Calcualtor/README.md`, `R/blob/Main/guessing_game/README.md`, `R/blob/Main/snake_game/README.md`: inputs, outputs, errors, exit codes, controls. |
| sites_https | MUST | Met | Project site, repo, and downloads are all on GitHub over HTTPS: `R` |
| discussion | MUST | Met | GitHub Issues and pull requests are searchable, addressable by URL, open to new participants, and need no proprietary client: `R/issues` |
| english | SHOULD | Met | All documentation is in English. Bug reports are accepted in English. |
| maintained | MUST | Met | Commits in Oct–Nov 2026 (`R/commits/Main`). SECURITY.md commits to response times. |

## Change control

| Criterion | Level | Status | Evidence / justification to paste |
| --- | --- | --- | --- |
| repo_public | MUST | Met | `R` (public GitHub repository) |
| repo_track | MUST | Met | Git tracks who changed what and when: `R/commits/Main` |
| repo_interim | MUST | Met | Interim work is committed and pushed to GitHub between releases: `R/commits/Main` |
| repo_distributed | SUGGESTED | Met | Git (distributed VCS). |
| version_unique | MUST | Met after manual step | Each package declares `version = "0.1.0"` in `Cargo.toml`. Create a `v0.1.0` tag/release (step 6) so the release identifier is unique. |
| version_semver | SUGGESTED | Met | Cargo SemVer versions (`0.1.0`). CHANGELOG states SemVer. |
| version_tags | SUGGESTED | Met after manual step | Create the `v0.1.0` tag (step 6): `R/releases` |
| release_notes | MUST | Met after manual step | `R/blob/Main/CHANGELOG.md` (human-written, not a raw git log). Paste its section into the v0.1.0 release (step 6). |
| release_notes_vulns | MUST | N/A | No publicly known vulnerabilities have been fixed. Use N/A if offered, otherwise Met with this explanation. |

## Reporting

| Criterion | Level | Status | Evidence / justification to paste |
| --- | --- | --- | --- |
| report_process | MUST | Met | `R/blob/Main/README.md#reporting-bugs-and-requesting-enhancements` → `R/issues` |
| report_tracker | SHOULD | Met | GitHub Issues: `R/issues` |
| report_responses | MUST | Your call | The repository has **0 issues so far**. Don't claim responses that didn't happen. Justification: "No bug reports have been submitted yet. Issues are monitored and CONTRIBUTING.md commits to responding within 14 days." Answer N/A if the form allows it, otherwise Met with that text. |
| enhancement_responses | SHOULD | Your call | Same situation as report_responses: no enhancement requests yet. |
| report_archive | MUST | Met | Public, searchable archive of reports: `R/issues?q=` |
| vulnerability_report_process | MUST | Met | `R/blob/Main/SECURITY.md` |
| vulnerability_report_private | MUST | Met after manual step | SECURITY.md points to GitHub private vulnerability reporting. **You must turn it on** (step 3), or the link won't work. |
| vulnerability_report_response | MUST | N/A | No vulnerability reports received in the last 6 months. SECURITY.md commits to acknowledging within 14 days. |

## Quality

| Criterion | Level | Status | Evidence / justification to paste |
| --- | --- | --- | --- |
| build | MUST | Met | Standard Cargo build (`cargo build`), documented in `R/blob/Main/README.md#running-a-project`. Run in CI: `R/blob/Main/.github/workflows/ci.yml` |
| build_common_tools | SUGGESTED | Met | Cargo, the standard Rust build tool. |
| build_floss_tools | SHOULD | Met | rustc and Cargo are open source (MIT/Apache-2.0). |
| test | MUST | Met | `cargo test` in each project: `R/blob/Main/Calcualtor/tests/cli.rs`, `R/blob/Main/guessing_game/tests/cli.rs`, `R/blob/Main/snake_game/src/snake.rs` (17 tests total). |
| test_invocation | SHOULD | Met | Standard `cargo test`, documented in `R/blob/Main/README.md#testing` |
| test_most | SUGGESTED | Your call | Tests cover every calculator operation, a full guessing game, and the snake's movement and collision logic. Window rendering, keyboard handling, and `random_food` in `snake_game/src/main.rs` are **not** tested. No coverage tool has been run, so don't claim a percentage. Mark Unmet unless you measure it. |
| test_continuous_integration | SUGGESTED | Met after manual step | `R/blob/Main/.github/workflows/ci.yml` runs on every push and PR. Paste a link to a passing run: `R/actions/workflows/ci.yml` |
| test_policy | MUST | Met | `R/blob/Main/CONTRIBUTING.md#tests`: "any new functionality must come with automated tests" |
| tests_are_added | MUST | Met | The test policy was followed in the commit that fixed the guessing_game bugs: `guessing_game/tests/cli.rs` checks both fixes. Link that commit after merge. |
| tests_documented_added | SUGGESTED | Met | Test requirement is in CONTRIBUTING.md and in the PR steps. |
| warnings | MUST | Met | rustc warnings and `cargo clippy -D warnings` run in CI: `R/blob/Main/.github/workflows/ci.yml` |
| warnings_fixed | MUST | Met | Both existing clippy warnings were fixed (unused import, collapsible `if`). CI fails on any new warning. |
| warnings_strict | SUGGESTED | Met | Warnings are errors in CI (`-D warnings`). |

## Security

| Criterion | Level | Status | Evidence / justification to paste |
| --- | --- | --- | --- |
| know_secure_design | MUST | Your call | This is about **you**, the primary developer. Only answer Met if you know secure design principles (least privilege, fail-safe defaults, input validation, etc.). If you've covered them in coursework, say so. |
| know_common_errors | MUST | Your call | Same: only Met if you know the common vulnerability types for this kind of software (e.g. unvalidated input, panics on bad input, integer overflow) and how to avoid them. |
| crypto_published … crypto_random (9 criteria) | MUST/SHOULD | N/A | The software doesn't use cryptography, store passwords, or generate security secrets. `rand` is only used for gameplay (secret number, food position), not for security. |
| delivery_mitm | MUST | Met | Delivered only through GitHub over HTTPS/SSH. Cargo fetches crates over HTTPS from crates.io, with checksums in `Cargo.lock`. |
| delivery_unsigned | MUST | Met | No hashes are fetched over HTTP. Cargo.lock checksums come from the repository itself. |
| vulnerabilities_fixed_60_days | MUST | Met | `cargo audit` found no known vulnerabilities in any lockfile (see below). CI re-runs `cargo audit` weekly. |
| vulnerabilities_critical_fixed | SHOULD | Met | No known critical vulnerabilities. Dependabot and weekly `cargo audit` surface new ones. |
| no_leaked_credentials | MUST | Met | The repository has no credentials, tokens, or private keys. Workflows use only the automatic `GITHUB_TOKEN`. Turn on secret scanning (step 4) for ongoing evidence. |

## Analysis

| Criterion | Level | Status | Evidence / justification to paste |
| --- | --- | --- | --- |
| static_analysis | MUST | Met | `cargo clippy` runs on every push and PR: `R/blob/Main/.github/workflows/ci.yml` |
| static_analysis_common_vulnerabilities | SUGGESTED | Unmet (or Met after step 5) | Clippy is a general linter, not a vulnerability scanner. If you turn on CodeQL default setup (step 5) and it analyzes Rust, you can cite it. |
| static_analysis_fixed | MUST | Met | The clippy findings were fixed in this change. CI fails on any new finding. |
| static_analysis_often | SUGGESTED | Met | Clippy runs on every commit pushed to `Main` and on every PR. |
| dynamic_analysis | SUGGESTED | Unmet | No fuzzing or dynamic analysis tools are used. |
| dynamic_analysis_unsafe | SUGGESTED | N/A | Written in Rust with no `unsafe` blocks. |
| dynamic_analysis_enable_assertions | SUGGESTED | Unmet | Not applicable without dynamic analysis. (`cargo test` builds in debug mode, which enables debug assertions and integer overflow checks.) |
| dynamic_analysis_fixed | MUST | N/A | No dynamic analysis is performed, so there are no findings to fix. |

## Dependency vulnerability scan

`cargo audit` (RustSec advisory database, 1403 advisories) was run locally on 2026-11-03 against each project's committed `Cargo.lock`:

| Project | Vulnerabilities | Informational warnings |
| --- | --- | --- |
| Calcualtor | 0 | 0 |
| guessing_game | 0 | 0 |
| snake_game | 0 | 4 "unmaintained" warnings, all transitive dependencies of `piston_window`: `paste` 1.0.15 (RUSTSEC-2024-0436), `rusttype` 0.9.3 (RUSTSEC-2021-0140), `ttf-parser` 0.15.2 and 0.25.1 (RUSTSEC-2026-0192) |

"Unmaintained" warnings aren't vulnerabilities and don't fail `cargo audit`. They can only go away when `piston_window` updates its dependencies, which Dependabot will propose. CI reruns the scan on every push and weekly: `R/actions/workflows/ci.yml`.

## Manual steps

1. **Merge** the `openssf-passing` branch into `Main` with a pull request. All evidence links above point to `Main`.
2. **Check CI**: open `R/actions/workflows/ci.yml` and confirm all 6 jobs are green.
3. **Private vulnerability reporting**: Settings → Security → Advisories (Code security) → **Private vulnerability reporting** → Enable.
4. **Dependabot and secrets**: on Settings → Code security, turn on **Dependency graph**, **Dependabot alerts**, **Dependabot security updates**, **Secret scanning**, and **Push protection**.
5. *(Optional)* **CodeQL**: on the same page, Code scanning → CodeQL analysis → **Set up → Default**.
6. **Release**: Releases → **Draft a new release** → new tag `v0.1.0` on `Main` → title `v0.1.0` → paste the CHANGELOG "Unreleased" section as the notes (rename it to `0.1.0` in CHANGELOG.md first if you like) → Publish.
7. **Branch ruleset**: Settings → Rules → Rulesets → **New branch ruleset**. Name it `Main`, set enforcement to Active, target the default branch, and turn on:
   - Restrict deletions
   - Block force pushes
   - Require a pull request before merging. Set **required approvals to 0**: you're the only maintainer and GitHub won't let you approve your own PR.
   - Require status checks to pass. Add the six `fmt / clippy / test (…)` and `cargo audit (…)` checks, which show up after the first CI run.
8. **Delete the stale `main` branch** (lowercase) on GitHub: Branches → `main` → delete. It's already merged into `Main` through PR #1, and having both names confuses Git on Windows and macOS, which treat them as the same name.
9. **Questionnaire**: at <https://www.bestpractices.dev/en/projects/16313/edit>, fill each criterion using the tables above. Replace `R` with the full repository URL.
