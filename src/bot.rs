// Imports for Twitter Bot
use egg_mode::{
    media::{media_types, upload_media},
    tweet::DraftTweet,
    KeyPair, Token,
};
use std::{
    error::Error,
    path::PathBuf,
    time::Duration,
    fs,
};
use log::info;
// Imports for ScreenShot Bot
use chrono::{ DateTime, Local };
use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{
    Browser,
    protocol::page::ScreenshotFormat,
};

use crate::utils;

pub struct TwitterBot {
    pub token: Token,
}

impl TwitterBot {
    pub fn from_config(config: utils::Config) -> Self {
        let consumer_key = config.twitter.consumer_key;
        let consumer_secret = config.twitter.consumer_secret;
        let consumer = KeyPair::new(consumer_key, consumer_secret);

        let access_key = config.twitter.access_key;
        let access_secret = config.twitter.access_secret;
        let access = KeyPair::new(access_key, access_secret);

        let token = Token::Access {
            consumer: consumer,
            access: access,
        };

        return Self { token };
    }

    pub async fn tweet(&self, image_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        // Create tweet
        let mut tweet = DraftTweet::new("");
        let bytes = std::fs::read(image_path)?;
        let media_handle = upload_media(&bytes, &media_types::image_png(), &self.token).await?;
        tweet.add_media(media_handle.id.clone());
        info!("Tweet created for {:?}", image_path);

        // Tweet
        tweet.send(&self.token).await.unwrap();
        info!("Tweet sent");
        return Ok(());
    }
}

pub struct ScreenshotBot {
    url: String,
}

impl ScreenshotBot {
    pub fn from_config(config: utils::Config) -> Result<Self, failure::Error> {
        let url = format!("http://{}:{}", config.server.host, config.server.port);
        return Ok(Self { url });
    }
    
    pub fn take_screenshot(&self, path: String, element: String) -> Result<PathBuf, failure::Error> {
        let launch_options = LaunchOptionsBuilder::default()
            .window_size((1920, 1080).into())
            .port(4444.into())
            .headless(true)
            .sandbox(false)
            .build().unwrap();

        let browser = Browser::new(launch_options)?;
        let tab = browser.wait_for_initial_tab()?;

        tab.navigate_to(format!("{}{}", &self.url, &path).as_str())?;
        tab.wait_for_element("#app")?;
        tab.wait_for_element_with_custom_timeout(".loaded", Duration::from_millis(30000))?;

        let png_data = tab.wait_for_element(&element)?
                                  .capture_screenshot(ScreenshotFormat::PNG)?;
        info!("Screenshot captured");

        let image_path = Self::save_screenshot(png_data)?;
        return Ok(image_path);
    }

    fn save_screenshot(png_data: Vec<u8>) -> Result<PathBuf, failure::Error> {
        let local: DateTime<Local> = Local::now();
        let time_string = local.format("%Y%m%d_%H%M%S%.3f");
        fs::create_dir_all("screenshots")?;
        let image_path = PathBuf::from(format!("screenshots/output_{}.png", time_string));
        fs::write(image_path.clone(), png_data)?;
        info!("Screenshot saved {}", image_path.as_path().display().to_string());
        Ok(image_path)
    }
}