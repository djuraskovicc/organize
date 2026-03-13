
#[derive(PartialEq, Debug, Eq)]
pub enum Flag {
    Help,
    Verbose { level: u8 },
    Output { path: String },
}
