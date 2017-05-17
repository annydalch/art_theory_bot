Art Theory Bot
==============

A Twitterbot that tries to sound like it knows about art. It tweets on [@qotd_bot](https://twitter.com/qotd_bot "Art Theory Bot on Twitter").
Written in [Rust](https://www.rust-lang.org/ "Rust") using the [twitter-api](https://crates.io/crates/twitter-api "twitter-api") and [oauth-client](https://crates.io/crates/oauth-client "oauth-client") crates.
It's licensed under the MIT License as it appears on [choosealicense](https://choosealicense.com/licenses/mit/ "MIT License").
Credit to several of my friends, particularly Ivar and Zoey, for pretentious artsy-sounding things to say.

Running
-------

The enterprising reader who wants to operate such a bot themself will require, of course, [Rust](https://www.rust-lang.org/ "Rust") and [Cargo](https://crates.io/ "Cargo"). I use rust-stable, and I built this bot using 1.17, but because it compiles on stable it should compile on anything 1.17+.
The build target `final` uses [`main.rs`](https://github.com/annydalch/art_theory_bot/blob/master/src/main.rs "main.rs"), and for every time it is called it will send a Tweet. I use [cron](https://en.wikipedia.org/wiki/Cron "Cron") to run it every 4 hours. If you want to tweet, you'll need to create a file called `src/tweet_manager/secret.rs` that defines 4 `Into<Cow<str>>`s: `CONSUMER_KEY`, `CONSUMER_SECRET`, `ACCESS_KEY`, `ACCESS_SECRET`. For me they're `pub const &str`s.
I recommend building this one with `cargo build --release --bin final`.

If, alternatively, you just want to generate the tweets and not send them, you can use the `just-generate` build target. `just-generate` builds [`generator.rs`](https://github.com/annydalch/art_theory_bot/blob/master/src/generator.rs "generator.rs"), which will just generate the text and write it to stdout. Building that probably won't require `secret.rs`, but who knows?
I recommend running this one with `cargo run --bin just-generate`.
