use chrono::{Timelike, Utc};
use rand;

use ::error;
use ::protocol::ProtocolVersion;
use ::tl::message::MessageCommon;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessagePurpose {
    Content,
    NonContent,
}


pub struct State {
    id: i64,
    pub(crate) auth_raw_key: [u8; 256],
    time_offset: i32,
    salt: i64,
    seq_no: u32,
    last_msg_id: i64,
    pub(crate) version: ProtocolVersion,
}

impl State {
    pub fn new(version: ProtocolVersion) -> State {
        State {
            id: rand::random(),
            auth_raw_key: [0; 256],
            time_offset: 0,
            salt: 0,
            seq_no: 0,
            last_msg_id: 0,
            version,
        }
    }

    pub fn create_message<T, M>(&mut self, obj: T, purpose: MessagePurpose) -> error::Result<M>
        where M: MessageCommon<T>
    {
        let message_id = self.get_new_msg_id();
        let seq_no = self.next_seq_no(purpose);

        M::new(self.salt, self.id, message_id, seq_no, obj)
    }

    pub fn update_message_id<M, T>(&mut self, msg: &mut M)
        where M: MessageCommon<T>
    {
        msg.set_message_id(self.get_new_msg_id());
    }

    fn get_new_msg_id(&mut self) -> i64 {
        let now = Utc::now();
        let timestamp_server = now.timestamp() + i64::from(self.time_offset);
        let nano = i64::from(now.nanosecond());

        let mut new_msg_id = (timestamp_server << 32) | (nano << 2);
        if self.last_msg_id >= new_msg_id {
            new_msg_id = self.last_msg_id + 4
        }

        self.last_msg_id = new_msg_id;

        new_msg_id
    }

    pub fn update_time_offset(&mut self, correct_msg_id: i64) -> i32 {
        let now = Utc::now();
        let correct_timestamp = correct_msg_id >> 32;  // FIXME: what about the sign bit?
        self.time_offset = (correct_timestamp - now.timestamp()) as i32;  // FIXME
        self.last_msg_id = 0;
        self.time_offset
    }

    fn next_seq_no(&mut self, purpose: MessagePurpose) -> u32 {
        match purpose {
            MessagePurpose::Content => {
                let result = self.seq_no * 2 + 1;
                self.seq_no += 1;
                result
            },
            MessagePurpose::NonContent => {
                self.seq_no * 2
            },
        }
    }
}