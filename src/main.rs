extern crate slack;
use slack::{Event, RtmClient};

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
        if let Event::Hello = event {
            // find the general channel id from the `StartResponse`
            let general_channel_id = cli
                .start_response()
                .channels
                .as_ref()
                .and_then(|channels| {
                    channels.iter().find(|chan| match chan.name {
                        None => false,
                        Some(ref name) => name == "bot-connector",
                    })
                })
                .and_then(|chan| chan.id.as_ref())
                .expect("general channel not found");
            let _ = cli
                .sender()
                .send_message(&general_channel_id, "Hello world! (rtm)");
            // Send a message over the real time api websocket
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("on_connect");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => {
            panic!("No api-key in args! <xoxb-4421616954532-4671980985489-KEcZgBUeOz90i396UtBAMOA6>")
        }
        x => args[x - 1].clone(),
    };
    let mut handler = MyHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}