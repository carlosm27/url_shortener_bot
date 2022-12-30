use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};


mod data;
use crate::data::{DATA, StoredURL};



#[tokio::main]
async fn main() {
    

    start_bot().await;

}

async fn start_bot() {
    
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    

    let bot = Bot::from_env();

    let addr = ([127, 0, 0, 1], 8000).into();
    let ngrok_url = "https://8106-186-185-27-89.ngrok.io".parse().unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, ngrok_url))
        .await
        .expect("Couldn't setup webhook");


    Command::repl_with_listener(bot, answer, listener).await;
}


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Use this command to save a URL")]
    Save(String),
    #[command(description = "Use this command to retrieve a URL with its ID")]
    Get(String),
    #[command(description = "handle user's chat ID")]
    ChatId,
    
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::ChatId => {
            bot.send_message(msg.chat.id, format!("Your chat ID is {chat_id}")).await?
        }
        Command::Save(text) => {
            bot.send_message(msg.chat.id, save_url(text)).await?
        }
        Command::Get(text) => {
            bot.send_message(msg.chat.id, get_url(text)).await?
        }
    };

    Ok(())
}

fn save_url(url: String) -> String {
    let new_id = &nanoid::nanoid!(6).to_string();  
    
    let new_url= StoredURL{id:new_id.clone(), https_address:url};

    
    let mut data = DATA.lock().unwrap();
    
    
    data.insert(new_url.id, new_url.https_address);
            
    format!("URL saved, the ID is {}", new_id)
    
  
}

pub fn get_url(id:String) -> String {
    
    
    let data = DATA.lock().unwrap();
    
    
    match data.get(&id) {
        Some(value) => format!("{}",value.to_string()),
        None => format!("There is not URL with this ID")
    
}
    
    
     
    
}    
    
    
 
    
