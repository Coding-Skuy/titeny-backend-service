> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 60-BLAST-RADIUS — titeny-backend-service

Mengacu: Titeny-TownHall v1.0.0 (https://github.com/Coding-Skuy/Titeny-TownHall/tree/main/versions/v1.0.0).

## Pernyataan read-only

Layanan ini read-only terhadap sumber dan tidak menulis ke DB sumber. Ia tidak pernah menulis ke DB Lumbung, TitipO, atau Pasaree. Satu-satunya basis data yang ditulis adalah DB `titeny` (tabel `event_masuk`, `prediksi_harga`, `prediksi_panen`). Kode hanya membuka satu pool ke `DATABASE_URL` milik `titeny`.

## Radius Dampak per Perubahan

| Perubahan | Dampak | Batas penahan |
|---|---|---|
| Skema `event_masuk` | Ingest dan ringkasan ikut berubah | Migrasi sqlx reversibel, staging dulu |
| Logika dedup | Risiko baris ganda bila salah | Uji kirim ulang wajib lolos sebelum rilis |
| Endpoint insight | Web dan desktop analis ikut terdampak | Versi respons dijaga kompatibel |
| JWT `aud=titeny` | Token salah audiens ditolak | Audiens hanya `titeny`, tidak melebar |
| Koneksi DB | Salah URL mematikan layanan | Secret per lingkungan, health check |

## Larangan Eksplisit

- Dilarang menambah string koneksi ke DB sumber.
- Dilarang menjalankan INSERT/UPDATE/DELETE di luar DB `titeny`.
- Dilarang memanggil API tulis milik Lumbung, TitipO, atau Pasaree.

## Rollback

Setiap rilis menyimpan tag image sebelumnya. Rollback berarti mengembalikan image dan migrasi DOWN yang sudah disiapkan, lalu memverifikasi `GET /health` dan satu `POST /ingest/event` duplikat tetap duplikat.

## Batasan

Batasan dokumen ini: hanya radius repo backend. Dampak model ada di `titeny-ai-models`, dampak kiriman ada di `titeny-data-pipeline`. Insiden DB sumber diteruskan ke pemilik sumber.
