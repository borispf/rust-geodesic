#[allow(non_snake_case, dead_code, non_camel_case_types)]
mod geodesic;

pub struct Geodesic {
    g: geodesic::Struct_geod_geodesic,
}

impl Geodesic {
    pub fn new(a: f64, f: f64) -> Geodesic {
        let mut g = geodesic::Struct_geod_geodesic::default();
        unsafe {
            geodesic::geod_init(&mut g, a, f);
        }
        Geodesic { g: g }
    }
    pub fn wgs84() -> Geodesic {
        Geodesic::new(6378137.0, 1.0 / 298.257223563)
    }
    pub fn direct(&self, lat1: f64, lon1: f64, azi1: f64, s12: f64) -> (f64, f64, f64) {
        let mut lat2 = std::f64::NAN;
        let mut lon2 = std::f64::NAN;
        let mut azi2 = std::f64::NAN;
        unsafe {
            geodesic::geod_direct(&self.g,
                                  lat1,
                                  lon1,
                                  azi1,
                                  s12,
                                  &mut lat2,
                                  &mut lon2,
                                  &mut azi2);
        }
        (lat2, lon2, azi2)
    }
    pub fn inverse(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64, f64) {
        let mut s12 = std::f64::NAN;
        let mut azi1 = std::f64::NAN;
        let mut azi2 = std::f64::NAN;
        unsafe {
            geodesic::geod_inverse(&self.g,
                                   lat1,
                                   lon1,
                                   lat2,
                                   lon2,
                                   &mut s12,
                                   &mut azi1,
                                   &mut azi2);
        }
        (s12, azi1, azi2)
    }
}
