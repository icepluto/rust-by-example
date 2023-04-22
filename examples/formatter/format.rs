use std::fmt::Display;

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}
impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lat_c = if &self.lat >= &0.0 { 'N' } else { 'S' };
        let lon_c = if &self.lon >= &0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}:{:.3}° {}, {:.3}° {}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

fn main() {
    let wuhan = City {
        name: "wuhan",
        lat: 0.00012312,
        lon: 10.2213123,
    };
    println!("{}", wuhan);
    for city in [
        City {
            name: "xi'an",
            lat: 12.3212312,
            lon: -13.121451,
        },
        City {
            name: "nanjing",
            lat: 8.8213123,
            lon: -9.21123,
        },
        City {
            name: "shanghai",
            lat: 1.21341,
            lon: -9.24123,
        },
        City {
            name: "shenzhen",
            lat: 6.82312,
            lon: -1.2413223,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
}
