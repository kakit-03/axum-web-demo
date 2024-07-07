use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::util::deserialize_datetime;

#[derive(Debug, Clone, Deserialize, Default, Validate, Serialize)]
pub struct HeartParam {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    pub mac: Option<String>,
    pub ip: Option<String>,
    #[serde(rename = "personCount")]
    pub person_count: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Default, Validate, Serialize)]
pub struct DeviceResultParam {
    pub id: Option<String>,
    pub time: Option<String>,
    pub name: Option<String>,
    pub sex: Option<String>,
    pub nation: Option<String>,
    #[serde(rename = "detectScore")]
    pub detect_score: Option<String>,
    pub deviceno: Option<String>,
    #[serde(rename = "icNum")]
    pub ic_num: Option<String>,
    #[serde(rename = "idNum")]
    pub id_num: Option<String>,
    pub birthday: Option<String>,
    pub address: Option<String>,
    pub depart: Option<String>,
    #[serde(rename = "validStart")]
    pub valid_start: Option<String>,
    #[serde(rename = "validEnd")]
    pub valid_end: Option<String>,
    pub mac: Option<String>,
    pub inout: Option<i32>,
    #[serde(rename = "recordType")]
    pub record_type: Option<i32>,
    #[serde(rename = "resultCode")]
    pub result_code: Option<i32>,
    #[serde(rename = "openType")]
    pub open_type: Option<String>,
    #[serde(rename = "resultStatus")]
    pub result_status: Option<i32>,
    #[serde(rename = "scenePhoto")]
    pub scene_photo: Option<String>,
    #[serde(rename = "userPhoto")]
    pub user_photo: Option<String>,
    #[serde(rename = "Temp")]
    pub temp: Option<String>,
    #[serde(rename = "TempState")]
    pub temp_state: Option<String>,
    pub sn: Option<String>,
    #[serde(rename = "qrCode")]
    pub qr_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default, Validate, Serialize)]
pub struct QrCodeResult {
    #[serde(rename = "actionName")]
    pub action_name: Option<String>,
    pub data: Option<String>,
    pub deviceno: Option<String>,
    pub version: Option<String>,
}
