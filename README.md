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

## Struktur

```text
src/main.rs     # titik masuk axum
src/insight.rs  # API insight
src/db.rs       # koneksi basis data titeny
migrations/     # skema SQL
```
