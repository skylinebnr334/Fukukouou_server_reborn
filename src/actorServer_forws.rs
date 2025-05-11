use actix::prelude::*;
use std::time::{Duration, Instant};
use actix::Addr;
use actix_web_actors::ws;
use crate::ws_actors::WsActor;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WsSession_Round1Refresh{
    id:u32,
    hb:Instant,
    addr:Addr<WsActor>
}
impl WsSession_Round1Refresh{
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                act.addr
                    .do_send(crate::ws_actors::Disconnect_Round1Refresh{id:act.id});
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });

    }
}
impl Actor for WsSession_Round1Refresh {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr=ctx.address();
        self.addr
            .send(
                crate::ws_actors::Connect_Round1Refresh{
                    addr:addr.recipient(),
                }
            )
            .into_actor(self)
            .then(|res, act, ctx| {
                match res{
                    Ok(res)=>act.id=res,

                    _=>ctx.stop()
                }
                fut::ready(())
            })
            .wait(ctx);

    }
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(crate::ws_actors::Disconnect_Round1Refresh{id:self.id});
        Running::Stop
    }

}
impl Handler<crate::ws_actors::Message> for WsSession_Round1Refresh {
    type Result = ();
    fn handle(&mut self, msg: crate::ws_actors::Message, _ctx: &mut Self::Context) -> Self::Result {
        _ctx.text(msg.0);
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession_Round1Refresh {

}