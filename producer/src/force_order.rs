// This is a auto-generated serde schema for forceOrder event from Binance Futures Websocket

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceOrder {
    pub e: String,
    #[serde(rename = "E")]
    pub e2: i64,
    pub o: O,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct O {
    pub s: String,
    #[serde(rename = "S")]
    pub s2: String,
    pub o: String,
    pub f: String,
    pub q: String,
    pub p: String,
    pub ap: String,
    #[serde(rename = "X")]
    pub x: String,
    pub l: String,
    pub z: String,
    #[serde(rename = "T")]
    pub t: i64,
}
