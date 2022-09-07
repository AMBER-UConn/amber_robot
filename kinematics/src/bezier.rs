use nalgebra as na;

struct ControlPoints {
    p0: na::Vector2<f32>,
    p1: na::Vector2<f32>,
    p2: na::Vector2<f32>,
    p3: na::Vector2<f32>,
}

impl ControlPoints {
    fn gen_curve(&self, t:f32) -> na::Vector2<f32> {
        let s0 = f32::powi(1.0-t, 3)*&self.p0;
        let s1 = 3.0*t*f32::powi(1.0-t, 2)*&self.p1;
        let s2 = 3.0*(1.0-t)*f32::powi(t, 2)*&self.p2;
        let s3 = f32::powi(t, 3)*&self.p3;

        let s = s0+s1+s2+s3;
        return s
    }
}