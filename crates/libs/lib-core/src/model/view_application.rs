use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ViewApplication {
    #[serde(rename = "0")]
    pub zero: Option<bool>,
    #[serde(rename = "1")]
    pub one: Option<bool>,
    #[serde(rename = "2")]
    pub two: Option<bool>,
    #[serde(rename = "3")]
    pub three: Option<bool>,
    #[serde(rename = "4")]
    pub four: Option<bool>,
    #[serde(rename = "5")]
    pub five: Option<bool>,
    #[serde(rename = "6")]
    pub six: Option<bool>,
    #[serde(rename = "7")]
    pub seven: Option<bool>,
    #[serde(rename = "8")]
    pub eight: Option<bool>,
}
