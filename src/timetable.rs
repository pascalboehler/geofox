
use crate::model::Station;
pub struct Timetable {
    id: u32,
    station: Station,
    departures: Vec<Departure>
}

struct Departure {
    id: u32
}