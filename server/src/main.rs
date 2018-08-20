mod map;
mod random;
mod recv_opcode;
mod send_opcode;

extern crate actix;
extern crate actix_web;
extern crate bincode;
extern crate pcg_rand;
extern crate rand;
extern crate webgl_test_common;

use actix::{Actor, StreamHandler};
use actix_web::{fs, http, server, ws, App, HttpRequest};
use std::io::{self, Cursor, Write};

struct Ws;

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self>;
}

// Handler for `ws::Message` messages
impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Binary(mut bin) => if !bin.is_empty() {
                let binary = bin.take();
                match binary[0] {
                    recv_opcode::MAP_REQUEST => {
                        const MAP_RADIUS: usize = 24;

                        let generated = map::generate_map(MAP_RADIUS);
                        let mut buf =
                            Vec::with_capacity(24 * MAP_RADIUS * MAP_RADIUS);
                        {
                            let mut cur = Cursor::new(&mut buf);
                            cur.write_all(&[send_opcode::MAP_DATA]).unwrap();
                            bincode::serialize_into(&mut cur, &generated)
                                .unwrap();
                        }
                        ctx.binary(buf);
                    },
                    _ => (),
                }
            },
            ws::Message::Ping(msg) => ctx.pong(&msg),
            _ => (),
        }
    }
}

fn main() -> io::Result<()> {
    server::new(|| {
        App::new()
            .resource("/ws/", |r| r.f(|req| ws::start(req, Ws)))
            .route("/", http::Method::GET, |_: HttpRequest| {
                fs::NamedFile::open("index.html")
            }).handler("/", fs::StaticFiles::new(".").unwrap())
    }).bind("0.0.0.0:11484")?
    .run();

    Ok(())
}
