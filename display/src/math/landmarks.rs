use crate::api::pose_api::LandmarkJson;
use bevy::math::Vec3;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};
use strum::EnumCount;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Landmark {
    pub position: Vec3,
    pub visibility: f32,
}

impl Add for Landmark {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            position: self.position + other.position,
            visibility: self.visibility + other.visibility,
        }
    }
}

impl Sub for Landmark {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            position: self.position - rhs.position,
            visibility: self.visibility - rhs.visibility,
        }
    }
}

impl Add for &Landmark {
    type Output = Landmark;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            position: self.position + other.position,
            visibility: self.visibility + other.visibility,
        }
    }
}

impl Mul<f32> for Landmark {
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self {
            position: self.position * other,
            visibility: self.visibility * other,
        }
    }
}

impl Mul<f32> for &Landmark {
    type Output = Landmark;
    fn mul(self, other: f32) -> Self::Output {
        Landmark {
            position: self.position * other,
            visibility: self.visibility * other,
        }
    }
}

impl From<LandmarkJson> for Landmark {
    fn from(lj: LandmarkJson) -> Self {
        Landmark {
            position: Vec3::new(-lj.x, -lj.y, lj.z),
            visibility: lj.visibility,
        }
    }
}

impl From<&LandmarkJson> for Landmark {
    fn from(lj: &LandmarkJson) -> Self {
        Landmark {
            position: Vec3::new(-lj.x, -lj.y, lj.z),
            visibility: lj.visibility,
        }
    }
}

pub trait LandmarkIndex: EnumCount {
    fn as_index(self) -> usize;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Landmarks<Index: LandmarkIndex> {
    pub data: Vec<Landmark>,
    _phantom: std::marker::PhantomData<Index>,
}

impl<'a, Index: LandmarkIndex> Add for &'a Landmarks<Index> {
    type Output = Landmarks<Index>;

    fn add(self, other: Self) -> Self::Output {
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            new_data.push(self.data[i] + other.data[i]);
        }
        Landmarks {
            data: new_data,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Index: LandmarkIndex> Sub for &'a Landmarks<Index> {
    type Output = Landmarks<Index>;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            new_data.push(self.data[i] - other.data[i]);
        }
        Landmarks {
            data: new_data,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Index: LandmarkIndex> Mul<f32> for &'a Landmarks<Index> {
    type Output = Landmarks<Index>;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in 0..self.data.len() {
            new_data.push(self.data[i] * rhs);
        }
        Landmarks {
            data: new_data,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LandmarksError {
    #[error("Expected {expected} landmarks, got {actual}")]
    IncorrectLength { expected: usize, actual: usize },
    #[error("Unable to find any keypoint")]
    NoKeyPoints,
}

impl<Index: LandmarkIndex> TryFrom<&[LandmarkJson]> for Landmarks<Index> {
    type Error = LandmarksError;
    fn try_from(value: &[LandmarkJson]) -> Result<Self, Self::Error> {
        if value.len() != Index::COUNT {
            return Err(LandmarksError::IncorrectLength {
                expected: Index::COUNT,
                actual: value.len(),
            });
        }
        Ok(Landmarks {
            data: value.into_iter().map(Landmark::from).collect::<Vec<_>>(),
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<Index: LandmarkIndex> TryFrom<Vec<LandmarkJson>> for Landmarks<Index> {
    type Error = LandmarksError;
    fn try_from(value: Vec<LandmarkJson>) -> Result<Self, Self::Error> {
        if value.len() != Index::COUNT {
            return Err(LandmarksError::IncorrectLength {
                expected: Index::COUNT,
                actual: value.len(),
            });
        }
        Ok(Landmarks {
            data: value.into_iter().map(Landmark::from).collect::<Vec<_>>(),
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<Index: LandmarkIndex> TryFrom<&Vec<LandmarkJson>> for Landmarks<Index> {
    type Error = LandmarksError;
    fn try_from(value: &Vec<LandmarkJson>) -> Result<Self, Self::Error> {
        if value.len() != Index::COUNT {
            return Err(LandmarksError::IncorrectLength {
                expected: Index::COUNT,
                actual: value.len(),
            });
        }
        Ok(Landmarks {
            data: value.into_iter().map(Landmark::from).collect::<Vec<_>>(),
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<Index: LandmarkIndex> std::ops::Index<Index> for Landmarks<Index> {
    type Output = Landmark;

    fn index(&self, index: Index) -> &Self::Output {
        &self.data[index.as_index()]
    }
}

impl<Index: LandmarkIndex> Add for Landmarks<Index> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(left, right)| left + right)
                .collect(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Index: LandmarkIndex> Mul<f32> for Landmarks<Index>
where
    Index: Copy + Into<usize>,
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        // let data: [Landmark; N] = std::array::from_fn(|i| &self.data[i] * rhs);
        Self {
            data: self.data.into_iter().map(|v| v * rhs).collect(),
            _phantom: std::marker::PhantomData,
        }
    }
}
