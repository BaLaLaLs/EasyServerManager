use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsMessage<T> where T:Serialize {
    pub(crate) msg_type: MsgType,
    pub(crate) data: T
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MsgType {
    SystemStatus
}
// impl WsMessage<T> {
//     fn new(data: T) {
//         WsMessag
//     }
// }