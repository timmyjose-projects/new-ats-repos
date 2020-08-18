pub mod twitter {
    use crate::errors::{GenError, GenResult, NewIdrisRepoError, NewIdrisRepoErrorKind};
    use egg_mode::Token;
    use std::env;

    const CONSUMER_KEY: &'static str = "TWITTER_CONSUMER_KEY";
    const CONSUMER_SECRET: &'static str = "TWITTER_CONSUMER_SECRET";
    const ACCESS_KEY: &'static str = "TWITTER_ACCESS_KEY";
    const ACCESS_SECRET: &'static str = "TWITTER_ACCESS_SECRET";

    pub async fn login() -> GenResult<Token> {
        let consumer_key = env::var(CONSUMER_KEY).unwrap();
        let consumer_secret = env::var(CONSUMER_SECRET).unwrap();
        let consumer_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

        let access_key = env::var(ACCESS_KEY).unwrap();
        let access_secret = env::var(ACCESS_SECRET).unwrap();
        let access_token = egg_mode::KeyPair::new(access_key, access_secret);

        let token = egg_mode::Token::Access {
            consumer: consumer_token,
            access: access_token,
        };

        if let Err(e) = egg_mode::auth::verify_tokens(&token).await {
            error!(
                "Got an error while trying to verify Twitter tokens... {:#?}",
                e
            );

            if let egg_mode::error::Error::TwitterError(_, twitter_errors) = e {
                let error = NewIdrisRepoError::from(twitter_errors.errors[0].code);
                if error.kind == NewIdrisRepoErrorKind::CouldNotLoginInternalError {
                    error!(
                        "Could not login due to internal server error: {:?}",
                        error.to_string()
                    );
                    return Err(GenError::from(error));
                } else {
                    error!("Some other Twitter error occurred.. aborting login process and terminating twitter service");
                    std::process::exit(1);
                }
            } else {
                error!("Some other general error occurred.. exiting");
                std::process::exit(1);
            }
        }

        info!("Loggged on to Twitter successfully");

        Ok(token)
    }
}