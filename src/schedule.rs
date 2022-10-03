use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingClass {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "classId")]
    pub class_id: String,

    #[serde(rename = "sessionId")]
    pub session_id: String,

    #[serde(rename = "resourceId")]
    pub resource_id: String,

    #[serde(rename = "courseId")]
    pub course_id: String,

    #[serde(rename = "classCode")]
    pub class_code: String,

    #[serde(rename = "courseName")]
    pub course_name: String,

    #[serde(rename = "courseCode")]
    pub course_code: String,

    #[serde(rename = "lecturers")]
    pub lecturers: Vec<Lecturer>,

    #[serde(rename = "dateStart")]
    pub date_start: String,

    #[serde(rename = "dateEnd")]
    pub date_end: String,

    #[serde(rename = "resources")]
    pub resources: Vec<Resource>,

    #[serde(rename = "sessionProgress")]
    pub session_progress: i64,

    #[serde(rename = "classDeliveryMode")]
    pub class_delivery_mode: String,

    #[serde(rename = "deliveryMode")]
    pub delivery_mode: String,

    #[serde(rename = "deliveryModeDesc")]
    pub delivery_mode_desc: String,

    #[serde(rename = "joinUrl")]
    pub join_url: Option<String>,

    #[serde(rename = "isEnded")]
    pub is_ended: bool,

    #[serde(rename = "meetingStart")]
    pub meeting_start: String,

    #[serde(rename = "sessionNumber")]
    pub session_number: i64,

    #[serde(rename = "classRoomNumber")]
    pub class_room_number: Option<String>,

    #[serde(rename = "classCampusName")]
    pub class_campus_name: Option<String>,

    #[serde(rename = "courseComponent")]
    pub course_component: String,

    #[serde(rename = "isHasOngoingClass")]
    pub is_has_ongoing_class: bool,

    #[serde(rename = "institutionDesc")]
    pub institution_desc: String,

    #[serde(rename = "academicCareerDesc")]
    pub academic_career_desc: String,

    #[serde(rename = "isWifiAttendance")]
    pub is_wifi_attendance: bool,

    #[serde(rename = "deliveryModeIdReal")]
    pub delivery_mode_id_real: String,

    #[serde(rename = "deliveryModeReal")]
    pub delivery_mode_real: String,

    #[serde(rename = "classDeliveryModeReal")]
    pub class_delivery_mode_real: String,

    #[serde(rename = "deliveryModeDescReal")]
    pub delivery_mode_desc_real: String,

    #[serde(rename = "facilityId")]
    pub facility_id: String,

    #[serde(rename = "roomDescription")]
    pub room_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lecturer {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "pictureUrl")]
    pub picture_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    #[serde(rename = "jumlah")]
    pub count: String,

    #[serde(rename = "type")]
    pub resource_type: String,

    #[serde(rename = "duration")]
    pub duration: String,
}
