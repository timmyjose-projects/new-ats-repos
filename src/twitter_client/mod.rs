use super::github_client as gh;
use crate::auth;
use egg_mode::Token;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

mod tweeter;

pub const PAUSE_EVENT_LOOP_FOR_IN_SECONDS: u64 = 15 * 60;
pub const DELAY_BETWEEN_TWEETS_IN_SECONDS: u64 = 5;

/// Start the twitter service when the http server starts up.
/// It will post the list of new ATS repos (if available)
/// once a day at midnight UTC.
/// Also planned is to support responding to follows,
/// direct messages, retweets, favouriting, as well as
/// periodically tweeting random ATS facts.
/// Possibly also tweet about major releases of ATS repos.
pub fn start_service() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(run());
}

/// The main event loop
async fn run() {
    loop {
        let token = match auth::twitter::login().await {
            Err(e) => {
                error!("Error while trying to log in: {:?}", e);
                std::process::exit(1);
            }
            Ok(token) => token,
        };

        query_repos_and_tweet(&token).await;

        info!(
            "Main event loop sleeping for {} seconds",
            PAUSE_EVENT_LOOP_FOR_IN_SECONDS
        );
        thread::sleep(Duration::from_secs(PAUSE_EVENT_LOOP_FOR_IN_SECONDS));
    }
}

/// query github for new ATS repos and attempt to
/// tweet them
async fn query_repos_and_tweet(token: &Token) {
    if let Some(maybe_repos) = gh::get_new_ats_repos() {
        if let Ok(new_repos) = maybe_repos {
            for repo in new_repos {
                match tweeter::tweet_new_repo(&token, &repo).await {
                    Ok(_) => info!("Sent tweet for id {} succesfully", repo.id),
                    Err(e) => error!("Error while trying to tweet id {}: {}", repo.id, e),
                }
                thread::sleep(Duration::from_secs(DELAY_BETWEEN_TWEETS_IN_SECONDS));
            }
        } else {
            info!("No newly-created ATS repos found for now...");
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
async fn check_for_follows(token: &Token) {
    todo!();
}

#[allow(dead_code)]
#[allow(unused_variables)]
async fn check_for_replies_and_retweets(token: &Token) {
    todo!();
}

#[allow(dead_code)]
#[allow(unused_variables)]
async fn check_for_direct_messages() {
    todo!();
}
