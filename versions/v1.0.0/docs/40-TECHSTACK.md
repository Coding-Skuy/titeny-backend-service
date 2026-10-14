> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 40-TECHSTACK — titeny-backend-service

Mengacu: Titeny-TownHall v1.0.0 (`https://github.com/Coding-Skuy/Titeny-TownHall/tree/main/platform/versions/v1.0.0`). Stack pengikat divisi: [`FSD/30-kontrak.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/FSD/30-kontrak.md).

## Konteks TownHall

Stack divisi dikunci di TownHall v1.0.0: web Bun + Svelte 5 + SvelteKit 2 + TypeScript; analis-desktop KMP Windows (Kotlin, Compose Multiplatform, Navigation3); backend Rust Axum 0.8; model dan pipeline Python 3.12. Repo ini: Backend inti Titeny: pintu masuk 11 event idempoten, API baca prediksi/rekomendasi, aksi analis, auth JWT `aud=titeny`, dan satu-satunya penulis DB `titeny`.

## Stack Repo Ini

| Lapisan | Pilihan | Versi target | STATUS |
|---|---|---|---|
| Bahasa | Rust | 1.83.0 (dipin README) | defined |
| HTTP | axum, path `{id}` | 0.8.4 (kode kini 0.7.9) | defined |
| Async | tokio full | 1.42+ | defined |
| DB driver | sqlx postgres + rustls + `migrate!` | 0.8.x | defined |
| Middleware | tower-http Trace + Timeout | sesuai axum 0.8 | defined |
| Observabilitas | tracing + tracing-subscriber JSON | 0.1 / 0.3 | defined |
| ID | uuid v7 | - | defined |
| Validasi | validator | - | defined |
| Auth | jsonwebtoken (`aud=titeny`, `iss=chefgenie-auth`, ed25519) + argon2 | akses 15 mnt, refresh 7 hari reuse-detection | defined |
| Basis data | PostgreSQL, database logis `titeny` | 16+ | defined |
| State/Galat | `AppState { db: PgPool }`, satu `AppError: IntoResponse` | - | defined |
| Uji | `cargo test`, `#[sqlx::test]` (idempotensi ingest) | - | defined |
| Dokumentasi API | OpenAPI 3.1 `docs/api/openapi.yaml` + Scalar | 1.0.0 | defined |
| Kontainer | `ghcr.io/coding-skuy/titeny-backend-service` | 0.1.0 (kode kini) | defined |

## Perintah

```bash
cp .env.example .env
sqlx migrate run
cargo test
cargo run          # http://localhost:8080
bun x serve docs/api   # dokumentasi API lokal
```

## Kewajiban Selaras Kode (migrasi)

| Butir | Kondisi kode | Target v1.0.0 |
|---|---|---|
| axum | 0.7.9, `route("/ingest/event")` | 0.8.4, `POST /api/v1/event/masuk` + header `X-Idempotency-Key` |
| Idempotensi | `INSERT` tanpa kunci unik | Kunci per tipe event (FSD/20), duplikat 200 `duplikat_diabaikan` |
| Galat | `unwrap()` di handler | `AppError: IntoResponse`, kode baku `tidak_ditemukan`... `kesalahan_server` |
| Endpoint | `/insight/ringkas`, `/insight/harga` | `/api/v1/harga`, `/demand`, `/panen`, `/rekomendasi`, `/kesehatan` |
| Auth | Belum ada | JWT `aud=titeny` + PIN zona + rate limit 60 req/menit/token |

## Konfigurasi

`DATABASE_URL` (DB `titeny`), kunci JWT ed25519, token layanan sumber event. Semua lewat secret manager; tidak di-commit. Bodi maks 256 KB; paginasi `limit` default 50 maks 200 dengan cursor opaque; cache prediksi 6 jam (`max-age=21600`), `/kesehatan` 30 detik; SLO p95 < 400 ms (cache-hit), p99 < 1.200 ms.

## Batasan

Dokumen ini hanya mengatur stack repo `titeny-backend-service`. Perubahan versi pustaka atau stack dicatat sebagai ADR baru di `docs/decisions/` dan dilaporkan ke TownHall. Titeny read-only terhadap DB sumber; satu-satunya basis tulis adalah DB `titeny`.
