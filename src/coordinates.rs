use math::IntPoint;
use math::RealPoint;

#[derive(Debug, PartialEq)]
pub struct PlanePoint(pub f64, pub f64);

#[derive(Debug, PartialEq)]
pub struct CameraCoordinates(pub f64, pub f64);

#[derive(Debug, Eq, PartialEq)]
pub struct CanvasPixel(pub i64, pub i64);