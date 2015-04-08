pub struct ArcaneSkills<T> {
    concentration: T,
    divine_lore: T,
    fairy_lore: T,
    finesse: T,
    hermetic_theory: T,
    infernal_lore: T,
    magic_lore: T,
    magic_sense: T,
    parma_magica: T,
    penetration: T,
}

pub trait HasArcaneSkills {
    fn skill_concentration(&self) -> u32;
    fn skill_divine_lore(&self) -> u32;
    fn skill_fairy_lore(&self) -> u32;
    fn skill_finesse(&self) -> u32;
    fn skill_hermetic_theory(&self) -> u32;
    fn skill_infernal_lore(&self) -> u32;
    fn skill_magic_lore(&self) -> u32;
    fn skill_magic_sense(&self) -> u32;
    fn skill_parma_magica(&self) -> u32;
    fn skill_penetration(&self) -> u32;
}
