use std::path;
extern crate bloodthorne;
use bloodthorne::replay::Replay;
extern crate env_logger;

#[test]
fn replay() {
    env_logger::init().unwrap();
    let mut replay = Replay::from_file(path::Path::new("example.dem")).expect("Error replay");

    replay.callbacks.on_CUserMessageSayText2 = Some(Box::new(|ref m| {
        println!("CUserMessageSayText2: `{}` says `{}`",
                 m.get_param1(),
                 m.get_param2());
    }));

    replay.parse().expect("Error parsing");
}