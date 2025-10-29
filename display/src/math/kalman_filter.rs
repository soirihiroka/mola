use std::ops::{Add, Mul, Sub};

use bevy::math::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VelocityKalman<T> {
    // x
    pub position: T, // Estimated position
    pub velocity: T, // Estimated velocity

    // Covariance matrix P:
    // [ p_pos, p_pos_vel ]
    // [ p_pos_vel, p_vel ]
    pub p_pos: f32,
    pub p_pos_vel: f32,
    pub p_vel: f32,

    // Q Process noise variances
    pub position_noise: f32,
    pub velocity_noise: f32,
    // R (measurement noise)
    pub measurement_noise: f32,
}

impl<T> VelocityKalman<T>
where
    for<'a> &'a T: Sub<&'a T, Output = T>,
    T: Clone,
{
    pub fn new(position: T) -> Self {
        let p = position.clone();
        Self {
            position,
            velocity: (&p - &p),
            position_noise: 1.,
            velocity_noise: 3.,
            measurement_noise: 100.,

            p_pos: 5.0,
            p_pos_vel: 0.0,
            p_vel: 5.0,
        }
    }

    pub fn with_noises(
        position: T,
        position_noise: f32,
        velocity_noise: f32,
        measurement_noise: f32,
    ) -> Self {
        let p = position.clone();
        Self {
            position,
            velocity: (&p - &p),
            position_noise,
            velocity_noise,
            measurement_noise,

            p_pos: 5.0,
            p_pos_vel: 0.0,
            p_vel: 5.0,
        }
    }
}

impl<T> VelocityKalman<T>
where
    for<'a> &'a T: Add<&'a T, Output = T> + Sub<&'a T, Output = T> + Mul<f32, Output = T>,
{
    pub fn update(&mut self, measured_pos: &T, dt: f32) -> &T {
        //// Prediction Step
        // State prediction
        let predicted_pos = &self.position + &(&self.velocity * dt);
        let predicted_vel = &self.velocity;

        // Covariance prediction:
        //
        // P = A * P * A^T + Q
        //
        // A = [1 dt; 0 1]
        //
        let p_pos_new =
            self.p_pos + 2.0 * dt * self.p_pos_vel + dt * dt * self.p_vel + self.position_noise;
        let p_pos_vel_new = self.p_pos_vel + dt * self.p_vel;
        let p_vel_new = self.p_vel + self.velocity_noise;

        //// Update Step
        // Innovation
        let innovation = measured_pos - &predicted_pos;

        // Innovation covariance S
        let s = p_pos_new + self.measurement_noise;

        // Kalman gain
        let k_pos = p_pos_new / s;
        let k_vel = p_pos_vel_new / s;

        // State update
        self.position = &predicted_pos + &(&innovation * k_pos);
        self.velocity = predicted_vel + &(&innovation * k_vel);

        // Covariance update
        self.p_pos = (1.0 - k_pos) * p_pos_new;
        self.p_pos_vel = (1.0 - k_pos) * p_pos_vel_new;
        self.p_vel = p_vel_new - k_vel * p_pos_vel_new;

        &self.position
    }
}

impl<T> VelocityKalman<T> {
    pub fn get(&self) -> &T {
        &self.position
    }

    pub fn set_position_noise(&mut self, noise: f32) -> &mut Self {
        self.position_noise = noise;
        self
    }

    pub fn set_velocity_noise(&mut self, noise: f32) -> &mut Self {
        self.velocity_noise = noise;
        self
    }

    pub fn set_measurement_noise(&mut self, noise: f32) -> &mut Self {
        self.measurement_noise = noise;
        self
    }
}
