use game::unit::Unit;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Fine, // size of an insect
    Diminutive, // size of a rodent
    Tiny, // size of a cat
    Small, // size of a dog
    Medium, // size of a person
    Large, // size of a horse
    Huge, // size of an elephant
    Gargantuan, // size of a killer whale
    Colossal, // size of a blue whale
}
