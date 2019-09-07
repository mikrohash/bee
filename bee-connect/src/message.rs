pub const MESSAGE_LENGTH : usize = 10;

/// A type for the payload messages that can be exchanged between bee nodes.
pub type Message = [u8; MESSAGE_LENGTH];

pub const DISCONNECT_REQUEST: Message = *b"/DISCONNEC";
pub const DISCONNECT_RESPONSE: Message = *b"+DISCONNEC";
pub const CONFUSED_RESPONSE: Message = *b"-CONFUSED ";

#[derive(std::fmt::Debug)]
pub enum MessageType {
    Payload,
    ConnectConfirmation,
    ConnectRequest,
    ConnectRejection,
}

impl MessageType {
    pub fn encode(&self) -> u8 {
        match self {
            MessageType::Payload => 0,
            MessageType::ConnectConfirmation => 1,
            MessageType::ConnectRequest => 2,
            MessageType::ConnectRejection => 3,
        }
    }

    pub fn decode(id : u8) -> Result<Self, String> {
        match id {
            0 => Ok(MessageType::Payload),
            1 => Ok(MessageType::ConnectConfirmation),
            2 => Ok(MessageType::ConnectRequest),
            3 => Ok(MessageType::ConnectRejection),
            _ => Err("Unknown message type ID.".to_string())
        }
    }
}

#[derive(std::fmt::Debug)]
pub struct MessageMetaData {
    message_type : MessageType,
    message_size : u16
}

const MESSAGE_SIZE_VARIANTS: u16 = 4096; // variants of 12 bits
const MESSAGE_TYPE_VARIANTS: u8 = 16; // variants of 4 bits

impl MessageMetaData {
    pub fn new(message_type : MessageType, message_size : usize) -> Result<Self, String> {
        MessageMetaData::assert_limit(message_size, MESSAGE_SIZE_VARIANTS as usize, &"message_size")?;
        MessageMetaData::assert_limit(message_type.encode() as usize, MESSAGE_TYPE_VARIANTS as usize, &"message_type")?;
        Ok(MessageMetaData {
            message_size : MessageMetaData::usize_to_u16(message_size),
            message_type
        })
    }

    fn assert_limit(val : usize, limit : usize, variable_name : &str) -> Result<(), String> {
        if val >= limit as usize {
            Err(format!("`{}` above limit ({} >= {})", variable_name, val, limit).to_string())
        } else {
            Ok(())
        }
    }

    fn usize_to_u16(val : usize) -> u16 {
        let le_bytes = val.to_le_bytes();
        u16::from_le_bytes([le_bytes[0], le_bytes[1]])
    }

    pub fn decode(compact : u16) -> Result<Self, String> {
        let message_type_id = (compact / MESSAGE_SIZE_VARIANTS).to_be_bytes()[1];
        let message_type = MessageType::decode(message_type_id).unwrap();
        let message_size = compact % MESSAGE_SIZE_VARIANTS;
        MessageMetaData::new(message_type, message_size as usize)
    }

    pub fn encode(&self) -> u16 {
        let metadata_last_12_bytes_for_size : u16 = self.message_size;
        let metadata_first_4_bytes_for_type : u16 = self.message_type.encode() as u16 * MESSAGE_SIZE_VARIANTS;
        metadata_first_4_bytes_for_type + metadata_last_12_bytes_for_size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_message_metadata_encoding_decoding() {
        let expected_message_type_id = 2;
        let expected_message_type = MessageType::decode(expected_message_type_id).unwrap();
        let expected_message_size = 1521;

        let metadata_original = MessageMetaData::new(expected_message_type, expected_message_size).unwrap();
        let compact = metadata_original.encode();
        let metadata_decoded = MessageMetaData::decode(compact).unwrap();

        assert_eq!(metadata_decoded.message_size as usize, expected_message_size);
        assert_eq!(metadata_decoded.message_type.encode(), expected_message_type_id);
    }

    #[test]
    fn test_message_type_limit() {
        for id in 0..4 {
            MessageType::decode(id).unwrap();
        }
        for id in 4..10 {
            MessageType::decode(id).unwrap_err();
        }
    }

    #[test]
    fn test_message_size_limit() {
        MessageMetaData::new(MessageType::Payload, MESSAGE_SIZE_VARIANTS as usize - 1) .unwrap();
        MessageMetaData::new(MessageType::Payload, MESSAGE_SIZE_VARIANTS as usize) .unwrap_err();
        MessageMetaData::new(MessageType::Payload, MESSAGE_SIZE_VARIANTS as usize + 1) .unwrap_err();
    }
}