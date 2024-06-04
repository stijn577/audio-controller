use thiserror_no_std::Error;

#[derive(Error, Debug, defmt::Format)]
pub enum MyError<'a> {
    #[error("Generic Error: {0}")]
    _Generic(&'a str),
}
