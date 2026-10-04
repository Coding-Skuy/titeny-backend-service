use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Kartu { pub judul: String, pub nilai: String }
#[derive(Serialize)]
pub struct Ringkas { pub kartu: Vec<Kartu> }

pub async fn ringkas(State(pool): State<PgPool>) -> Json<Ringkas> {
    let jumlah: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM event_masuk")
        .fetch_one(&pool).await.unwrap_or((0,));
    Json(Ringkas { kartu: vec![
        Kartu { judul: "Event masuk".into(), nilai: jumlah.0.to_string() },
        Kartu { judul: "Basis data".into(), nilai: "titeny".into() },
        Kartu { judul: "Model".into(), nilai: "harga + panen".into() },
    ]})
}

pub async fn harga(Query(q): Query<HashMap<String, String>>) -> Json<serde_json::Value> {
    let komoditas = q.get("komoditas").cloned().unwrap_or_else(|| "cabai".into());
    Json(serde_json::json!({"komoditas": komoditas, "prediksi_7_hari": [10000, 10200, 10150], "sumber": "titeny-ai-models"}))
}

#[derive(Deserialize)]
pub struct EventMasuk { pub nama: String, pub muatan: serde_json::Value }

pub async fn terima_event(State(pool): State<PgPool>, Json(e): Json<EventMasuk>) -> Json<serde_json::Value> {
    sqlx::query("INSERT INTO event_masuk (nama, muatan) VALUES ($1, $2)")
        .bind(&e.nama).bind(&e.muatan).execute(&pool).await.unwrap();
    Json(serde_json::json!({"status": "diterima", "nama": e.nama}))
}
