pub struct ArtisticSkills<T> {
    dance: T,
    painting: T,
    poetry: T,
    sculpture: T,
    sing: T,
}

pub trait HasArtisticSkills {
    fn skill_dance() -> u32;
    fn skill_painting() -> u32;
    fn skill_poetry() -> u32;
    fn skill_sculpture() -> u32;
    fn skill_sing() -> u32;
}
