#[derive(Debug, Clone, Copy)]
pub struct Skills<T> {
    // academic skills
    artes_liberales: T,
    law: T,
    medicine: T,
    philosophaie: T,
    theology: T,
    // arcane skills
    code_of_hermes: T,
    divine_lore: T,
    fairy_lore: T,
    finesse: T,
    hermetic_theory: T,
    infernal_lore: T,
    magic_lore: T,
    parma_magica: T,
    penetration: T,
    // martial skills
    archery: T,
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
    // physical skills
    athletics: T,
    awareness: T,
    brawl: T,
    carouse: T,
    concentration: T,
    dungeon_lore: T,
    escape_artist: T,
    hunt: T,
    palming: T,
    stealth: T,
    survival: T,
    swim: T,
    // social skills
    animal_handling: T,
    bargain: T,
    charm: T,
    empathy: T,
    etiquette: T,
    guile: T,
    intrigue: T,
    leadership: T,
    teaching: T,
}
