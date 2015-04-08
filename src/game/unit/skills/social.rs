pub struct SocialSkills<T> {
    begging: T,
    charm: T,
    carouse: T,
    etiquette: T,
    folk_ken: T,
    guile: T,
    impersonation: T,
    interrogation: T,
    intimidation: T,
    intruige: T,
    leadership: T,
    negotiation: T,
    oratory: T,
    teaching: T,
}

pub trait HasSocialSkills {
    fn skill_begging(&self) -> u32;
    fn skill_charm(&self) -> u32;
    fn skill_carouse(&self) -> u32;
    fn skill_etiquette(&self) -> u32;
    fn skill_folk_ken(&self) -> u32;
    fn skill_guile(&self) -> u32;
    fn skill_impersonation(&self) -> u32;
    fn skill_interrogation(&self) -> u32;
    fn skill_intimidation(&self) -> u32;
    fn skill_intruige(&self) -> u32;
    fn skill_leadership(&self) -> u32;
    fn skill_negotiation(&self) -> u32;
    fn skill_oratory(&self) -> u32;
    fn skill_teaching(&self) -> u32;
}
