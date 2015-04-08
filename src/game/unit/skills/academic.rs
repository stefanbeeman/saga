pub struct AcademicSkills<T> {
    artes_liberales: T,
    caligraphy: T,
    cartography: T,
    engineering: T,
    heraldry: T,
    history: T,
    law: T,
    medicine: T,
    playwright: T,
    philosophaie: T,
    theology: T,
}

pub trait HasAcademicSkills {
    fn skill_artes_liberales(&self) -> u32;
    fn skill_caligraphy(&self) -> u32;
    fn skill_cartography(&self) -> u32;
    fn skill_engineering(&self) -> u32;
    fn skill_heraldry(&self) -> u32;
    fn skill_history(&self) -> u32;
    fn skill_law(&self) -> u32;
    fn skill_medicine(&self) -> u32;
    fn skill_playwright(&self) -> u32;
    fn skill_philosophaie(&self) -> u32;
    fn skill_theology(&self) -> u32;
}
