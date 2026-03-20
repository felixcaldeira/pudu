use bitflags::bitflags;
use serde::{Serialize, Deserialize, Serializer};
use crate::AppError;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
    pub struct GradeFlags: u32 {
        const UNSET            = 0;
        const FIRST_GRADE      = 1 << 0;
        const SECOND_GRADE     = 1 << 1;
        const THIRD_GRADE      = 1 << 2;
        const FOURTH_GRADE     = 1 << 3;
        const FIFTH_GRADE      = 1 << 4;
        const SIXTH_GRADE      = 1 << 5;
        const SEVENTH_GRADE    = 1 << 6;
        const EIGHTH_GRADE     = 1 << 7;
        const NINTH_GRADE      = 1 << 8;
        const TENTH_GRADE      = 1 << 9;
        const ELEVENTH_GRADE   = 1 << 10;
        const TWELFTH_GRADE    = 1 << 11;
        const THIRTEENTH_GRADE = 1 << 12;
    }
}
impl GradeFlags {
    pub fn to_strings(self) -> Vec<&'static str> {
        const ALL: &[(GradeFlags, &str)] = &[
            (GradeFlags::FIRST_GRADE, "1. Klasse"),
            (GradeFlags::SECOND_GRADE, "2. Klasse"),
            (GradeFlags::THIRD_GRADE, "3. Klasse"),
            (GradeFlags::FOURTH_GRADE, "4. Klasse"),
            (GradeFlags::FIFTH_GRADE, "5. Klasse"),
            (GradeFlags::SIXTH_GRADE, "6. Klasse"),
            (GradeFlags::SEVENTH_GRADE, "7. Klasse"),
            (GradeFlags::EIGHTH_GRADE, "8. Klasse"),
            (GradeFlags::NINTH_GRADE, "9. Klasse"),
            (GradeFlags::TENTH_GRADE, "10. Klasse"),
            (GradeFlags::ELEVENTH_GRADE, "11. Klasse"),
            (GradeFlags::TWELFTH_GRADE, "12. Klasse"),
            (GradeFlags::THIRTEENTH_GRADE, "13. Klasse"),
        ];

        ALL.iter()
            .filter_map(|(flag, label)| {
                if self.contains(*flag) {
                    Some(*label)
                } else {
                    None
                }
            })
            .collect()
    }
}
impl Serialize for GradeFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}
impl From<u32> for GradeFlags {
    fn from(value: u32) -> Self {
        GradeFlags::from_bits_truncate(value)
    }
}