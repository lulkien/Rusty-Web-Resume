use crate::model::Resume;

const RESUME_DATA: &str =
    include_str!("../config/resume.toml");

pub fn parse_resume_data() -> Option<Resume> {
    let resume: Resume =
        toml::from_str(RESUME_DATA).unwrap();

    Some(resume)
}
