#![warn(clippy::all, clippy::pedantic)]
#![deny(clippy::unwrap_used, unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::user::User;

pub mod act;
pub mod artist;
pub mod day;
pub mod festival;
pub mod macros;
pub mod serde_utils;
pub mod stage;
pub mod user;
pub mod venue;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PagedResult<T, U> {
    pub items: Vec<T>,
    pub has_next_page: bool,
    pub after: Option<U>,
}

#[derive(Clone)]
pub struct UiState {
    pub locale: String,
    pub user: Option<User>,
}

routes! {
    pub mod routes {
        home: "";
        sign_up: "signup";
        login: "login";
        logout: "logout";
        festival: "festival" => {
            new: "new" => {
                details_step: "details" => {
                    form: "";
                    validate_name: "validate_name";
                    validate_description: "validate_description";
                    upload_image: "upload_image";
                };
                schedule_step: "schedule";
                confirm_step: "confirm";
            };
            detail: "{festival_id}" (festival_id: uuid::Uuid) => {
                details: "";
                lineup: "lineup";
                timetable: "timetable" => {
                    day: "{day_id}" (day_id: uuid::Uuid) => {
                        stage: "{stage_id}" (stage_id: uuid::Uuid);
                    };
                };
                edit: "edit";
                history: "history";
            };
        };
        artist: "artist" => {
            detail: "{artist_id}" (artist_id: uuid::Uuid);
        };
        venue: "venue" => {
            search: "search";
            new: "new" => {
                inline: "inline" => {
                    form: "";
                };
            };
            detail: "{venue_id}" (venue_id: uuid::Uuid);
        };
        user: "user" => {
            profile: "{user_id}" (user_id: uuid::Uuid);
        };
        assets: "assets" => {
            file: "{file_path}" (file_path: &str);
        };
    }
}

// routes! {
//     pub mod routes {
//         home: "";
//         foo: "foo" => {
//             bar: "bar";
//             baz: "baz" => {
//                 detail: "{baz_id}" (baz_id: i64) => {
//                     edit: "edit";
//                 };
//             };
//         };
//     }
// }
