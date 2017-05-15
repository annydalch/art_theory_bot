extern crate twitter_api as twitter;
extern crate oauth_client as oauth;

mod secret;
use self::secret::*;
use std::convert::AsRef;

pub fn new_tweet<T>(text: T) -> Result<(), twitter::Error>
    where T: AsRef<str>
{
    let consumer_token = oauth::Token::new(
        CONSUMER_KEY,
        CONSUMER_SECRET
    );
    let access_token = oauth::Token::new(
        ACCESS_KEY,
        ACCESS_SECRET
    );
    twitter::update_status(
        &consumer_token,
        &access_token,
        text.as_ref()
    )
}
