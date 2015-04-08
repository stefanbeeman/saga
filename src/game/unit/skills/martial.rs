pub struct MartialSkills<T> {
    archery: T,
    armor: T,
    artillery: T,
    crossbow: T,
    cut_and_thrust: T,
    dagger: T,
    firearm: T,
    greatsword: T,
    longsword: T,
    mass_weapon: T,
    polearm: T,
    sling: T,
    spear: T,
    sword: T,
    thrown: T,
}

pub trait HasMartialSkills {
    fn skill_archery(&self) -> u32;
    fn skill_armor(&self) -> u32;
    fn skill_artillery(&self) -> u32;
    fn skill_crossbow(&self) -> u32;
    fn skill_cut_and_thrust(&self) -> u32;
    fn skill_dagger(&self) -> u32;
    fn skill_firearm(&self) -> u32;
    fn skill_greatsword(&self) -> u32;
    fn skill_longsword(&self) -> u32;
    fn skill_mass_weapon(&self) -> u32;
    fn skill_polearm(&self) -> u32;
    fn skill_sling(&self) -> u32;
    fn skill_spear(&self) -> u32;
    fn skill_sword(&self) -> u32;
    fn skill_thrown(&self) -> u32;
}
