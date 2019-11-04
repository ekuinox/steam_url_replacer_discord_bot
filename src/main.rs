use std::env;
use dotenv::dotenv;
use regex::Regex;
use serenity::{
    model::{channel::Message, gateway::Ready, guild::Guild},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let re = Regex::new(r"store\.steampowered\.com/app/([0-9]+)").unwrap();
        if let Some(caps) = re.captures(msg.content.as_str()) {
            let app_id = &caps[1];

            if let Err(why) = msg.channel_id.say(&ctx.http, format!("steam://store/{}", app_id)) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        ready.guilds.into_iter()
            .map(move |guild| { guild.id() })
            .map(move |id| {
                match Guild::get(&ctx.http, id) {
                    Ok(guild) => Some((id, guild.name)),
                    Err(_) => None
                }
            })
            .filter_map(move |r| { r })
            .for_each(move |(id, name)| {
                println!("joined at {} ({})", name, id);
            });
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
