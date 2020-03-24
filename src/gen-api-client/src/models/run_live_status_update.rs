/*
 * JJS main API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RunLiveStatusUpdate : Represents Live Status Update  Some fields can be missing for various reasons, it is normal that particular object will look like {liveScore: null, currentTest: null, finish: false}. Information in all fields except `finish` can be inaccurate, incorrect or outdated. You can rely on following: if `finish` is true, final judging results are available

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunLiveStatusUpdate {
    /// Current running test
    #[serde(rename = "current_test")]
    pub current_test: Option<i32>,
    /// Whether final status is available
    #[serde(rename = "finish")]
    pub finish: bool,
    /// Estimation of score. Usually, final score will be greater than or equal to `live_score`
    #[serde(rename = "live_score")]
    pub live_score: Option<i32>,
}

impl RunLiveStatusUpdate {
    /// Represents Live Status Update  Some fields can be missing for various reasons, it is normal that particular object will look like {liveScore: null, currentTest: null, finish: false}. Information in all fields except `finish` can be inaccurate, incorrect or outdated. You can rely on following: if `finish` is true, final judging results are available
    pub fn new(
        current_test: Option<i32>,
        finish: bool,
        live_score: Option<i32>,
    ) -> RunLiveStatusUpdate {
        RunLiveStatusUpdate {
            current_test,
            finish,
            live_score,
        }
    }
}
