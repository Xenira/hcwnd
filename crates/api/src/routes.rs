use std::fmt::Display;

pub const EVENT_ROUTE: &str = "/event";
pub const EVENT_DETAIL_ROUTE: &str = "/detail";
pub const EVENT_TIMETABLE_ROUTE: &str = "/timetable";
pub const EVENT_LINEUP_ROUTE: &str = "/lineup";

pub const ARTIST_ROUTE: &str = "/artist";

pub const VENUE_ROUTE: &str = "/venue";
pub const VENUE_NEW_ROUTE: &str = "/new";
pub const VENUE_NEW_INLINE_ROUTE: &str = "/new/inline";

#[derive(Debug, Clone)]
pub struct RouteBuilder<T: RouteSegment = Route> {
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

impl<T: RouteSegment> RouteBuilder<T> {
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
}

pub struct VenueRoute;

impl RouteBuilder<VenueRoute> {
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
    pub fn inline(self) -> RouteBuilder<VenueNewRoute> {
        self.into()
    }
}

impl RouteSegment for VenueNewRoute {
    const PATH: &'static str = "/new";
}

pub trait RouteSegment {
    const PATH: &'static str;
}

impl Display for VenueRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::PATH)
    }
}
