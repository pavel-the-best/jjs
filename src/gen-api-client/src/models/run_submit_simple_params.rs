/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunSubmitSimpleParams {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "contest")]
    pub contest: String,
    #[serde(rename = "problem")]
    pub problem: String,
    #[serde(rename = "toolchain")]
    pub toolchain: String,
}

impl RunSubmitSimpleParams {
    pub fn new(
        code: String,
        contest: String,
        problem: String,
        toolchain: String,
    ) -> RunSubmitSimpleParams {
        RunSubmitSimpleParams {
            code,
            contest,
            problem,
            toolchain,
        }
    }
}
