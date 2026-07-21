# Disposition trail — de-novo project audit A (2026-07-21)

**Report:** `audits/2026-07-21-de-novo-project-audit.md`  
**Status:** in progress — default gate closed; residuals tracked.

| ID | Finding | Disposition | Landing |
|---|---|---|---|
| P0 | Builder ignores unmet flux while status forbids erosion | **fix (default path)** | Admission check via `requisite_chain`; refuse exit 2; lock `Drop` |
| P0 residual A | Waived artifacts look lawful in census | **open / deferred** | `--allow-unmet` logs waiver; **store census does not yet label provisional**; no key/metadata bit. `#form-flux-web` known-incomplete (3) discloses this. Not fully “integrated” until census or key flags exist |
| P0 residual B | Waived keys == lawful keys | **open / deferred** | Same root as residual A — need explicit provisional key field or status labeling |
| P0 residual C | No integration test for gate | **open / deferred** | Gate lives in bin; lib exercises `requisite_chain` only |
| P1 | Builder lock TOCTOU | **defer** | Drop guard helps setup failure; atomic `create_new` still owed |
| P1 | Query `put` errors swallowed | **defer** | API shape change |
| P1 | 64-bit FNV | **defer** | Named on complete-key segment |
| **New** | `world_dir` only read `rest[0]` | **fix** | First **non-flag** positional; name is second positional |
| Under-keying | dep versions omitted from keys | **fix (direct deps)** | `Key::with_dep_versions`; query keys for IT/climate/erosion/water; nomotheke test `key_with_dep_versions_embeds_each_dep_identity` |

**Verify note (auditor 2026-07-21):** Default refuse path independently reproduced. Level 5 + TILE_NX 64 panics sphere overflow (pre-existing).
