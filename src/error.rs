use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArpchatError {
    #[error("unknown channel type, only eth is supported")]
    UnknownChannelType,

    #[error("error getting channel, might be missing permissions")]
    ChannelError(#[from] std::io::Error),

    #[error("invalid interface {0}")]
    InvalidInterface(String),

    #[error("no mac address")]
    NoMAC,

    #[error("couldn't capture packet")]
    CaptureFailed,

    #[error("couldn't serialize arp packet")]
    ARPSerializeFailed,

    #[error("couldn't send arp packet")]
    ARPSendFailed,

    #[error("couldn't parse packet as ethernet")]
    EthParseFailed,

    #[error("message too long to send")]
    MsgTooLong,

    #[error("tried to set interface, but interface is already initialized")]
    InterfaceAlreadySet,
}
