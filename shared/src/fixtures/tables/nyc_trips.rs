use serde::{Deserialize, Serialize};
use soa_derive::StructOfArray;
use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow, StructOfArray, Default, Serialize, Deserialize)]
pub struct NycTripsTable {
    #[sqlx(rename = "VendorID", default)]
    #[serde(rename = "VendorID")]
    pub vendor_id: Option<i32>,
    // For now, we're commenting out the datetime fields because they are presenting
    // a problem when serializing to parquet with serde-arrow. While these fields
    // do exist in the nyc_trips Postgres table that we create, we'll entirely
    // skip reading them into Rust with sqlx.
    // pub tpep_pickup_datetime: Option<PrimitiveDateTime>,
    // pub tpep_dropoff_datetime: Option<PrimitiveDateTime>,
    pub passenger_count: Option<i64>,
    pub trip_distance: Option<f64>,
    #[sqlx(rename = "RatecodeID", default)]
    #[serde(rename = "RatecodeID")]
    pub ratecode_id: Option<f64>,
    #[sqlx(rename = "store_and_fwd_flag", default)]
    #[serde(rename = "store_and_fwd_flag")]
    pub store_and_fwd_flag: Option<String>,
    #[sqlx(rename = "PULocationID", default)]
    #[serde(rename = "PULocationID")]
    pub pu_location_id: Option<f32>,
    #[sqlx(rename = "DOLocationID", default)]
    #[serde(rename = "DOLocationID")]
    pub do_location_id: Option<f32>,
    pub payment_type: Option<f64>,
    pub fare_amount: Option<f64>,
    pub extra: Option<f64>,
    pub mta_tax: Option<f64>,
    pub tip_amount: Option<f64>,
    pub tolls_amount: Option<f64>,
    pub improvement_surcharge: Option<f64>,
    pub total_amount: Option<f64>,
}

impl NycTripsTable {
    pub fn setup() -> String {
        NYC_TRIPS_TABLE_SETUP.to_string()
    }

    pub fn setup_fdw(s3_endpoint: &str, s3_object_path: &str) -> String {
        format!(
            r#"
        CREATE FOREIGN DATA WRAPPER s3_wrapper HANDLER s3_fdw_handler VALIDATOR s3_fdw_validator;
        
        CREATE SERVER s3_server FOREIGN DATA WRAPPER s3_wrapper
        OPTIONS (region 'us-east-1', allow_anonymous 'true', endpoint '{s3_endpoint}');
        
        CREATE FOREIGN TABLE trips (
            "VendorID"              INT,
            -- Commented out until serde-arrow serialization issue is addressed.
            -- "tpep_pickup_datetime"  TIMESTAMP,
            -- "tpep_dropoff_datetime" TIMESTAMP,
            "passenger_count"       BIGINT,
            "trip_distance"         DOUBLE PRECISION,
            "RatecodeID"            DOUBLE PRECISION,
            "store_and_fwd_flag"    TEXT,
            "PULocationID"          REAL,
            "DOLocationID"          REAL,
            "payment_type"          DOUBLE PRECISION,
            "fare_amount"           DOUBLE PRECISION,
            "extra"                 DOUBLE PRECISION,
            "mta_tax"               DOUBLE PRECISION,
            "tip_amount"            DOUBLE PRECISION,
            "tolls_amount"          DOUBLE PRECISION,
            "improvement_surcharge" DOUBLE PRECISION,
            "total_amount"          DOUBLE PRECISION
        )
        SERVER s3_server
        OPTIONS (path '{s3_object_path}', extension 'parquet'); 
    "#
        )
    }
}

static NYC_TRIPS_TABLE_SETUP: &str = r#"
CREATE TABLE nyc_trips (
    "VendorID"              INT,
    "tpep_pickup_datetime"  TIMESTAMP,
    "tpep_dropoff_datetime" TIMESTAMP,
    "passenger_count"       BIGINT,
    "trip_distance"         DOUBLE PRECISION,
    "RatecodeID"            DOUBLE PRECISION,
    "store_and_fwd_flag"    TEXT,
    "PULocationID"          REAL,
    "DOLocationID"          REAL,
    "payment_type"          DOUBLE PRECISION,
    "fare_amount"           DOUBLE PRECISION,
    "extra"                 DOUBLE PRECISION,
    "mta_tax"               DOUBLE PRECISION,
    "tip_amount"            DOUBLE PRECISION,
    "tolls_amount"          DOUBLE PRECISION,
    "improvement_surcharge" DOUBLE PRECISION,
    "total_amount"          DOUBLE PRECISION
);

INSERT INTO nyc_trips ("VendorID", tpep_pickup_datetime, tpep_dropoff_datetime, passenger_count, trip_distance, "RatecodeID", store_and_fwd_flag, "PULocationID", "DOLocationID", payment_type, fare_amount, extra, mta_tax, tip_amount, tolls_amount, improvement_surcharge, total_amount)
VALUES
(2, '2024-01-24 15:17:12', '2024-01-24 15:34:53', 1, 3.33, 1, 'N', 239, 246, 1, 20.5, 0, 0.5, 3, 0, 1, 27.5),
(2, '2024-01-24 15:52:24', '2024-01-24 16:01:39', 1, 1.61, 1, 'N', 234, 249, 1, 10.7, 0, 0.5, 3.67, 0, 1, 18.37),
(2, '2024-01-24 15:08:55', '2024-01-24 15:31:35', 1, 4.38, 1, 'N', 88, 211, 1, 25.4, 0, 0.5, 5.88, 0, 1, 35.28),
(2, '2024-01-24 15:42:55', '2024-01-24 15:51:35', 1, 0.95, 1, 'N', 211, 234, 1, 9.3, 0, 0.5, 2.66, 0, 1, 15.96),
(2, '2024-01-24 15:52:23', '2024-01-24 16:12:53', 1, 2.58, 1, 'N', 68, 144, 1, 18.4, 0, 0.5, 4.48, 0, 1, 26.88),
(1, '2024-01-24 15:30:55', '2024-01-24 16:38:46', 1, 15.8, 2, 'N', 164, 132, 1, 70, 2.5, 0.5, 10, 6.94, 1, 90.94),
(2, '2024-01-24 15:21:48', '2024-01-24 15:59:06', 2, 7.69, 1, 'N', 231, 161, 1, 40.8, 0, 0.5, 6.5, 0, 1, 51.3),
(2, '2024-01-24 15:47:59', '2024-01-24 16:12:38', 1, 8.31, 1, 'N', 138, 262, 1, 35.2, 5, 0.5, 10, 6.94, 1, 62.89),
(2, '2024-01-24 15:55:32', '2024-01-24 16:23:01', 1, 8.47, 1, 'N', 132, 192, 2, 36.6, 0, 0.5, 0, 0, 1, 39.85),
(1, '2024-01-24 15:02:22', '2024-01-24 15:13:11', 1, 1.4, 1, 'N', 226, 7, 2, 11.4, 0, 0.5, 0, 0, 1, 12.9),
(1, '2024-01-24 15:49:04', '2024-01-24 15:55:15', 1, 0.9, 1, 'N', 43, 237, 1, 7.9, 5, 0.5, 2.85, 0, 1, 17.25),
(2, '2024-01-24 15:10:53', '2024-01-24 15:20:45', 1, 0.55, 1, 'N', 237, 237, 1, 10, 0, 0.5, 2.8, 0, 1, 16.8),
(1, '2024-01-24 15:09:28', '2024-01-24 16:21:23', 1, 16.2, 5, 'N', 230, 132, 1, 86.55, 0, 0, 17.5, 0, 1, 105.05),
(2, '2024-01-24 15:14:11', '2024-01-24 15:27:17', 1, 0.74, 1, 'N', 236, 237, 2, 12.1, 0, 0.5, 0, 0, 1, 16.1),
(2, '2024-01-24 15:56:34', '2024-01-24 16:27:32', 1, 3.79, 1, 'N', 230, 144, 1, 27.5, 0, 0.5, 7.88, 0, 1, 39.38),
(2, '2024-01-24 15:31:32', '2024-01-24 15:46:48', 2, 1.9, 1, 'N', 246, 161, 1, 14.9, 0, 0.5, 3.78, 0, 1, 22.68),
(2, '2024-01-24 15:50:45', '2024-01-24 16:22:14', 1, 6.82, 1, 'N', 162, 261, 1, 33.8, 0, 0.5, 3.78, 0, 1, 41.58),
(2, '2024-01-24 15:54:18', '2024-01-24 16:24:41', 1, 8.26, 1, 'N', 138, 262, 1, 37.3, 5, 0.5, 10.65, 6.94, 1, 65.64),
(2, '2024-01-24 15:11:02', '2024-01-24 15:33:35', 1, 1.6, 1, 'N', 162, 263, 1, 19.1, 0, 0.5, 4.62, 0, 1, 27.72),
(2, '2024-01-24 15:20:01', '2024-01-24 15:34:38', 2, 1.79, 1, 'N', 68, 163, 2, 14.2, 0, 0.5, 0, 0, 1, 18.2),
(2, '2024-01-24 15:50:36', '2024-01-24 15:59:20', 1, 0.58, 1, 'N', 162, 229, 1, 9.3, 0, 0.5, 3.33, 0, 1, 16.63),
(1, '2024-01-24 15:04:08', '2024-01-24 15:23:57', 1, 2, 1, 'N', 246, 161, 1, 14.9, 2.5, 0.5, 1, 0, 1, 19.9),
(1, '2024-01-24 15:25:27', '2024-01-24 15:37:29', 1, 1.6, 1, 'N', 161, 233, 1, 10.7, 2.5, 0.5, 3.65, 0, 1, 18.35),
(1, '2024-01-24 15:40:53', '2024-01-24 15:45:56', 1, 1.1, 1, 'Y', 233, 162, 1, 7.2, 2.5, 0.5, 2.24, 0, 1, 13.44),
(1, '2024-01-24 15:56:09', '2024-01-24 16:05:35', 1, 1.6, 1, 'N', 237, 239, 1, 10, 2.5, 0.5, 4.2, 0, 1, 18.2),
(2, '2024-01-24 15:03:07', '2024-01-24 15:21:19', 2, 5.73, 5, 'N', 180, 132, 1, 84, 0, 0, 17, 0, 1, 102),
(2, '2024-01-24 16:02:45', '2024-01-24 16:11:52', 1, 1.1, 1, 'N', 263, 141, 1, 10, 0, 0.5, 2.1, 0, 1, 16.1),
(2, '2024-01-24 15:19:51', '2024-01-24 15:30:56', 1, 0.77, 1, 'N', 162, 161, 1, 10.7, 0, 0.5, 2.94, 0, 1, 17.64),
(2, '2024-01-24 15:32:10', '2024-01-24 15:39:06', 1, 0.85, 1, 'N', 161, 170, 1, 7.9, 0, 0.5, 2.98, 0, 1, 14.88),
(2, '2024-01-24 15:44:04', '2024-01-24 15:56:43', 2, 1.07, 1, 'N', 170, 163, 1, 12.1, 0, 0.5, 3.22, 0, 1, 19.32),
(2, '2024-01-24 15:57:39', '2024-01-24 16:02:55', 1, 0.54, 1, 'N', 161, 237, 1, 6.5, 0, 0.5, 2.1, 0, 1, 12.6),
(1, '2024-01-24 15:04:50', '2024-01-24 15:25:58', 2, 2.9, 1, 'N', 161, 246, 1, 21.9, 2.5, 0.5, 5.15, 0, 1, 31.05),
(2, '2024-01-24 15:27:35', '2024-01-24 15:50:28', 1, 2.11, 1, 'N', 164, 79, 1, 20.5, 0, 0.5, 4.9, 0, 1, 29.4),
(2, '2024-01-24 15:13:53', '2024-01-24 15:55:09', 3, 5.62, 1, 'N', 161, 261, 1, 38, 0, 0.5, 8.4, 0, 1, 50.4),
(1, '2024-01-24 15:29:37', '2024-01-24 15:50:25', 1, 2.2, 1, 'N', 237, 230, 1, 18.4, 2.5, 0.5, 5.55, 0, 1, 27.95),
(1, '2024-01-24 15:34:29', '2024-01-24 15:45:41', 1, 2, 1, 'N', 142, 151, 1, 12.1, 2.5, 0.5, 3.22, 0, 1, 19.32),
(1, '2024-01-24 15:54:16', '2024-01-24 16:04:40', 2, 1.6, 1, 'N', 238, 143, 1, 10.7, 5, 0.5, 3.4, 0, 1, 20.6),
(2, '2024-01-24 15:05:20', '2024-01-24 15:16:38', 1, 1.27, 1, 'N', 142, 230, 2, 11.4, 0, 0.5, 0, 0, 1, 15.4),
(2, '2024-01-24 15:21:05', '2024-01-24 16:36:49', 1, 7.49, 1, 'N', 163, 181, 1, 61.8, 0, 0.5, 21.82, 6.94, 1, 94.56),
(2, '2024-01-24 15:13:19', '2024-01-24 15:28:32', 1, 2.51, 1, 'N', 143, 236, 1, 16.3, 0, 0.5, 4.06, 0, 1, 24.36),
(2, '2024-01-24 15:38:01', '2024-01-24 15:49:52', 1, 1.83, 1, 'N', 239, 262, 1, 12.8, 0, 0.5, 4.2, 0, 1, 21),
(2, '2024-01-24 15:09:19', '2024-01-24 15:26:41', 1, 2.42, 1, 'N', 238, 237, 1, 17, 0, 0.5, 4.2, 0, 1, 25.2),
(2, '2024-01-24 15:30:22', '2024-01-24 15:45:27', 1, 2.25, 1, 'N', 237, 233, 1, 15.6, 0, 0.5, 3.92, 0, 1, 23.52),
(1, '2024-01-24 15:57:50', '2024-01-24 16:45:02', 0, 15, 1, 'N', 138, 265, 2, 60.4, 9.25, 0.5, 0, 6.94, 1, 78.09),
(2, '2024-01-24 15:41:46', '2024-01-24 15:50:08', 1, 0.8, 1, 'N', 161, 100, 1, 8.6, 0, 0.5, 2.52, 0, 1, 15.12),
(2, '2024-01-24 15:54:22', '2024-01-24 15:59:06', 1, 0.5, 1, 'N', 100, 164, 2, 6.5, 0, 0.5, 0, 0, 1, 10.5),
(2, '2024-01-24 15:25:27', '2024-01-24 15:34:11', 2, 1.09, 1, 'N', 164, 234, 1, 9.3, 0, 0.5, 3.99, 0, 1, 17.29),
(2, '2024-01-24 15:14:18', '2024-01-24 15:22:17', 1, 0.78, 1, 'N', 234, 249, 1, 8.6, 0, 0.5, 2.52, 0, 1, 15.12),
(2, '2024-01-24 15:33:41', '2024-01-24 15:47:12', 1, 1.54, 1, 'N', 113, 231, 1, 12.8, 0, 0.5, 5.04, 0, 1, 21.84),
(2, '2024-01-24 15:53:15', '2024-01-24 16:04:11', 1, 1.63, 1, 'N', 125, 68, 1, 12.1, 0, 0.5, 2.42, 0, 1, 18.52),
(1, '2024-01-24 15:13:03', '2024-01-24 15:23:58', 1, 1.4, 1, 'N', 142, 161, 1, 10, 2.5, 0.5, 2.8, 0, 1, 16.8),
(1, '2024-01-24 15:31:49', '2024-01-24 15:46:47', 1, 1.8, 1, 'N', 161, 68, 1, 12.8, 2.5, 0.5, 3.36, 0, 1, 20.16),
(1, '2024-01-24 15:48:50', '2024-01-24 16:06:14', 1, 1.1, 1, 'N', 68, 246, 1, 12.1, 2.5, 0.5, 2, 0, 1, 18.1),
(2, '2024-01-24 15:17:46', '2024-01-24 15:28:19', 1, 1.02, 1, 'N', 236, 236, 1, 10.7, 0, 0.5, 3.67, 0, 1, 18.37),
(2, '2024-01-24 15:30:25', '2024-01-24 15:38:09', 1, 0.84, 1, 'N', 236, 141, 1, 8.6, 0, 0.5, 2.52, 0, 1, 15.12),
(2, '2024-01-24 15:47:13', '2024-01-24 15:50:30', 1, 0.54, 1, 'N', 237, 162, 1, 5.8, 0, 0.5, 2.45, 0, 1, 12.25),
(1, '2024-01-24 15:04:49', '2024-01-24 15:29:05', 1, 6.6, 1, 'N', 132, 134, 1, 27.5, 1.75, 0.5, 0, 0, 1, 30.75),
(1, '2024-01-24 15:52:43', '2024-01-24 16:48:43', 1, 16.3, 2, 'N', 132, 230, 1, 70, 4.25, 0.5, 15.15, 0, 1, 90.9),
(1, '2024-01-24 15:10:42', '2024-01-24 16:07:13', 1, 16.9, 2, 'N', 162, 132, 1, 70, 2.5, 0.5, 16.15, 6.94, 1, 97.09),
(1, '2024-01-24 15:24:26', '2024-01-24 15:53:43', 1, 3.1, 1, 'N', 236, 164, 2, 25.4, 2.5, 0.5, 0, 0, 1, 29.4),
(1, '2024-01-24 15:55:46', '2024-01-24 16:02:04', 1, 0.8, 1, 'N', 164, 107, 1, 7.9, 2.5, 0.5, 2.35, 0, 1, 14.25),
(1, '2024-01-24 15:57:50', '2024-01-24 16:21:27', 1, 2.9, 1, 'N', 75, 143, 1, 21.9, 5, 0.5, 5.65, 0, 1, 34.05),
(2, '2024-01-24 15:56:42', '2024-01-24 16:01:57', 1, 0.73, 1, 'N', 237, 162, 2, 7.2, 0, 0.5, 0, 0, 1, 11.2),
(2, '2024-01-24 15:02:26', '2024-01-24 15:14:20', 1, 1.41, 1, 'N', 151, 41, 2, 12.1, 0, 0.5, 0, 0, 1, 13.6),
(2, '2024-01-24 15:43:11', '2024-01-24 15:52:26', 1, 2.03, 1, 'N', 75, 239, 1, 12.1, 0, 0.5, 3.22, 0, 1, 19.32),
(1, '2024-01-24 15:09:57', '2024-01-24 15:17:06', 1, 0.9, 1, 'N', 186, 234, 1, 8.6, 2.5, 0.5, 1.5, 0, 1, 14.1),
(1, '2024-01-24 15:15:44', '2024-01-24 16:03:27', 1, 5.2, 1, 'N', 234, 41, 2, 40.8, 2.5, 0.5, 0, 0, 1, 44.8),
(2, '2024-01-24 15:03:30', '2024-01-24 15:15:18', 1, 1.74, 1, 'N', 142, 162, 1, 12.8, 0, 0.5, 3, 0, 1, 19.8),
(2, '2024-01-24 15:16:18', '2024-01-24 15:26:54', 1, 1.02, 1, 'N', 162, 230, 1, 10.7, 0, 0.5, 2.94, 0, 1, 17.64),
(1, '2024-01-24 15:09:12', '2024-01-24 15:26:06', 1, 2.5, 1, 'N', 163, 43, 2, 15.6, 2.5, 0.5, 0, 0, 1, 19.6),
(1, '2024-01-24 15:36:01', '2024-01-24 16:09:08', 1, 3.4, 1, 'N', 238, 164, 1, 26.8, 2.5, 0.5, 3.08, 0, 1, 33.88),
(1, '2024-01-24 15:01:40', '2024-01-24 15:30:58', 1, 4, 1, 'N', 231, 181, 1, 23.3, 2.5, 0.5, 6.85, 0, 1, 34.15),
(1, '2024-01-24 15:44:58', '2024-01-24 16:02:01', 1, 1, 1, 'N', 97, 33, 2, 13.5, 0, 0.5, 0, 0, 1, 15),
(1, '2024-01-24 15:08:08', '2024-01-24 15:19:26', 1, 1.1, 1, 'N', 262, 75, 2, 10.7, 2.5, 0.5, 0, 0, 1, 14.7),
(1, '2024-01-24 15:24:26', '2024-01-24 15:51:30', 1, 2.8, 1, 'N', 75, 48, 1, 21.9, 2.5, 0.5, 5.2, 0, 1, 31.1),
(1, '2024-01-24 15:05:32', '2024-01-24 16:11:42', 1, 8.1, 1, 'N', 186, 85, 2, 49.2, 2.5, 0.5, 0, 0, 1, 53.2),
(1, '2024-01-24 15:16:02', '2024-01-24 15:25:14', 1, 0.5, 1, 'N', 162, 161, 1, 9.3, 2.5, 0.5, 2.65, 0, 1, 15.95),
(1, '2024-01-24 15:29:34', '2024-01-24 15:34:45', 1, 0.3, 1, 'N', 161, 162, 2, 6.5, 2.5, 0.5, 0, 0, 1, 10.5),
(1, '2024-01-24 15:56:23', '2024-01-24 16:12:18', 1, 1.4, 1, 'N', 48, 164, 1, 14.9, 2.5, 0.5, 3.75, 0, 1, 22.65),
(1, '2024-01-24 15:22:06', '2024-01-24 15:46:23', 1, 4.4, 1, 'N', 68, 238, 1, 26.1, 2.5, 0.5, 7.5, 0, 1, 37.6),
(2, '2024-01-24 15:28:46', '2024-01-24 15:43:33', 1, 1.49, 1, 'N', 113, 186, 1, 13.5, 0, 0.5, 3.5, 0, 1, 21),
(2, '2024-01-24 15:49:11', '2024-01-24 16:03:14', 1, 1.49, 1, 'N', 90, 161, 1, 13.5, 0, 0.5, 3.5, 0, 1, 21),
(1, '2024-01-24 15:09:45', '2024-01-24 15:43:41', 1, 2.6, 1, 'N', 158, 170, 1, 28.2, 2.5, 0.5, 6.4, 0, 1, 38.6),
(2, '2024-01-24 15:10:12', '2024-01-24 15:30:12', 1, 2.64, 1, 'N', 186, 141, 1, 19.1, 0, 0.5, 2, 0, 1, 25.1),
(2, '2024-01-24 15:08:02', '2024-01-24 15:20:36', 1, 1.59, 1, 'N', 142, 161, 1, 13.5, 0, 0.5, 3.5, 0, 1, 21),
(2, '2024-01-24 15:54:25', '2024-01-24 16:25:45', 1, 3.55, 1, 'N', 236, 234, 1, 27.5, 0, 0.5, 6.3, 0, 1, 37.8),
(2, '2024-01-24 15:09:55', '2024-01-24 15:22:14', 1, 1.85, 1, 'N', 236, 143, 1, 13.5, 0, 0.5, 2, 0, 1, 19.5),
(2, '2024-01-24 15:33:37', '2024-01-24 15:39:20', 2, 0.59, 1, 'N', 238, 238, 1, 7.2, 0, 0.5, 2.24, 0, 1, 13.44),
(2, '2024-01-24 15:58:14', '2024-01-24 16:02:46', 2, 0.42, 1, 'N', 239, 142, 1, 5.8, 0, 0.5, 1.96, 0, 1, 11.76),
(2, '2024-01-24 15:05:34', '2024-01-24 15:51:33', 1, 11.54, 1, 'N', 138, 142, 1, 52, 5, 0.5, 13.94, 6.94, 1, 83.63),
(2, '2024-01-24 15:19:22', '2024-01-24 15:28:49', 1, 1.38, 1, 'N', 230, 143, 1, 10.7, 0, 0.5, 1.47, 0, 1, 16.17),
(2, '2024-01-24 15:22:30', '2024-01-24 15:47:17', 1, 3.6, 1, 'N', 163, 74, 2, 22.6, 0, 0.5, 0, 0, 1, 26.6),
(1, '2024-01-24 15:51:41', '2024-01-24 15:54:17', 1, 0.3, 1, 'N', 249, 90, 1, 4.4, 5, 0.5, 2, 0, 1, 12.9),
(2, '2024-01-24 15:02:26', '2024-01-24 15:07:59', 1, 0.66, 1, 'N', 161, 163, 1, 7.2, 0, 0.5, 2.24, 0, 1, 13.44),
(2, '2024-01-24 15:09:01', '2024-01-24 15:25:34', 1, 1.38, 1, 'N', 163, 236, 1, 14.9, 0, 0.5, 1, 0, 1, 19.9),
(1, '2024-01-24 15:06:58', '2024-01-24 15:24:35', 1, 1.4, 1, 'N', 236, 161, 1, 14.9, 2.5, 0.5, 3.8, 0, 1, 22.7),
(1, '2024-01-24 15:39:09', '2024-01-24 16:03:25', 1, 2.5, 1, 'N', 233, 68, 1, 19.8, 2.5, 0.5, 4.75, 0, 1, 28.55),
(2, '2024-01-24 15:21:47', '2024-01-24 15:55:15', 1, 8.65, 1, 'N', 70, 230, 2, 40.8, 5, 0.5, 0, 6.94, 1, 58.49),
(2, '2024-01-24 15:32:46', '2024-01-24 16:01:04', 1, 2.16, 1, 'N', 113, 79, 1, 23.3, 0, 0.5, 8.19, 0, 1, 35.49),
(2, '2024-01-24 15:37:00', '2024-01-24 16:01:28', 1, 4.56, 1, 'N', 261, 170, 1, 25.4, 0, 0.5, 5.88, 0, 1, 35.28);
"#;
