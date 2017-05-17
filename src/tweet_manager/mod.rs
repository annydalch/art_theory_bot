extern crate twitter_api as twitter;
extern crate oauth_client as oauth;

// secret holds api keys and crap
// they are const &strs named:
// CONSUMER_KEY
// CONSUMER_SECRET
// ACCESS_KEY
// ACCESS_SECRET
// secret.rs is not in the repo for obvious reasons
mod secret;
use self::secret::*;
use std::convert::AsRef;

pub fn new_tweet<T>(text: T) -> Result<(), twitter::Error>
    where T: AsRef<str>
// given an AsRef<str> containing the text to tweet,
// tweet it out
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
