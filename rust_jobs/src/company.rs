use serde::{Serialize, Deserialize};

// {
//     "id":1,
//     "name":"Oxide Computer Company",
//     "space":"Cloud Services",
//     "profile":"",
//     "startup":false,
//     "url":null,
//     "open_source_github_org":null,
//     "created_at":"2023-06-20T01:25:02.176Z",
//     "updated_at":"2023-06-20T01:25:02.176Z",
//     "__dozer_record_id":0,
//     "__dozer_record_version":1
// }

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub(crate) struct Company {
    pub id: usize,
    pub name: String,
    pub space: String,
    pub profile: String,
    pub startup: bool,
    pub url: Option<String>,
    pub open_source_github_org: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub __dozer_record_id: usize,
    pub __dozer_record_version: usize,
}
