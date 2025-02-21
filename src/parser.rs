use crate::components::Resume;

const RESUME_STR: &str =
    include_str!("../data/resume.toml");

pub fn parse_resume_data() -> Option<Resume> {
    let resume: Resume =
        toml::from_str(RESUME_STR).unwrap();

    export_resume(&resume);

    Some(resume)
}

pub fn export_resume(resume: &Resume) -> String {
    let toml = toml::to_string(resume).unwrap();
    log::info!("{toml}");
    toml
}
