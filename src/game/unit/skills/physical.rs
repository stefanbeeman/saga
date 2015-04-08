pub struct PhysicalSkills<T> {
    athletics: T,
    awareness: T,
    brawl: T,
    chirugry: T,
    escape_artist: T,
    hunt: T,
    navigation: T,
    palming: T,
    ride: T,
    running: T,
    stealth: T,
    swimming: T,
    survival: T,
}

pub trait HasPhysicalSkills {
    fn skill_athletics(&self) -> u32;
    fn skill_awareness(&self) -> u32;
    fn skill_brawl(&self) -> u32;
    fn skill_chirugry(&self) -> u32;
    fn skill_escape_artist(&self) -> u32;
    fn skill_hunt(&self) -> u32;
    fn skill_navigation(&self) -> u32;
    fn skill_palming(&self) -> u32;
    fn skill_ride(&self) -> u32;
    fn skill_running(&self) -> u32;
    fn skill_stealth(&self) -> u32;
    fn skill_swimming(&self) -> u32;
    fn skill_survival(&self) -> u32;
}
