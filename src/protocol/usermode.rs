use std::str::FromStr;

pub enum UserMode {
    Away,
    Invisible,
    ReceiveWallops,
    RestrictedUserConnection,
    Operator,
    LocalOperator,
    ReceiveServerNotice,
}
