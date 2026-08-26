# Security Advisories

The following advisories have been reviewed and cannot be resolved at this time.

## RUSTSEC-2026-0258 — h2: unbounded empty DATA frames

- **Crate**: h2 0.3.27
- **Path**: rocket → rocket_http → hyper 0.14 → h2 0.3
- **Severity**: unrated (filed 2026-08-17)
- **Fix**: requires h2 >=0.4.16, which is incompatible with rocket 0.5 (uses hyper 0.14)
- **Impact**: potential DoS via memory exhaustion from empty HTTP/2 DATA frames
- **Mitigation**: the backend is not exposed to untrusted HTTP/2 traffic via public endpoints that would trigger this; a reverse proxy (e.g. nginx) sits in front. Monitor for a rocket 0.6 release that adopts hyper 1.x.

## RUSTSEC-2023-0071 — rsa: Marvin Attack timing side-channel

- **Crate**: rsa 0.9.10
- **Path**: sqlx → sqlx-mysql → rsa (MySQL TLS)
- **Severity**: 5.9 (medium)
- **Fix**: no fixed version is available per the advisory
- **Impact**: potential RSA private key recovery via timing side-channel during MySQL TLS handshake
- **Mitigation**: this project uses PostgreSQL (Supabase), not MySQL. sqlx-mysql is compiled in as a transitive feature dependency but no MySQL connections are made. Risk is effectively zero.
