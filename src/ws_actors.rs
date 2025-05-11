use actix::prelude::*;
use std::collections::HashMap;

pub struct WsActor {
    sessions_Round1Reflesh:HashMap<u32,Recipient<Message>>,

    sessions_Round2Reflesh:HashMap<u32,Recipient<Message>>,
}

# [derive(Message)]
# [rtype(result = "()")]
pub struct Message(pub String);
# [derive(Message)]
# [rtype(u32)]
pub struct Connect_Round1Refresh{
    pub addr:Recipient<Message>,
}

# [derive(Message)]
# [rtype(result = "()")]
pub struct Disconnect_Round1Refresh{
    pub id:u32
}


# [derive(Message)]
# [rtype(u32)]
pub struct Connect_Round2Refresh{
    pub addr:Recipient<Message>,
}

# [derive(Message)]
# [rtype(result = "()")]
pub struct Disconnect_Round2Refresh{
    pub id:u32
}
# [derive(Message)]
# [rtype(result = "()")]
pub struct Round1RefreshMessage{
    pub msg: String,
}
# [derive(Message)]
# [rtype(result = "()")]
pub struct Round2RefreshMessage{
    pub msg: String,
}

impl WsActor{
    pub fn new()->WsActor{
        WsActor{
            sessions_Round1Reflesh:HashMap::new(),
            sessions_Round2Reflesh:HashMap::new(),
        }
    }
    fn send_message_R1Refresh(&self,message:&str){
        for(_,addr ) in &self.sessions_Round1Reflesh{
            let _=addr.do_send(Message(message.to_owned()));
        }
    }
    fn send_message_R2Refresh(&self,message:&str){
        for(_,addr ) in &self.sessions_Round2Reflesh{
            let _=addr.do_send(Message(message.to_owned()));
        }
    }
}
impl Actor for WsActor{
    type Context = Context<Self>;
}