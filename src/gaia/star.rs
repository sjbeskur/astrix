use std::f64::consts::PI;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct Star {
    #[serde(rename = "ra")]
    ra_deg: f64,  

    #[serde(rename = "dec")]
    dec_deg: f64,

    #[serde(rename = "b1mag")]
    mag: f32,
}


impl Star {
    pub fn new(ra_deg: f64, dec_deg: f64, mag: f32) -> Self {
        Self { ra_deg, dec_deg, mag }
    }

    pub fn angular_separation(&self, other_star: &Star) -> f64 {
        // Calculate the angular separation between the stars
        //let sep_deg = ra1 * 15.0 / 180.0 * PI, dec1 / 180.0 * PI, ra2 * 15.0 / 180.0 * PI, dec2 / 180.0 * PI;
        let ra1 = self.ra_deg.to_radians();
        let dec1 = self.dec_deg.to_radians();
        
        let ra2 = other_star.ra_deg.to_radians();
        let dec2 = other_star.dec_deg.to_radians();

        // Calculate the cosine of the angular separation
        let cos_theta = dec1.sin() * dec2.sin() + dec1.cos() * dec2.cos() * (ra1 - ra2).cos();

        //cos⁻¹(sin(54.9253)sin(49.3131) + cos(54.9253)cos(49.3131)cos(206.8846 - 200.9812)) = 5.750 degrees

        // sep = math.acos(
        //     math.sin(delta1) * math.sin(delta2) + math.cos(delta1)* math.cos(delta2) * math.cos(alpha1 - alpha2)
        // )
        //sep_degrees = math.degrees(sep)

        // Calculate the angular separation in radians
        let theta = cos_theta.acos();

        // Convert the angular separation from radians to degrees
        let sep_deg = theta.to_degrees();

        sep_deg
    }
}