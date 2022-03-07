use serde::{Deserialize, Serialize};

pub type Schedule = Vec<ScheduleElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleElement {
    #[serde(rename = "StartDate")]
    pub start_date: String,

    #[serde(rename = "DisplayStartDate")]
    pub display_start_date: String,

    #[serde(rename = "StartTime")]
    pub start_time: String,

    #[serde(rename = "EndTime")]
    pub end_time: String,

    #[serde(rename = "SsrComponentDescription")]
    pub ssr_component_description: SsrComponentDescription,

    #[serde(rename = "ClassCode")]
    pub class_code: String,

    #[serde(rename = "Room")]
    pub room: None,

    #[serde(rename = "Campus")]
    pub campus: None,

    #[serde(rename = "DeliveryMode")]
    pub delivery_mode: DeliveryMode,

    #[serde(rename = "CourseCode")]
    pub course_code: String,

    #[serde(rename = "CourseTitleEn")]
    pub course_title_en: String,

    #[serde(rename = "ClassType")]
    pub class_type: None,

    #[serde(rename = "WeekSession")]
    pub week_session: i64,

    #[serde(rename = "CourseSessionNumber")]
    pub course_session_number: i64,

    #[serde(rename = "MeetingId")]
    pub meeting_id: String,

    #[serde(rename = "MeetingPassword")]
    pub meeting_password: String,

    #[serde(rename = "MeetingUrl")]
    pub meeting_url: String,

    #[serde(rename = "UserFlag")]
    pub user_flag: UserFlag,

    #[serde(rename = "BinusianId")]
    pub binusian_id: String,

    #[serde(rename = "PersonCode")]
    pub person_code: String,

    #[serde(rename = "FullName")]
    pub full_name: String,

    #[serde(rename = "AcademicCareer")]
    pub academic_career: String,

    #[serde(rename = "ClassMeetingId")]
    pub class_meeting_id: String,

    #[serde(rename = "Location")]
    pub location: None,

    #[serde(rename = "MeetingStartDate")]
    pub meeting_start_date: String,

    #[serde(rename = "Id")]
    pub id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum None {
    #[serde(rename = "-")]
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeliveryMode {
    #[serde(rename = "GSLC")]
    Gslc,

    #[serde(rename = "VC")]
    Vc,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SsrComponentDescription {
    #[serde(rename = "Laboratory")]
    Laboratory,

    #[serde(rename = "Lecture")]
    Lecture,

    #[serde(rename = "Tutorial")]
    Tutorial,
}

impl std::fmt::Display for DeliveryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserFlag {
    #[serde(rename = "Student")]
    Student,
}
