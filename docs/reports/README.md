# Reports

Store implementation and debugging reports here.

- Use one file per issue/slice with this pattern: `docs/reports/<slug>.report.md`
- Keep the reusable template at `docs/reports/{issue}.report.md`
- Include repro commands, validation commands, and concrete root cause notes

Current report for this slice:
- `docs/reports/2026-03-03-minimal-c-abi-cli-slice.report.md`

Additional investigation report:
- `docs/reports/2026-03-03-pps-fdump-crash-native-isolation.report.md`

Hardening follow-up report:
- `docs/reports/2026-03-03-pps-fdump-crash-hardening-fix.report.md`

Docker parity foundation + MVP harness report:
- `docs/reports/2026-03-03-docker-dual-cli-parity-phase1-mvp.report.md`

Sequenced progress report (CLI + C ABI + renderers + docker parity checks):
- `docs/reports/2026-03-03-cli-abi-progress-sequenced.report.md`

Native suite wrapper pass report:
- `docs/reports/2026-03-03-native-suite-wrapper-pass1.report.md`

C ABI divergence tracker:
- `docs/reports/2026-03-04-c-abi-divergence-tracker.report.md`

## Reporting Standards
- Use one report file per implementation slice and include exact validation commands.
- Record durable decisions with `[DECISION]`, temporary shortcuts with `[HACK]`, and hard blockers with `[BLOCKER]`.
- Include at least one baseline-vs-local docker comparison command/result whenever parity work is in scope.
- End each report with a continuation prompt that states current context and the next ordered goals.
