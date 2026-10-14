> Versi: v1.0.0 | Status: disetujui | Menggantikan: -

# 60-BLAST-RADIUS — titeny-backend-service

Mengacu: Titeny-TownHall v1.0.0 (`https://github.com/Coding-Skuy/Titeny-TownHall/tree/main/platform/versions/v1.0.0`), [`townhall-roadmap.md` bagian 2](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/townhall-roadmap.md) (baris Titeny: point of concern "11 event; MAPE H+1 <= 8%"; batas context "12 komoditas x 8 pasar; read-only ke DB sumber") dan [`KONTEKS-LINTAS-DIVISI.md` bagian 8-9](https://github.com/Coding-Skuy/ChefGenie-TownHall/blob/main/KONTEKS-LINTAS-DIVISI.md).

## Pernyataan read-only

Repo ini tidak menulis ke DB sumber. Titeny hanya membaca dari Lumbung, TitipO, dan Pasaree (lewat event/konektor SELECT); satu-satunya basis tulis adalah DB `titeny`.

## Yang BOLEH disentuh repo ini

Seluruh kode API Titeny, migrasi SQLx untuk DB `titeny`, `docs/api/`, tabel `event_masuk`, `prediksi_harga`, `prediksi_panen`, `prediksi_demand`, `rekomendasi`, `validasi_analis`, `tahan_prediksi`, `audit_aksi`, `umpan_balik`.

## Yang DILARANG

- Menambah string koneksi ke DB Lumbung, TitipO, atau Pasaree.
- INSERT/UPDATE/DELETE di luar DB `titeny`; memanggil API tulis divisi lain.
- Menambah `event_type` di luar 11 terkunci (ditolak `event_type_tidak_dikenal`).
- Mengubah kontrak tanpa bump versi + ADR baru (tambah field opsional = minor; ubah tipe/makna = mayor + dual-read 30 hari).

## Kontrak

- **Dikonsumsi**: Event 11 terkunci dari `titeny-data-pipeline` / sumber (lumbung, titipo, pasaree, cuaca, internal) via `POST /api/v1/event/masuk`; artefak dan baris prediksi dari `titeny-ai-models`.
- **Diproduksi**: API baca/tulis `FSD/30-kontrak.md` (`docs/api/openapi.yaml`) untuk `titeny-web` dan `titeny-app-kmp`; token JWT `aud=titeny`; log audit 2 tahun.
- Modul bersama `:shared:pantry-resep` adalah milik Pawonee dan tidak dipakai atau diubah oleh Titeny. `pedaree.stok.v1` adalah kontrak bilateral Pedaree-TitipO dan bukan bagian registry Titeny (11 event terkunci).

## Radius Dampak per Perubahan

| Perubahan | Dampak | Batas penahan |
|---|---|---|
| Skema `event_masuk` | Ingest dan ringkasan ikut berubah | Migrasi sqlx reversibel, staging dulu |
| Logika dedup | Risiko baris ganda bila salah | Uji kirim ulang wajib lolos sebelum rilis |
| Endpoint insight | Web dan desktop analis ikut terdampak | Versi respons dijaga kompatibel |
| JWT `aud=titeny` | Token salah audiens ditolak | Audiens hanya `titeny`, tidak melebar |
| Koneksi DB | Salah URL mematikan layanan | Secret per lingkungan, health check |

## Rollback

Setiap rilis menyimpan tag image sebelumnya. Rollback berarti mengembalikan image dan migrasi DOWN yang sudah disiapkan, lalu memverifikasi `GET /api/v1/kesehatan` dan satu `POST /api/v1/event/masuk` duplikat tetap duplikat.

## Batasan

Dokumen ini hanya mengatur radius repo `titeny-backend-service`. Dampak repo lain ada di dokumen blast-radius repo tersebut. Insiden DB sumber diteruskan ke pemilik sumber.
