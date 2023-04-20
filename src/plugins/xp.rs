use serde_json::Value;
use serenity::model::prelude::Message;

pub fn run(msg: &Message, config: &mut Value) {
    let users = config.as_object_mut().unwrap().get_mut("users").unwrap();

    let xp_gain = 100;

    let xp = users
        .as_object_mut()
        .unwrap()
        .get_mut(&msg.author.id.to_string())
        .unwrap();

    *xp = (xp.as_u64().unwrap() + xp_gain).into();
}
