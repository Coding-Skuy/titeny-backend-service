# titeny-backend-service

Layanan backend **inti** divisi **Titeny (AI Insight)**, org **Coding-Skuy**. Bahasa **Rust**, menyediakan API insight dengan basis data `titeny`.

Ekosistem: [Titeny-TownHall](https://github.com/Coding-Skuy/Titeny-TownHall).

## Prasyarat (versi dipin)

- Rust 1.83.0
- axum 0.7.9
- tokio 1.42.0
- sqlx 0.8.3 (postgres, runtime tokio-rustls)
- serde 1.0.215, serde_json 1.0.133
- PostgreSQL 16.4 (basis data bernama `titeny`)

## Cara jalan

```sh
cp .env.example .env
sqlx migrate run
cargo run
```

Layanan hidup di `http://localhost:8080`.

## Titik akhir

- `GET /health` — cek sehat, mengembalikan `ok`
- `GET /insight/ringkas` — ringkasan insight terbaru
- `GET /insight/harga?komoditas=cabai` — prediksi harga dari `titeny-ai-models`
- `POST /ingest/event` — menerima 1 dari 11 event baku dari `titeny-data-pipeline`

## Dokumentasi API

Kontrak OpenAPI 3.1 v1.0.0 ada di [`docs/api/openapi.yaml`](docs/api/openapi.yaml) dengan tampilan Scalar di [`docs/api/index.html`](docs/api/index.html).

- Online (GitHub Pages): https://coding-skuy.github.io/titeny-backend-service/api/
- Lokal: `bun x serve docs/api` lalu buka alamat yang tampil.
- Validasi: `bun x @redocly/cli@latest lint docs/api/openapi.yaml`

Endpoint wajib: `POST /api/v1/event/masuk` (header `X-Idempotency-Key`, retry 3x 5/30/300 detik lalu DLQ). Perubahan kontrak hanya lewat bump versi + ADR baru di `docs/decisions/`.

## Struktur

```text
src/main.rs     # titik masuk axum
src/insight.rs  # API insight
src/db.rs       # koneksi basis data titeny
migrations/     # skema SQL
```
