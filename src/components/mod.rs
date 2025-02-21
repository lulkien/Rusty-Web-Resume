mod education;
mod interest;
mod language;
mod personal_info;
mod project;
mod skill;
mod summary;

use project::{ProjectItem, Projects};
use serde::{Deserialize, Serialize};

use education::{Education, EducationItem};
use interest::Interests;
use language::{LanguageItem, Languages};
use personal_info::{CustomPersonalInfo, PersonalInfo};
use skill::{SkillItem, Skills};
use summary::Summary;

#[derive(Serialize, Deserialize)]
pub struct Resume {
    pub personal_info: PersonalInfo,
    pub sumary: Option<Summary>,
    pub education: Option<Education>,
    pub skills: Option<Skills>,
    pub projects: Option<Projects>,
    pub languages: Option<Languages>,
    pub interests: Option<Interests>,
}

impl Resume {
    #[allow(unused)]
    pub fn dummy() -> Self {
        Self {
            personal_info: PersonalInfo {
                fullname: "Luu Hoang Kien".to_string(),
                title: Some(
                    "Software Developer".to_string(),
                ),
                email: Some(
                    "kien.luuhoang.arch@outlook.com"
                        .to_string(),
                ),
                website: Some("cv.lulkien.cc".to_string()),
                phone: Some("+84988449631".to_string()),
                localtion: Some(
                    "Hanoi, Vietnam".to_string(),
                ),
                custom_field: Some(vec![
                    CustomPersonalInfo {
                        name: "Github".to_string(),
                        data: "https://github.com/lulkien"
                            .to_string(),
                    },
                ]),
            },
            sumary: Some(Summary(
                "Results-driven Software Developer with \
                 proficiency in C++ and Qt framework. \
                 Specialized in in-vehicle infotainment, \
                 I bring expertise in developing robust \
                 applications that seamlessly integrate \
                 innovative features and enhance user \
                 experiences.
Passionate about Linux and specializing in crafting \
                 automation scripts, I find joy in \
                 operating within a Linux environment. My \
                 focus is on refining workflows to ensure \
                 seamless and efficient operations."
                    .to_string(),
            )),
            education: Some(Education(vec![
                EducationItem {
                    institution: "University of \
                                    Engineering and \
                                    Technology"
                        .to_string(),
                    degree: Some(
                        "Bachelor degree".to_string(),
                    ),
                    area_of_study: Some(
                        "Electronic and Telecommunication"
                            .to_string(),
                    ),
                    date_range: "September 2016 - \
                                    October 2020"
                        .to_string(),
                    score: None,
                    sumary: None,
                    website: None,
                },
            ])),
            skills: Some(Skills(vec![
                SkillItem {
                    name: "C++".to_string(),
                    description: None,
                    level: None,
                },
                SkillItem {
                    name: "Rust".to_string(),
                    description: None,
                    level: None,
                },
                SkillItem {
                    name: "Qt Framework".to_string(),
                    description: None,
                    level: None,
                },
                SkillItem {
                    name: "GNU/Linux".to_string(),
                    description: None,
                    level: None,
                },
                SkillItem {
                    name: "Shell scripting".to_string(),
                    description: None,
                    level: None,
                },
                SkillItem {
                    name: "SQL".to_string(),
                    description: None,
                    level: None,
                },
                SkillItem {
                    name: "Git".to_string(),
                    description: None,
                    level: None,
                },
            ])),
            projects: Some(Projects(vec![
                ProjectItem {
                    name: "SNaviBox".to_string(),
                    date_range: "May 2024 - June 2024".to_string(),
                    description: None,
                    website: None,
                    sumary: Some("Maintain and enhance the car \
                        navigation system for a major Japanese client.
    - Utilize expertise in automotive software and customer-specific \
                        requirements.
    - Collaborate with cross-functional teams to implement new \
                        features and improvements.".to_string()),
                },
                ProjectItem {
                    name: "POIP.Cluster".to_string(),
                    date_range: "January 2024 - March 2024".to_string(),
                    description: None,
                    website: None,
                    sumary: Some("Develop a HMI layer for the \
                        cluster of a new luxury car model that \
                        will be on the market in the near future. 
    - Developed Charging Manager application. ".to_string()),
                },
                ProjectItem {
                    name: "Cockpit2024".to_string(),
                    date_range: "July 2023 - December 2023".to_string(),
                    description: None,
                    website: None,
                    sumary: Some("Develop the applications for \
                        the new Cockpit system that was exhibited \
                        at CES2024.
    - Developed Navigation Application. 
    - Developed Media Player. 
    - Integrated text-to-speech feature. 
    - Configured Linux environment. 
    - Automated build and release process with shell scripting.".to_string()),
                },
                ProjectItem {
                    name: "Gen5.Premium".to_string(),
                    date_range: "December 2020 - June 2023".to_string(),
                    description: None,
                    website: None,
                    sumary: Some("Maintain HMI layer applications \
                        for an automotive AVN system for high-end cars.
    - Maintained and implemented new features for Home Screen Application.
    - Elevated the user experience of the Settings Application by \
                        revamping and optimizing the search function.
    - Provide solutions to problems for other features. ".to_string()),
                },
                ProjectItem {
                    name: "Sonant".to_string(),
                    date_range: "Ongoing".to_string(),
                    description: Some("Personal project".to_string()),
                    website: Some("https://github.com/lulkien/sonant".to_string()),
                    sumary: Some("Sonant is a simple C++ wrapper for \
                        recording audio with ALSA and transcribing \
                        with Whisper.cpp library.".to_string()),
                },
            ] )),
            languages: Some(Languages(vec![
                LanguageItem {
                    name: "English".to_string(),
                    description: Some(
                        "General Proficiency".to_string(),
                    ),
                    level: None,
                },
            ])),
            interests: Some(Interests(vec![
                "Customize GNU/Linux's desktop environment.".to_string(),
                "Learn to design PCB and develop firmware in free time.".to_string(),
            ]))
        }
    }
}
