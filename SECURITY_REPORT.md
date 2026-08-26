# Security Report — Good Idea Randomizer
Date: 2026-08-26

## Summary
0 critical, 0 high, 0 medium (all resolved), 4 informational findings.

**APPROVED FOR DEPLOYMENT**

---

## Checklist Results

### A — Secrets & Credentials ✓
- No API keys, tokens, or passwords hardcoded in source. Test files use literal strings `"test-secret"` / `"correct-secret"` — these are test fixtures, not real credentials.
- `.env` is in `.gitignore`. Only `.env.template` (no real values) is committed.
- All secrets read from environment variables via `config.rs` (`Config::from_env()`).
- Dockerfile sets no secrets via `ENV`; docker-compose uses `${VAR}` expansion from host environment.

### B — Authentication & Authorisation ✓
- Per SPEC, this app has no auth requirement. No user accounts, no private data.
- `AuthUser` request guard is correctly implemented (validated by 4 unit tests): checks signature (HS256), expiry (`exp`), and audience (`authenticated`).
- The only backend route, `/health`, is intentionally public.
- No user-owned data means horizontal privilege escalation is not applicable.

### C — Input Validation & Injection ✓
- No user-supplied input reaches any backend route handler.
- `/health` accepts zero input parameters.
- All app logic runs client-side in WASM; no server-side rendering.
- No file uploads, no Stripe webhooks.

### D — Transport & Headers ✓ (after fix)
- HTTPS enforced by nginx reverse proxy with Let's Encrypt (`VIRTUAL_HOST` / `LETSENCRYPT_HOST` in docker-compose).
- No CORS configuration needed — app is served from same origin, no cross-origin API calls.
- **FIXED:** Security headers fairing added to Rocket (`main.rs`); all responses now include:
  - `X-Content-Type-Options: nosniff`
  - `X-Frame-Options: DENY`
  - `Referrer-Policy: strict-origin-when-cross-origin`
- HSTS already set by `nginx-proxy`.

### E — Dependency Audit ✓ (documented)
Two advisories found; both pre-documented in `SECURITY.md` with mitigations:
- `RUSTSEC-2026-0258` (h2 0.3.27, unscored): potential DoS via empty HTTP/2 DATA frames via `rocket → hyper 0.14 → h2`. Mitigation: nginx reverse proxy sits in front; cannot fix without replacing Rocket.
- `RUSTSEC-2023-0071` (rsa 0.9.10, medium 5.9): Marvin timing side-channel via `sqlx-mysql`. Mitigation: project uses PostgreSQL; no MySQL connections are made. Risk is effectively zero.

### F — Supabase RLS ✓
- No user tables exist. SPEC specifies zero state, no database schema.
- RLS is not applicable.

### G — Tauri ✓ (after fix)
- **FIXED:** `src-tauri/tauri.conf.json` CSP was `null` (disabled). Updated to `"default-src 'self'; style-src 'self' 'unsafe-inline'"`.
- `'unsafe-inline'` is restricted to styles only — acceptable for CSS in a WASM app where script execution comes from the WASM binary, not inline scripts.

---

## Findings

### [MEDIUM — RESOLVED] Missing HTTP Security Headers
**Location:** `crates/backend/src/main.rs`
**Description:** Rocket server responses lacked `X-Content-Type-Options`, `X-Frame-Options`, and `Referrer-Policy` headers.
**Risk:** MIME sniffing attacks, clickjacking, referrer leakage (low impact given no auth/sensitive data).
**Remediation:** Added `SecurityHeaders` Rocket fairing that injects all three headers on every response.
**Status:** RESOLVED — SecurityHeaders fairing added to main.rs.

### [MEDIUM — RESOLVED] Tauri CSP Disabled
**Location:** `crates/app/src-tauri/tauri.conf.json`
**Description:** `"csp": null` disabled Content Security Policy in the Tauri desktop app wrapper.
**Risk:** If the desktop app is built and distributed, a compromised dependency or injected script could execute without CSP restriction.
**Remediation:** Set CSP to `"default-src 'self'; style-src 'self' 'unsafe-inline'"` matching the project-specific Tauri config.
**Status:** RESOLVED — CSP updated in src-tauri/tauri.conf.json.

### [INFO] JWT Missing Issuer (`iss`) Check
**Location:** `crates/backend/src/auth.rs:46`
**Description:** `Validation` checks audience and expiry but does not validate the `iss` claim against the Supabase project URL.
**Risk:** A JWT signed with the same secret but issued by a different Supabase project would be accepted — relevant only if the same secret is shared across projects.
**Remediation:** Add `validation.set_issuer(&[config.supabase_url])` when protected routes are added. Not blocking for current deployment — no protected routes exist.
**Status:** OPEN (informational — no protected routes).

### [INFO] `format!()` in Schema SET Statement
**Location:** `crates/backend/src/db.rs:11`
**Description:** `SET search_path TO {schema}` built with `format!()`. The `schema` value comes from the `SUPABASE_SCHEMA` environment variable, not user input.
**Risk:** No practical SQL injection risk — schema is operator-controlled config, not user-supplied.
**Remediation:** Consider `SET search_path TO "$schema$"` quoting for defence-in-depth when routes that use user-supplied schema names are added.
**Status:** OPEN (informational — schema is trusted config, not user input).

### [INFO] h2 DoS Advisory (RUSTSEC-2026-0258)
**Location:** transitive via `rocket → hyper 0.14 → h2 0.3.27`
**Description:** Unbounded memory growth from empty HTTP/2 DATA frames.
**Risk:** Potential DoS if exposed to untrusted HTTP/2 traffic. Nginx reverse proxy in front absorbs this.
**Remediation:** Monitor for Rocket 0.6 release with hyper 1.x.
**Status:** OPEN (informational — mitigated by nginx).

### [INFO] RSA Timing Side-Channel (RUSTSEC-2023-0071)
**Location:** transitive via `sqlx-mysql → rsa 0.9.10`
**Description:** Marvin Attack — RSA private key recovery via timing side-channel.
**Risk:** Effectively zero — project uses PostgreSQL only; no MySQL connections are made.
**Remediation:** No upstream fix available. Remove if sqlx gains a feature flag to exclude mysql.
**Status:** OPEN (informational — MySQL not used).
