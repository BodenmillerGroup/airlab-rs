mod error;

pub use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i32,
}

impl Ctx {
    #[must_use]
    pub const fn root_ctx() -> Self {
        Self { user_id: 0 }
    }

    pub const fn new(user_id: i32) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

impl Ctx {
    #[must_use]
    pub const fn user_id(&self) -> i32 {
        self.user_id
    }
}
