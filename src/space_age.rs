fn main() {
    let seconds = 189839836;
    let duration = Duration::from(seconds);
    let output = Venus::years_during(&duration);

    println!("{}", output); // 9.78
}

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            seconds: s
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

/**
 * Get years for planet
 * 
 * @param {f64} duration - Duration in seconds
 * @param {f64} orbital - Orbital period in Earth years
 */
fn get_years_for_planet(duration: u64, orbital: f64) -> f64 {
    let orbital = 1 as f64 / orbital;
    let result = (duration as f64 / 31_557_600.00) * orbital;

    (result * 100.0).round() / 100.0
}

impl Planet for Mercury {
    // Orbital period in Earth years
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 0.2408467)
    }
}
impl Planet for Venus {
    // Orbital period in Earth years
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 0.61519726)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 1.0)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 1.8808158)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 11.862615)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 29.447498)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 84.016846)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        get_years_for_planet(d.seconds, 164.79132)
    }
}
