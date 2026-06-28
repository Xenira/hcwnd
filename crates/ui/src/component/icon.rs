use std::fmt::Display;

use maud::{html, Markup};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Icons {
    ActImagePlaceholder,
    AddAct,
    Close,
    Date,
    Edit,
    EditCancel,
    EndDate,
    EndTime,
    EventDetails,
    EventLineup,
    EventTimetable,
    Like,
    Loading,
    NoActs,
    NoTimetable,
    Stage,
    StartTime,
    Unlike,
    Verified,
    Website,
    Upvote,
    Downvote,
}

impl Display for Icons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ph-")?;

        #[allow(clippy::match_same_arms)]
        match self {
            Icons::ActImagePlaceholder => write!(f, "vinyl-record"),
            Icons::AddAct => write!(f, "user-circle-plus"),
            Icons::Close => write!(f, "x"),
            Icons::Date => write!(f, "calendar"),
            Icons::Edit => write!(f, "pencil-simple"),
            Icons::EditCancel => write!(f, "pencil-simple-slash"),
            Icons::EndDate => write!(f, "calendar-slash"),
            Icons::EndTime => write!(f, "clock-countdown"),
            Icons::EventDetails => write!(f, "article"),
            Icons::EventLineup => write!(f, "users-three"),
            Icons::EventTimetable => write!(f, "calendar-dots"),
            Icons::Like => write!(f, "heart"),
            Icons::Loading => write!(f, "vinyl-record"),
            Icons::NoActs => write!(f, "user-circle-dashed"),
            Icons::NoTimetable => write!(f, "mask-sad"),
            Icons::Stage => write!(f, "map-pin-area"),
            Icons::StartTime => write!(f, "clock"),
            Icons::Unlike => write!(f, "heart-break"),
            Icons::Verified => write!(f, "seal-check"),
            Icons::Website => write!(f, "globe"),
            Icons::Upvote => write!(f, "thumbs-up"),
            Icons::Downvote => write!(f, "thumbs-down"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Style {
    Bold,
    Duotone,
    Fill,
    Light,
    #[default]
    Regular,
    Thin,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Bold => write!(f, "ph-bold"),
            Style::Duotone => write!(f, "ph-duotone"),
            Style::Fill => write!(f, "ph-fill"),
            Style::Light => write!(f, "ph-light"),
            Style::Regular => write!(f, "ph"),
            Style::Thin => write!(f, "ph-thin"),
        }
    }
}

pub fn icon(icon: &Icons, icon_style: Option<Style>) -> Markup {
    html! {
        i .(icon) .(icon_style.unwrap_or_default()) {}
    }
}
