use std::env;
use std::fs;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::id::RoleId;

#[derive(RustcDecodable)]
struct Settings {
    token: String,
    role_id: u64,
    random_sentences: Vec<String>,
}

struct Handler {
    settings: Settings
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let guild = msg.guild_id.to_guild();
        if msg.author.has_role(&ctx.http, guild, RoleId(&self.settings.role_id.into())) {
            msg.channel_id.say(&ctx.http, "test")
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let serialized_settings = fs::read_to_string("Settings.json")
        .expect("Something went wrong reading the file");
    let settings: Settings = json::decode(serialized_settings).unwrap();
    let token = env::var("")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler(settings)).expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}