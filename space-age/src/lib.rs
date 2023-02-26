// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const SECONDS_PER_YEAR: f64 = 31557600.0;

macro_rules! planets {
    ( $($name:ident: $orbital_period:literal),* ) => {
        $(
            pub struct $name;

            impl Planet for $name {
                fn years_during(d: &Duration) -> f64 {
                    d.seconds as f64 / (SECONDS_PER_YEAR * $orbital_period)
                }
            }
        )*
    };
}

#[derive(Debug)]
pub struct Duration {
    pub seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            seconds: s
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

planets![
    Mercury: 0.2408467,
    Venus: 0.61519726,
    Earth: 1.0,
    Mars: 1.8808158,
    Jupiter: 11.862615,
    Saturn: 29.447498,
    Uranus: 84.016846,
    Neptune: 164.79132
];


