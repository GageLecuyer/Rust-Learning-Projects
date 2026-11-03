# Security Policy

## Supported versions

This repository contains small learning projects with no formal releases yet. Only the latest code on the `Main` branch is supported, and security fixes are applied there.

## Reporting a vulnerability

**Please do not report security vulnerabilities in public GitHub issues, discussions, or pull requests.**

Report them privately through GitHub's private vulnerability reporting:

1. Go to <https://github.com/GageLecuyer/Rust-Learning-Projects/security/advisories/new>
   (or open the repository's **Security** tab and click **Report a vulnerability**).
2. Describe the problem, which project and file it affects, steps to reproduce it, and what impact you think it has.

Only you and the repository maintainer can see the report.

## What to expect

- **Acknowledgement within 14 days** of your report.
- An initial assessment that says whether the report is accepted as a vulnerability, and how severe it is.
- For accepted reports, a fix on `Main` as soon as practical. The goal is within 60 days for medium or higher severity.
- Credit in the fix's advisory and in [CHANGELOG.md](CHANGELOG.md), unless you'd rather stay anonymous.
- Updates on progress at least every 14 days until the report is resolved.

Once a fix is available, the vulnerability is disclosed publicly through a GitHub Security Advisory on this repository.

## Scope

In scope: code in this repository (`Calcualtor`, `guessing_game`, `snake_game`) and how it uses its dependencies.

For vulnerabilities in a third-party crate (for example `rand` or `piston_window`), report them to that crate's maintainers or to the [RustSec Advisory Database](https://rustsec.org/). Let me know too if it affects this repository. Dependency updates are proposed automatically by Dependabot.
