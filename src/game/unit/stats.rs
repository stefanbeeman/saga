pub struct RootStats<T> {
    brawn: T,
    daring: T,
    tenacity: T,
    heart: T,
    sagacity: T,
    cunning: T,
}

pub struct DerivedStats<T> {
    reflex: T,
    aim: T,
    knockdown: T,
    knockout: T,
    movement: T,
    power: T,
}

pub struct SpecialStats<T> {
    warping: T,
    decrepitude: T,
}

pub trait HasStats {
    // Root stats
    fn stat_brawn(&self) -> u32;
    fn stat_daring(&self) -> u32;
    fn stat_tenacity(&self) -> u32;
    fn stat_heart(&self) -> u32;
    fn stat_sagacity(&self) -> u32;
    fn stat_cunning(&self) -> u32;
    // Derived stats
    fn stat_reflex(&self) -> u32 {
        (self.stat_daring() + self.stat_cunning())/2u32
    }
    fn stat_aim(&self) -> u32 {
        (self.stat_sagacity() + self.stat_cunning())/2u32
    }
    fn stat_knockdown(&self) -> u32 {
        (self.stat_brawn() + self.stat_daring())/2u32
    }
    fn stat_knockout(&self) -> u32 {
        (self.stat_brawn() + self.stat_tenacity())/2u32
    }
    fn stat_movement(&self) -> u32 {
        (self.stat_brawn() + self.stat_daring() + self.stat_cunning())/2u32
    }
    fn stat_power(&self, bonus: u32) -> u32 {
        (self.stat_tenacity() + self.stat_heart() + self.stat_sagacity() + bonus)/3u32
    }
    // Special stats
    fn stat_warping(&self) -> u32;
    fn stat_decrepitude(&self) -> u32;
}
