use itertools::Itertools as _;
use ui::event::timetable::{
    EventAct, EventActBuilder, EventDay, EventDayBuilder, EventStage, EventStageBuilder,
    EventTimetable, EventTimetableBuilder,
};

use crate::entity::{
    act::Act,
    day::Day,
    event::EventId,
    stage::{Stage, StageId},
};

pub fn get_event_timetable(
    event_id: EventId,
    active_day: Option<u16>,
    days: &[Day],
    acts: &[Act],
    stages: &[Stage],
) -> EventTimetable {
    let multiple_days = days.len() > 1;
    let event_days = days
        .iter()
        .map(|day| get_event_day_timetable(event_id, day, multiple_days, acts, stages))
        .collect_vec();

    timetable(
        event_id,
        event_days
            .get(active_day.unwrap_or(1) as usize - 1)
            .or_else(|| event_days.first())
            .cloned()
            .expect("Expected at least one day for the event"),
        &event_days,
    )
}

fn day_stages(stages: &[Stage], acts_for_day: Vec<(Option<StageId>, EventAct)>) -> Vec<EventStage> {
    stages
        .iter()
        .map(|stage| {
            EventStageBuilder::default()
                .event_id(stage.event_id)
                .stage_id(stage.id)
                .name(stage.name.clone())
                .has_multiple_stages(stages.len() > 1)
                .acts(
                    acts_for_day
                        .iter()
                        .filter(|(stage_id, act)| *stage_id == Some(stage.id))
                        .map(|(_, act)| act)
                        .cloned()
                        .collect_vec(),
                )
                .build()
                .expect("Failed to build event stage")
        })
        .collect_vec()
}

fn acts_for_day(
    acts: &[Act],
    day: &Day,
) -> Vec<(
    Option<crate::entity::stage::StageId>,
    ui::event::timetable::EventAct,
)> {
    acts.iter()
        .filter(|act| {
            let (Some(start_time), Some(end_time)) = (act.start_time, act.end_time) else {
                return false;
            };
            start_time >= day.start_time && end_time < day.end_time
        })
        .map(|act| {
            (
                act.stage_id,
                EventActBuilder::default()
                    .name(act.name.clone())
                    .image_url(act.image_url.clone())
                    .start_time(act.start_time)
                    .end_time(act.end_time)
                    .build()
                    .expect("Failed to build event act"),
            )
        })
        .collect_vec()
}

pub fn get_event_day_timetable(
    event_id: EventId,
    day: &Day,
    has_multiple_days: bool,
    acts: &[Act],
    stages: &[Stage],
) -> EventDay {
    let acts_for_day = acts_for_day(acts, &day);
    let day_stages = day_stages(stages, acts_for_day);

    EventDayBuilder::default()
        .event_id(event_id)
        .n(day.n)
        .date(day.start_time.date())
        .has_multiple_days(has_multiple_days)
        .stages(day_stages)
        .build()
        .expect("Failed to build event day")
}

fn timetable(event_id: EventId, active_day: EventDay, days: &[EventDay]) -> EventTimetable {
    EventTimetableBuilder::default()
        .event_id(event_id)
        .active_day(active_day)
        .days(days)
        .build()
        .expect("Failed to build event timetable")
}
