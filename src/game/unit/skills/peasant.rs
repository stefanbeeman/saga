pub struct PeasantSkills<T> {
    animal_husbandry: T,
    baking: T,
    bird_husbandry: T,
    butchery: T,
    cooking: T,
    cooper: T,
    farming: T,
    fishing: T,
    insect_husbandry: T,
    mending: T,
    mining: T,
    prospecting: T,
    sewing: T,
}

pub trait HasPeasantSkills {
    fn skill_animal_husbandry(&self) -> u32;
    fn skill_baking(&self) -> u32;
    fn skill_bird_husbandry(&self) -> u32;
    fn skill_butchery(&self) -> u32;
    fn skill_cooking(&self) -> u32;
    fn skill_cooper(&self) -> u32;
    fn skill_farming(&self) -> u32;
    fn skill_fishing(&self) -> u32;
    fn skill_insect_husbandry(&self) -> u32;
    fn skill_mending(&self) -> u32;
    fn skill_mining(&self) -> u32;
    fn skill_prospecting(&self) -> u32;
    fn skill_sewing(&self) -> u32;
}
