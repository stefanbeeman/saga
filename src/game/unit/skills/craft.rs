pub struct CraftSkills<T> {
    apothecary: T,
    armorer: T,
    atillitor: T,
    blacksmith: T,
    bowyer: T,
    brewer: T,
    carpentry: T,
    cobbler: T,
    coppersmith: T,
    engraving: T,
    jeweler: T,
    lapidary: T,
    locksmith: T,
    mason: T,
    plumbing: T,
    potter: T,
    tanner: T,
    taxidermy: T,
    vinter: T,
    weaponsmith: T,
    weaving: T,
    whitesmith: T,
}

pub trait HasCraftSkills {
    fn skill_apothecary(&self) -> u32;
    fn skill_armorer(&self) -> u32;
    fn skill_atillitor(&self) -> u32;
    fn skill_blacksmith(&self) -> u32;
    fn skill_bowyer(&self) -> u32;
    fn skill_brewer(&self) -> u32;
    fn skill_carpentry(&self) -> u32;
    fn skill_cobbler(&self) -> u32;
    fn skill_coppersmith(&self) -> u32;
    fn skill_engraving(&self) -> u32;
    fn skill_jeweler(&self) -> u32;
    fn skill_lapidary(&self) -> u32;
    fn skill_locksmith(&self) -> u32;
    fn skill_mason(&self) -> u32;
    fn skill_plumbing(&self) -> u32;
    fn skill_potter(&self) -> u32;
    fn skill_tanner(&self) -> u32;
    fn skill_taxidermy(&self) -> u32;
    fn skill_vinter(&self) -> u32;
    fn skill_weaponsmith(&self) -> u32;
    fn skill_weaving(&self) -> u32;
    fn skill_whitesmith(&self) -> u32;
}
