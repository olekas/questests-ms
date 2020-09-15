use mongodb;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Error {
    pub message: String
}

impl Error {
    pub(crate) fn new(msg: &str) -> Error {
        Error {
            message: String::from(msg)
        }
    }
}

impl From<mongodb::error::Error> for Error
{
    fn from(err: mongodb::error::Error) -> Self {
        Self::new(&*format!("{:?}", err))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl std::ops::Deref for Error {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.message
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
