> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 40-TECHSTACK — titeny-backend-service

Mengacu: Titeny-TownHall v1.0.0 (https://github.com/Coding-Skuy/Titeny-TownHall/tree/main/versions/v1.0.0).

## Konteks TownHall

Stack divisi dikunci di TownHall v1.0.0: web Bun latest + Svelte 5 + SvelteKit Kit 2 + TypeScript latest; analis-desktop KMP Windows; backend Rust axum 0.8.4; model dan pipeline Python 3.12. Repo ini adalah backend Rust axum 0.8.4.

## Stack Repo Ini

| Lapisan | Pilihan | Versi target | STATUS |
|---|---|---|---|
| Bahasa | Rust | 1.82+ stabil | defined |
| HTTP | axum | 0.8.4 (kode kini 0.7.9 → target 0.8.4) | defined |
| Async | tokio full | 1.42+ | defined |
| DB driver | sqlx postgres + rustls | 0.8.x | defined |
| Migrasi | sqlx migrate | 0.8.x | defined |
| Basis data | PostgreSQL, database `titeny` | 16+ | defined |
| Observabilitas | tracing + tracing-subscriber JSON | 0.1 / 0.3 | defined |
| Auth | JWT `aud=titeny`, `iss=chefgenie-auth` | — | defined |
| Kontainer | `ghcr.io/coding-skuy/titeny-backend-service` | 0.1.0 | defined |
| Uji | cargo test + uji idempotensi ingest | — | defined |

`DATABASE_URL` bawaan `postgres://postgres:postgres@localhost:5432/titeny`. Rahasia produksi hanya lewat secret manager, tidak di-commit.

## Batasan

Batasan dokumen ini: hanya stack backend. Dilarang menambah dependensi yang membuka koneksi ke DB selain `titeny`. Dilarang menurunkan axum di bawah 0.8.4 setelah migrasi tanpa persetujuan TownHall. Semua endpoint baru memakai `State<PgPool>` yang sama dan format galat baku.
