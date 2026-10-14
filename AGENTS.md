# AGENTS.md — titeny-backend-service

Backend inti Titeny: pintu masuk 11 event idempoten, API baca prediksi/rekomendasi, aksi analis, auth JWT `aud=titeny`, dan satu-satunya penulis DB `titeny`.

## Sumber kebenaran

Semua requirement bersumber dari [Titeny-TownHall](https://github.com/Coding-Skuy/Titeny-TownHall) versi beku `v1.0.0`. Dilarang menduplikasi requirement di repo kode; requirement berubah = PR di TownHall dulu.

| Dokumen (Titeny-TownHall) | Isi |
|---|---|
| [`platform/versions/v1.0.0/FSD/30-kontrak.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/FSD/30-kontrak.md) | Kontrak API, event, galat, stack pengikat |
| [`platform/versions/v1.0.0/FSD/20-model-data.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/FSD/20-model-data.md) | Entitas, enum, aturan angka, retensi |
| [`platform/versions/v1.0.0/FSD/10-alur.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/FSD/10-alur.md) | Urutan ingest/hitung/tayang, jadwal job, urutan layar |
| [`data/20-kontrak-event.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/data/20-kontrak-event.md) | Kontrak 11 event terkunci + amplop (sumber utama) |
| [`produk/21-kontrak-api-web-bun.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/produk/21-kontrak-api-web-bun.md) | Kontrak API web |
| [`produk/22-kontrak-desktop-windows.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/produk/22-kontrak-desktop-windows.md) | Kontrak desktop analis Windows |
| [`platform/versions/v1.0.0/BRD/00-ikhtisar.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/BRD/00-ikhtisar.md) | BR-001..BR-007: cakupan, read-only, JWT, bahasa |
| [`platform/versions/v1.0.0/FRD/10-fungsional.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/FRD/10-fungsional.md) | FR-001..FR-303 |
| [`platform/versions/v1.0.0/SRS/30-non-functional.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/SRS/30-non-functional.md) | Persyaratan non-fungsional |
| [`platform/versions/v1.0.0/SRS/40-acceptance-criteria.md`](https://github.com/Coding-Skuy/Titeny-TownHall/blob/main/platform/versions/v1.0.0/SRS/40-acceptance-criteria.md) | QA-001..QA-004 (QA gate) |
| [`KONTEKS-LINTAS-DIVISI.md`](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/KONTEKS-LINTAS-DIVISI.md) (ChefGenie-TownHall) | Event registry (bagian 3), satuan/ID/uang (bagian 6), metrik lintas-divisi (bagian 7) |
| [`townhall-roadmap.md`](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/townhall-roadmap.md) (ChefGenie-TownHall) | Epic E4 #5, batas blast radius (bagian 2), definisi selesai (bagian 6) |

Rujukan versi: `https://github.com/Coding-Skuy/Titeny-TownHall/tree/main/platform/versions/v1.0.0`.

## Tech stack terkunci

- **Rust + Axum 0.8.4** (path parameter `{id}`, bukan `:id`), Tokio, **SQLx** Postgres + `migrate!`, tower-http (Trace + Timeout), tracing, uuid v7, validator, jsonwebtoken + argon2.
- `AppState { db: PgPool }`; satu `AppError: IntoResponse` dengan amplop galat baku; uji `#[sqlx::test]`.
- Database logis `titeny` (PostgreSQL 16+); JWT akses 15 menit audiens `titeny` + refresh 7 hari dengan reuse-detection; service key least-privilege untuk sumber event; PIN zona 6 digit untuk petani/vendor.
- Kode kini axum 0.7.9, endpoint lama `/ingest/event` dan `/insight/*`: wajib dimigrasikan ke `/api/v1/*` sesuai `FSD/30-kontrak.md`.
- **Dilarang**: koneksi ke DB selain `titeny`; INSERT/UPDATE/DELETE di luar DB `titeny`; `unwrap()` pada jalur request; ORM selain SQLx.

Detail matriks: `versions/v1.0.0/docs/40-TECHSTACK.md`. Batas context: `versions/v1.0.0/docs/60-BLAST-RADIUS.md`.

## Aturan kerja AI

1. Baca `AGENTS.md` ini dan semua ADR di `docs/decisions/` sebelum coding.
2. TDD merah-hijau: tulis test yang gagal dulu; rilis diblokir bila QA gate gagal (QA-001..QA-004, tidak boleh skip test).
3. Kontrak API/event hanya diubah lewat bump versi + catatan di ADR baru; tidak ada `event_type` di luar 11 terkunci tanpa bump versi + dual-read 30 hari.
4. Commit memakai awalan `docs:` / `feat:` / `fix:` dan wajib memperbarui `CHANGELOG.md` repo.
5. Keputusan arsitektur baru = tambah ADR bernomor di `docs/decisions/`; jangan edit ADR lama (immutable, sama seperti aturan versi TownHall).
6. Dilarang menyentuh DB divisi lain: DB per service; Titeny read-only terhadap DB Lumbung, TitipO, dan Pasaree dan hanya menulis ke DB `titeny`.
7. Bahasa artefak: Indonesia; uang integer IDR; waktu ISO-8601 Asia/Jakarta; tanpa data pribadi di Titeny.

## Definisi selesai

Dari [`townhall-roadmap.md` bagian 6](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/townhall-roadmap.md):

- Dokumen versi lengkap (BRD/FRD/FSD/PRD/SRS) + `SNAPSHOT-ROADMAP.md` di TownHall; dokumen beku tidak diedit.
- QA gate berbasis SRS lulus (TDD merah-hijau, QA-001..QA-004); rilis diblokir bila gagal.
- Kontrak API terbit; audit keamanan bersih (JWT audiens `titeny` + DB per service).
- Bukti metrik lintas-divisi (KONTEKS-LINTAS-DIVISI.md bagian 7) tercatat: MAPE harga H+1 <= 8% median / H+7 <= 12%, demand H+7 <= 15%, panen <= 20%, cakupan P10-P90 70-90%.
- CHANGELOG repo dan CHANGELOG versi diperbarui.
