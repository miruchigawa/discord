use std::{error, sync::Arc};

use crate::utils::{sd, waifu::Waifu};

pub struct Data {
    pub waifu: Arc<Waifu>,
    pub sd: Arc<sd::Client>,
}

pub type Error = Box<dyn error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
