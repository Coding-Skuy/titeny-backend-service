# ADR-0001 — Fondasi teknis titeny-backend-service

- **Status**: Diterima
- **Tanggal**: 2026-10-14

## Konteks

Backend inti Titeny: pintu masuk 11 event idempoten, API baca prediksi/rekomendasi, aksi analis, auth JWT `aud=titeny`, dan satu-satunya penulis DB `titeny`. Titeny (epic E4 Otak Data) mengonsumsi 11 event terkunci dari Lumbung, TitipO, Pasaree, cuaca, dan kalender internal, lalu menayangkan prediksi harga 12 komoditas x 8 pasar, panen 6 komoditas x 5 kabupaten, dan demand H+7/H+30. Dokumen formal v1.0.0 dibekukan di Titeny-TownHall pada 2026-10-07; repo ini harus mengikutinya tanpa menduplikasi requirement.

## Keputusan

1. **Stack**: Rust + Axum 0.8, SQLx Postgres, JWT `aud=titeny`; DB logis `titeny`; OpenAPI 3.1 + Scalar. Detail di `versions/v1.0.0/docs/40-TECHSTACK.md`.
2. **DB per service**: satu-satunya basis tulis Titeny adalah DB logis `titeny`; Titeny read-only terhadap DB Lumbung, TitipO, dan Pasaree (tidak ada INSERT/UPDATE/DELETE di luar `titeny`).
3. **JWT audiens divisi**: token `aud=titeny`, `iss=chefgenie-auth`; PIN zona 6 digit untuk petani/vendor baca via web; service key least-privilege.
4. **Registry event terkunci 11** (`harga.diperbarui`, `stok.berubah`, `stok.snapshot_harian`, `transaksi.dibuat`, `lapak.status_berubah`, `panen.dicatat`, `logistik.biaya_diperbarui`, `vendor.status_berubah`, `cuaca.diperbarui`, `kalender.dikunci`, `keuangan.ringkasan_harian`); amplop wajib `event_id` `evt_YYYYMMDD_NNNNNN`, `schema_version`, `source` dalam {lumbung, titipo, pasaree, cuaca, internal}, `occurred_at`/`received_at`, `batch_id`. `pedaree.stok.v1` bukan bagian registry.
5. **Idempotensi dan pengiriman**: `POST /api/v1/event/masuk` dengan header `X-Idempotency-Key`; retry 3x (5/30/300 detik) lalu DLQ; duplikat dijawab `200 duplikat_diabaikan`.
- **Tulis hanya DB `titeny`**; sumber dibaca lewat event/agregat, tidak pernah lewat koneksi langsung ke DB sumber.
- **Idempotensi** adalah kontrak: semua POST tulis membawa `event_id` atau `aksi_id`; kirim ulang kembali 200 duplikat tanpa rekaman ganda.
- **Sesi**: akses 15 menit + refresh 7 hari (reuse-detection); umur sesi analis dibatasi 8 jam dan refresh via OTP sesuai FSD/30.
6. **Perubahan kontrak/arsitektur** hanya lewat ADR baru dan bump versi; requirement berubah = PR di TownHall lebih dulu.

## Konsekuensi

- Pengembangan AI wajib membaca `AGENTS.md` dan ADR sebelum coding; deviasi dari keputusan di atas membutuhkan ADR baru (ADR ini tidak diedit).
- Kode awal yang masih menyimpang dari stack/kontrak (lihat bagian "Kewajiban Selaras Kode" di `40-TECHSTACK.md`) wajib dimigrasikan sebelum rilis v1.0.0.
- Kepatuhan diaudit melalui QA gate (TDD merah-hijau) dan audit 30 hari tanpa tulis ke DB sumber.
