use std::error::Error;
use std::fmt;

pub type GenError = Box<dyn std::error::Error>;
pub type GenResult<T> = std::result::Result<T, GenError>;

#[derive(Debug)]
pub struct NewIdrisRepoError {
    pub kind: NewIdrisRepoErrorKind,
}

#[derive(Debug, PartialEq)]
pub enum NewIdrisRepoErrorKind {
    AlreadyFavorited,
    AlreadyRetweeted,
    CannotReplyToDeletedOrInvisibleTweet,
    CouldNotLoginInternalError,
    PostingLimitReached,
    TweetIsTooLong,
    TweetIsUnavailable,
    UnknownError,
}

impl From<NewIdrisRepoErrorKind> for NewIdrisRepoError {
    fn from(kind: NewIdrisRepoErrorKind) -> NewIdrisRepoError {
        NewIdrisRepoError { kind: kind }
    }
}

impl From<i32> for NewIdrisRepoError {
    fn from(code: i32) -> NewIdrisRepoError {
        match code {
            131 => NewIdrisRepoError::from(NewIdrisRepoErrorKind::CouldNotLoginInternalError),
            139 => NewIdrisRepoError::from(NewIdrisRepoErrorKind::AlreadyFavorited),
            185 => NewIdrisRepoError::from(NewIdrisRepoErrorKind::PostingLimitReached),
            186 => NewIdrisRepoError::from(NewIdrisRepoErrorKind::TweetIsTooLong),
            327 => NewIdrisRepoError::from(NewIdrisRepoErrorKind::AlreadyRetweeted),
            385 => {
                NewIdrisRepoError::from(NewIdrisRepoErrorKind::CannotReplyToDeletedOrInvisibleTweet)
            }
            421 => NewIdrisRepoError::from(NewIdrisRepoErrorKind::TweetIsUnavailable),
            _ => NewIdrisRepoError::from(NewIdrisRepoErrorKind::UnknownError),
        }
    }
}

impl Error for NewIdrisRepoError {}

impl fmt::Display for NewIdrisRepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
