pub enum DatabaseError {
    CodeParsingFailed,
    DatabaseCardUpdateFailed,
    UserIdFilterCreationFailed,
    CheckingCardDeviceTypeFailed,
    CannotDeleteCardWithEmptyDeviceType,
}