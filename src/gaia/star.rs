use super::{Point3};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
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
		Self {
			ra_deg,
			dec_deg,
			mag,
		}
	}

	// conversion for hours, minutes, seconds.. I think
	// let sep_deg = ra1 * 15.0 / 180.0 * PI, dec1 / 180.0 * PI, ra2 * 15.0 / 180.0 * PI, dec2 / 180.0 * PI;
	pub fn angular_separation(&self, other_star: &Star) -> f64 {
		// Calculate the angular separation between the stars
		let ra1 = self.ra_deg.to_radians();
		let dec1 = self.dec_deg.to_radians();

		let ra2 = other_star.ra_deg.to_radians();
		let dec2 = other_star.dec_deg.to_radians();

		// Calculate the cosine of the angular separation
		let cos_theta = dec1.sin() * dec2.sin() + dec1.cos() * dec2.cos() * (ra1 - ra2).cos();

		// Calculate the angular separation in radians
		let theta = cos_theta.acos();

		// Convert the angular separation from radians to degrees
		theta.to_degrees()
	}


	/// RA and Dec represents the spherical coordinates of a celestial object. 
	/// in order to convert them to Cartesian coordinates (x, y, z) using the following formulas:
	///
	/// x = cos(Dec) * cos(RA) 
	/// y = cos(Dec) * sin(RA) 
	/// z = sin(Dec)
	/// 
	pub fn to_cartesian(&self) -> Point3 {
		let x = self.dec_deg.to_radians().cos() * self.ra_deg.to_radians().cos();
		let y = self.dec_deg.to_radians().cos() * self.ra_deg.to_radians().sin();
		let z = self.dec_deg.to_radians().sin();
		Point3::new(x,y,z)
	}

	/// If we have the Cartesian coordinates (x, y, z), we can convert them to spherical coordinates (r, θ, φ) 
	/// using the following formulas:
	///
	/// r = sqrt(x^2 + y^2 + z^2)
	/// θ = arccos(z/r)
    /// φ = atan2(y, x)
	pub fn to_spherical(&self) {
		todo!("convert to spherical coordinates using the formula described above")
	}



}
