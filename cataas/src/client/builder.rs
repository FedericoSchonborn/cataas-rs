use crate::{Client, USER_AGENT};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default)]
pub struct ClientBuilder {
    user_agent: Option<String>,
}

impl ClientBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_user_agent<S>(&mut self, user_agent: Option<S>) -> &mut Self
    where
        S: Into<String>,
    {
        self.user_agent = user_agent.map(Into::into);
        self
    }

    pub fn user_agent<S>(&mut self, user_agent: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.set_user_agent(Some(user_agent))
    }

    #[must_use]
    pub fn build(&self) -> Client {
        Client {
            client: reqwest::Client::new(),
            user_agent: self.user_agent.as_deref().unwrap_or(USER_AGENT).to_owned(),
        }
    }
}
