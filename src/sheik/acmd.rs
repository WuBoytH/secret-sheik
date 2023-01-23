mod normals;
mod aerials;
mod throws;
mod specials;

pub fn install() {
    normals::install();
    aerials::install();
    throws::install();
    specials::install();
}