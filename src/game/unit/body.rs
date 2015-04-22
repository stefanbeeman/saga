use game::unit::Unit;

#[derive(Debug, Clone, Copy)]
pub enum Body {
    Beast,
    BeastWinged,
    Bird,
    Blob,
    Centaur,
    Cherub,
    Crab,
    Elephant,
    Eye,
    Fish,
    Formicid,
    Human,
    HumanTail,
    HumanWinged,
    HumanWingedTail,
    Hydra(u32),
    Insect,
    InsectWinged,
    Merfolk,
    Naga,
    NagaWinged,
    Octopus,
    Scorpion,
    Snake,
    Spider,
    Spidertaur,
    Spirit,
    Tentacle,
    Tentacles,
    Thing,
    Weird,
}

pub trait HasBody {
    fn body(&self) -> Body;
}

impl HasBody for Unit {
    fn body(&self) -> Body {
        return self.body.clone();
    }
}
