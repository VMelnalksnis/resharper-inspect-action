use serde_derive::{Deserialize};
use serde_xml_rs::{from_str};

pub fn parse_report(report_content: &str) -> Report {
    from_str(report_content).unwrap()
}

#[derive(Deserialize)]
pub struct Report {
    #[serde(rename = "Information")]
    pub information: Information,
    #[serde(rename = "IssueTypes")]
    pub issue_types: IssueTypes,
    #[serde(rename = "Issues")]
    pub issues: Issues,
}

#[derive(Deserialize)]
pub struct Information {
    #[serde(rename = "Solution")]
    pub solution: String,
    #[serde(rename = "InspectionScope")]
    pub inspection_scope: InspectionScope,
}

#[derive(Deserialize)]
pub struct InspectionScope {
    #[serde(rename = "Element")]
    pub element: String,
}

#[derive(Deserialize)]
pub struct IssueTypes {
    #[serde(rename = "$value")]
    pub issue_types: Vec<IssueType>,
}

#[derive(Deserialize)]
pub struct IssueType {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "CategoryId")]
    pub category_id: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Severity")]
    pub severity: String,
}

#[derive(Deserialize)]
pub struct Issues {
    #[serde(rename = "$value")]
    pub projects: Vec<Project>,
}

#[derive(Deserialize)]
pub struct Project {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub issues: Vec<Issue>,
}

#[derive(Deserialize)]
pub struct Issue {
    #[serde(rename = "TypeId")]
    pub type_id: String,
    #[serde(rename = "File")]
    pub file: String,
    #[serde(rename = "Offset")]
    pub offset: String,
    #[serde(rename = "Line")]
    pub line: u32,
    #[serde(rename = "Message")]
    pub message: String,
}
