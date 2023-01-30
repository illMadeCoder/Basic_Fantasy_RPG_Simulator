#[derive(Debug)]
pub enum CharacterError {
    AncestryError(u8),
    ClassError(u8),
    RestrictionError,
}
