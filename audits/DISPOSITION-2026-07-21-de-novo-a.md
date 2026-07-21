# Disposition trail — de-novo project audit A (2026-07-21)

**Report:** `audits/2026-07-21-de-novo-project-audit.md`  
**Status:** in progress — P0 integrated; remaining P1/P2 open.

| ID | Finding | Disposition | Landing |
|---|---|---|---|
| P0 | Builder ignores unmet flux while status forbids erosion | **fix** | `cmd_build` admission check via `audit::requisite_chain`; refuse exit 2; `--allow-unmet` provisional waiver + log line; lock `Drop` guard for setup-failure cleanup |
| P1 | Builder lock TOCTOU / non-atomic | **defer** | Documented in code comment; atomic `create_new` + token still owed |
| P1 | Query `put` errors swallowed | **defer** | Needs API shape change `io::Result`; separate PR |
| P1 | 64-bit FNV silent collision | **defer** | Named on `#form-complete-content-addressed-key`; hash upgrade not decided |
| P2 | decode_f32 no shape check | **defer** | Same complete-key / schema wave |

**Verify note:** Fresh world `build --level 7 --epochs 1` completes initial-topography then **REFUSED** on erosion with unmet `emerged land` (manual 2026-07-21). Level 5 + TILE_NX 64 still panics on sphere overflow (pre-existing; not this fix).
