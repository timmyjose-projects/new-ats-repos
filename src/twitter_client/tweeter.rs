use crate::errors;
use crate::github_client::Repo;
use egg_mode::tweet::DraftTweet;
use egg_mode::Token;

#[derive(Debug)]
struct Tweet<'a> {
    login: &'a str,
    name: &'a str,
    url: &'a str,
    description: Option<&'a str>,
}

impl Tweet<'_> {
    fn formatted(self: &Self) -> String {
        format!(
            "{}: {} --- {}\n{}",
            self.login,
            self.name,
            if let Some(d) = self.description {
                d
            } else {
                "Description not available"
            },
            self.url,
        )
    }
}

fn construct_tweet(repo: &Repo) -> Tweet {
    Tweet {
        login: &repo.owner.login,
        name: &repo.name,
        url: &repo.html_url,
        description: if let Some(d) = &repo.description {
            Some(d)
        } else {
            None
        },
    }
}

pub async fn tweet_new_repo(token: &Token, repo: &Repo) -> errors::GenResult<()> {
    info!("prepating to send tweet with id {}", repo.id);

    let tweet_text = construct_tweet(&repo).formatted();
    let tweet = DraftTweet::new(tweet_text);
    tweet.send(&token).await?;

    info!("finished sending tweet with id {}", repo.id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github_client::Owner;

    #[test]
    fn test_tweet() {
        let repo1 = Repo {
            id: 12345789,
            created_at: "2020-07-05".to_owned(),
            name: "qwerty lib".to_owned(),
            html_url: "https://example.com".to_owned(),
            description: Some("A nice bloke".to_owned()),
            owner: Owner {
                id: 987654321,
                login: "bob".to_owned(),
                avatar_url: Some("foo@bar@baz.com".to_owned()),
            },
        };

        let tweet1 = construct_tweet(&repo1);
        println!("{:#?}", tweet1);

        let repo2 = Repo {
            id: 12345789,
            created_at: "2020-07-05".to_owned(),
            name: "dvorak lib".to_owned(),
            html_url: "https://example.com".to_owned(),
            description: None,
            owner: Owner {
                id: 987654321,
                login: "bob".to_owned(),
                avatar_url: Some("foo@bar@baz.com".to_owned()),
            },
        };

        let tweet2 = construct_tweet(&repo2);
        println!("{:#?}", tweet2);
    }
}