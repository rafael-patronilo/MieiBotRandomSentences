use std::fs;
use serde::{Deserialize};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::id::RoleId;
use serenity::model::guild::GuildContainer;

#[derive(Deserialize)]
struct Settings {
    token: String,
    role_id: u64,
    random_sentences: Vec<String>,
}

struct Handler {
    settings: Settings
}

impl EventHandler for Handler {
    //Will reply with a random message case the author doesn't have a specified role
    fn message(&self, ctx: Context, msg: Message) {
        let role_id: u64 = self.settings.role_id.clone();
        //checks bot if the author hasn't the role and isn't a bot
        //not checking if is a bot would result in the bot replying itself afterwards
        if !(msg.author.has_role(&ctx, GuildContainer::Id(msg.guild_id.unwrap()), RoleId(role_id)).unwrap()
            || msg.author.bot) {
            //select the random index
            let index: usize = rand::random::<usize>() % self.settings.random_sentences.len();
            //Gets the string in the specified index replacing any "{}" for the target user's mention
            let content: String = self.settings.random_sentences.get(index).unwrap().replace("{}", &msg.author.mention());
            msg.channel_id.say(ctx.http, content).expect("Something went wrong replying to message");
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    //Obtains the Settings.json file in the root of the project. This is where the bot's token and other data is stored
    let serialized_settings = fs::read_to_string("Settings.json")
        .expect("Setting.json could not be found");
    let settings: Settings = serde_json::from_str(serialized_settings.as_str()).expect("Setting.json could not be deserialized");

    let mut client = Client::new(settings.token.clone(), Handler { settings }).expect("Error creating client");


    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}