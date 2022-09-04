#[cfg(test)]
mod inverse_kinematics;
mod tests {
    
    use rand::Rng;

    #[test]
    fn test_inverse_k() {
        let mut rng = rand::thread_rng();
        let theta = inverse_kinematics::inverse_ik::<f32>(1.414, 0.899);
        let coordinates = forward_kinematics::forward_ik::<f32>(theta.x, theta.y);

    }
}