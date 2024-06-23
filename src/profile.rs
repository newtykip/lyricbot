#[cfg(feature = "discord")]
use crate::CommandError;
use crate::{Error, Result};
use lazy_static::lazy_static;
#[cfg(feature = "discord")]
use poise::serenity_prelude as serenity;
use prost::Message;
use std::{fs::File, io::Write, path::PathBuf};
#[cfg(feature = "twitter")]
use twitter_v2::{authorization::BearerToken, TwitterApi};

lazy_static! {
    static ref PROFILE_DIR: PathBuf = {
        let data_dir = dirs::data_local_dir().unwrap(); // it exists on every platform
        data_dir.join("lyricbot")
    };
}

include!(concat!(env!("OUT_DIR"), "/profile.rs"));

impl Profile {
    /// Create a new profile with a random ID and a given name.
    pub fn new(name: String) -> Result<Self> {
        let profile = Self {
            name: name.to_string(),
            discord: None,
            twitter: None,
        };

        // save the profile to disk
        profile.save()?;

        Ok(profile)
    }

    /// Load a profile given an ID.
    pub fn load(id: String) -> Result<Self> {
        let data = std::fs::read(PROFILE_DIR.join(id))?;
        Profile::decode(data.as_slice()).map_err(|e| e.into())
    }

    /// Save the current profile to disk.
    pub fn save(&self) -> Result<()> {
        let mut file = File::create(PROFILE_DIR.join(nanoid::nanoid!(10)))?;
        let data = self.encode_to_vec();
        file.write_all(data.as_slice())?;

        Ok(())
    }

    /// Create a Discord client with the current profile.
    #[cfg(feature = "discord")]
    pub async fn discord_client(&self) -> Result<serenity::Client> {
        let token = self
            .discord
            .clone()
            .ok_or(Error::NoAuth("Discord".to_string()))?;
        let intents = serenity::GatewayIntents::non_privileged();
        let framework = poise::Framework::<_, CommandError>::builder()
            .options(poise::FrameworkOptions {
                commands: vec![],
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(())
                })
            })
            .build();
        let client = serenity::ClientBuilder::new(token, intents)
            .framework(framework)
            .await?;
        Ok(client)
    }

    /// Create a Twitter client with the current profile.
    #[cfg(feature = "twitter")]
    pub fn twitter_client(&self) -> Result<TwitterApi<BearerToken>> {
        let auth = self
            .twitter
            .clone()
            .map(BearerToken::new)
            .ok_or(Error::NoAuth("Twitter".to_string()))?;
        Ok(TwitterApi::new(auth))
    }
}
