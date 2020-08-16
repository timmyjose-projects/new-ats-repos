# New ATS Repos Bot

This is the source code for the Twitter Bot (@NewATSRepos) that periodically queries Github for new ATS repositories, and then posts a link to the 
repository (along with a brief summary) in a tweet. 

This bot was inspired by the Common Lisp bot (@NewLispRepos) created by Zach Beane.

# How to build and run

  1. Clone this project.

  2. Configure the proper keys locally. For instance, have a file like `env.sh` (make sure to add this to `.gitignore`) like so:

  ```
    export LAST_QUERY_TIMESTAMP="<timestamp in rfc-3339 format" // Eg: 2020-07-04T17:59:37.489450Z"
    export TWITTER_CONSUMER_KEY=<your twitter consumer key>
    export TWITTER_CONSUMER_SECRET=<your twitter secret key>
    export TWITTER_ACCESS_KEY=<your twitter access key>
    export TWITTER_ACCESS_SECRET=<your twitter access secret>
    export BOT_HEROKU_QUERY_URL=https://api.heroku.com/apps/<your-heroku-app-name-or-id>/config-vars
    export BOT_HEROKU_AUTH_TOKEN=<base64 encoded heroku auth token>
  ```

   3. Source the `env.sh` file in the terminal window from where you invoke `cargo build --release && cargo run`. 
      This is for local testing. For deployment, see the next step.

   4. Note that the default configuration is to deploy on Heroku (see `Procfile`). To deploy Rust projects on Heroku, a 
      [buildpack is handily available](https://github.com/emk/heroku-buildpack-rust).

Also note that currently the project is linked to the Heroku ecosystem, but it can be made independent of any deployment mechanism
relatively easily.

# LICENCE

See [LICENCE](LICENSE.md).
