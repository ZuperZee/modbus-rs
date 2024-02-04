#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExceptionCode {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    ServerDeviceFailure = 0x04,
    Acknowledge = 0x05,
    ServerDeviceBusy = 0x06,
    MemoryParityError = 0x08,
    GatewayPathUnavailable = 0x0a,
    GatewayTargetDeviceFailedToRespond = 0x0b,
}

impl TryFrom<u8> for ExceptionCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::IllegalFunction),
            0x02 => Ok(Self::IllegalDataAddress),
            0x03 => Ok(Self::IllegalDataValue),
            0x04 => Ok(Self::ServerDeviceFailure),
            0x05 => Ok(Self::Acknowledge),
            0x06 => Ok(Self::ServerDeviceBusy),
            0x08 => Ok(Self::MemoryParityError),
            0x0a => Ok(Self::GatewayPathUnavailable),
            0x0b => Ok(Self::GatewayTargetDeviceFailedToRespond),
            v => Err(v),
        }
    }
}
