use game::unit::Unit;

#[derive(Debug, Clone, Copy)]
pub struct Core<T> {
    brawn: T,
    cunning: T,
    curiosity: T,
    daring: T,
    heart: T,
    tenacity: T,
}
