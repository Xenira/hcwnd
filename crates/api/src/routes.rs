use std::fmt::Display;

use uuid::Uuid;

pub const EVENT_ROUTE: &str = "/event";
pub const EVENT_DETAIL_ROUTE: &str = "/detail";
pub const EVENT_TIMETABLE_ROUTE: &str = "/timetable";
pub const EVENT_LINEUP_ROUTE: &str = "/lineup";

pub const ARTIST_ROUTE: &str = "/artist";

pub const VENUE_ROUTE: &str = "/venue";
pub const VENUE_NEW_ROUTE: &str = "/new";
pub const VENUE_NEW_INLINE_ROUTE: &str = "/new/inline";

#[derive(Debug, Clone)]
pub struct RouteBuilder<T = Route> {
    buffer: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T: RouteSegment> Default for RouteBuilder<T> {
    fn default() -> Self {
        Self {
            buffer: T::PATH.to_string(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> RouteBuilder<T> {
    pub fn new(buffer: String) -> Self {
        Self {
            buffer,
            _marker: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn build(self) -> String {
        self.buffer
    }

    #[must_use]
    pub fn into<U: RouteSegment>(self) -> RouteBuilder<U> {
        RouteBuilder {
            buffer: format!("{}{}", self.buffer, U::PATH),
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct Route;

impl RouteSegment for Route {
    const PATH: &'static str = "";
}

impl RouteBuilder<Route> {
    #[must_use]
    pub fn venue(self) -> RouteBuilder<VenueRoute> {
        self.into()
    }

    #[must_use]
    pub fn festival(self) -> RouteBuilder<FestivalRoute> {
        self.into()
    }
}

// ###############
// Festival routes
// ###############
pub struct FestivalRoute;

impl RouteBuilder<FestivalRoute> {
    #[must_use]
    pub fn detail(self, festival_id: Uuid) -> RouteBuilder<FestivalDetailRoute> {
        RouteBuilder::new(format!("{}/{festival_id}", self.buffer))
    }

    #[must_use]
    pub fn new_festival(self) -> RouteBuilder<FestivalNewRoute> {
        self.into()
    }
}

impl RouteSegment for FestivalRoute {
    const PATH: &'static str = "/festival";
}

pub struct FestivalNewRoute;

impl RouteBuilder<FestivalNewRoute> {
    #[must_use]
    pub fn details(self) -> RouteBuilder<FestivalNewDetailRoute> {
        self.into()
    }
}

impl RouteSegment for FestivalNewRoute {
    const PATH: &'static str = "/new";
}

pub struct FestivalNewDetailRoute;


// ############
// Venue routes
// ############
pub struct VenueRoute;

impl RouteBuilder<VenueRoute> {
    #[must_use]
    pub fn detail(self, venue_id: Uuid) -> RouteBuilder<VenueDetailRoute> {
        RouteBuilder::new(format!("{}/{venue_id}", self.buffer))
    }

    #[must_use]
    pub fn new_venue(self) -> RouteBuilder<VenueNewRoute> {
        self.into()
    }
}

impl RouteSegment for VenueRoute {
    const PATH: &'static str = "/venue";
}

pub struct VenueNewRoute;

impl RouteBuilder<VenueNewRoute> {
    #[must_use]
    pub fn inline(self) -> RouteBuilder<VenueNewInlineRoute> {
        self.into()
    }
}

impl RouteSegment for VenueNewRoute {
    const PATH: &'static str = "/new";
}

pub struct VenueNewInlineRoute;

impl RouteSegment for VenueNewInlineRoute {
    const PATH: &'static str = "/inline";
}

pub struct VenueDetailRoute;

pub trait RouteSegment {
    const PATH: &'static str;
}

impl Display for VenueRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::PATH)
    }
}
