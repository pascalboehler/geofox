struct Train {}

pub struct Station {
    pub id: String,
    pub name: String,
    pub city: String,
    pub combined_name: String,
    pub layer: i8, //must be signed, as underground stations might exist

    pub isHVV: bool,
}
