// Refactor 4.0
pub enum DatabaseError {
    CodeParsingFailed,
    DatabaseCardUpdateFailed,
    UserIdFilterCreationFailed,
    CheckingCardDeviceTypeFailed,
    CannotDeleteCardWithEmptyDeviceType,
}