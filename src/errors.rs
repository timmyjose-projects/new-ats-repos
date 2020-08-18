use std::error::Error;
use std::fmt;

pub type GenError = Box<dyn std::error::Error>;
pub type GenResult<T> = std::result::Result<T, GenError>;

#[derive(Debug)]
pub struct NewATSRepoError {
    pub kind: NewATSRepoErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum NewATSRepoErrorKind {
    AlreadyFavorited,
    AlreadyRetweeted,
    CannotReplyToDeletedOrInvisibleTweet,
    CouldNotLoginInternalError,
    PostingLimitReached,
    TweetIsTooLong,
    TweetIsUnavailable,
    UnknownError,
}

impl From<NewATSRepoErrorKind> for NewATSRepoError {
    fn from(kind: NewATSRepoErrorKind) -> NewATSRepoError {
        NewATSRepoError { kind: kind }
    }
}

impl From<i32> for NewATSRepoError {
    fn from(code: i32) -> NewATSRepoError {
        match code {
            131 => NewATSRepoError::from(NewATSRepoErrorKind::CouldNotLoginInternalError),
            139 => NewATSRepoError::from(NewATSRepoErrorKind::AlreadyFavorited),
            185 => NewATSRepoError::from(NewATSRepoErrorKind::PostingLimitReached),
            186 => NewATSRepoError::from(NewATSRepoErrorKind::TweetIsTooLong),
            327 => NewATSRepoError::from(NewATSRepoErrorKind::AlreadyRetweeted),
            385 => NewATSRepoError::from(NewATSRepoErrorKind::CannotReplyToDeletedOrInvisibleTweet),
            421 => NewATSRepoError::from(NewATSRepoErrorKind::TweetIsUnavailable),
            _ => NewATSRepoError::from(NewATSRepoErrorKind::UnknownError),
        }
    }
}

impl Error for NewATSRepoError {}

impl fmt::Display for NewATSRepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
