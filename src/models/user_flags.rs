use bitflags::bitflags;
use serde::{Deserialize, Serialize, Serializer};
use crate::AppError;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
    pub struct UserFlags: u32 {
        const DEFAULT           = 0;
        const MANAGE_NEWSLETTER = 1 << 0; // 0000_0001 / 1 
        const MANAGE_MODULES    = 1 << 1; // 0000_0010 / 2
        const MANAGE_ARTICLES   = 1 << 2; // 0000_0100 / 4
        const MANAGE_WORKSHOPS  = 1 << 3; // 0000_1000 / 8
        const MANAGE_FILES      = 1 << 4; // 0001_0000 / 16
        const MANAGE_IMAGES     = 1 << 5; // 0010_0000 / 32
        const MANAGE_USERS      = 1 << 6; // 0100_0000 / 64

        const ADMIN             = Self::MANAGE_NEWSLETTER.bits()
                                | Self::MANAGE_MODULES.bits()
                                | Self::MANAGE_ARTICLES.bits()
                                | Self::MANAGE_WORKSHOPS.bits()
                                | Self::MANAGE_FILES.bits()
                                | Self::MANAGE_IMAGES.bits()
                                | Self::MANAGE_USERS.bits();
    }
}
// {% if user and has_flag(flags=user.flags, bit=64) %}
impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}
impl From<u32> for UserFlags {
    fn from(value: u32) -> Self {
        UserFlags::from_bits_truncate(value)
    }
}




// let perms = UserFlags::from_bits_truncate(...);
// perms.contains(UserFlags::ADMIN);
// contains(X) → (perms & X) == X — all bits in X must be set
// intersects(X) → (perms & X) != 0 — at least one bit in X must be set