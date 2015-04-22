pub mod body;
pub mod mind;
pub mod skills;
pub mod stats;
pub mod sex;
pub mod size;

pub struct Unit {
    name: String,
    species: String,
    size: size::Size,
    body: body::Body,
    mind: mind::Mind,
    age: f64,
    sex: sex::Sex,
    stats: stats::Stats<f64>,
    skills: skills::Skills<f64>,
}

impl Unit {
    new() -> Self {
        return Unit {
            name: "Generic Unit".to_string(),
            species: "Human".to_string(),
            size: size::Size::Meduim,
            body: body::Body::Human,
            mind: mind::Mind::Abstract,
            age: 20.0,
            sex: sex::Sex::Male,
        }
    }
}
