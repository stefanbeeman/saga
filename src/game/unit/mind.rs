use game::unit::Unit;

// In reality intelligence is not a "scale" that you can go "up", but it's
// a good enough approximation for now.
#[derive(Debug, Clone, Copy)]
pub enum Mind {
    Mindless, // Unit has no intelligence whatsoever and cannot have skills.
    Instinctual, // Unit operates on pre-programmed instinct like a robot, cannot increase skills.
    Conditional, // Unit can associate related stimuli, and can advance existing skills.
    Insightful, // Unit can learn by watching others, and can learn some skills.
    Symbolic, // Unit has the capacity to use langauge, and can learn most skills.
    Abstract, // Unit has abstract thought, and can learn all skills.
}

pub trait HasMind {
    fn mind(&self) -> Mind;
}

impl HasMind for Unit {
    fn mind(&self) -> Mind {
        return self.mind.clone();
    }
}
