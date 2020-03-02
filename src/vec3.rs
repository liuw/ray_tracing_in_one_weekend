use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    v1: f32,
    v2: f32,
    v3: f32,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 { v1: self.v1 + other.v1,
               v2: self.v2 + other.v2,
               v3: self.v3 + other.v3 }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 { v1: self.v1 - other.v1,
               v2: self.v2 - other.v2,
               v3: self.v3 - other.v3 }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 { v1: self.v1 * other.v1,
               v2: self.v2 * other.v2,
               v3: self.v3 * other.v3 }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3 { v1: self.v1 * other,
               v2: self.v2 * other,
               v3: self.v3 * other }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { v1: other.v1 * self,
               v2: other.v2 * self,
               v3: other.v3 * self }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 { v1: self.v1 / other.v1,
               v2: self.v2 / other.v2,
               v3: self.v3 / other.v3 }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3 { v1: self.v1 / other,
               v2: self.v2 / other,
               v3: self.v3 / other }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3 { v1: self.v1 + other.v1,
                       v2: self.v2 + other.v2,
                       v3: self.v3 + other.v3 }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec3 { v1: self.v1 - other.v1,
                       v2: self.v2 - other.v2,
                       v3: self.v3 - other.v3 }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Vec3 { v1: self.v1 * other.v1,
                       v2: self.v2 * other.v2,
                       v3: self.v3 * other.v3 }
    }
}

impl ops::MulAssign<f32> for Vec3  {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3 { v1: self.v1 * other,
                       v2: self.v2 * other,
                       v3: self.v3 * other }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Vec3 { v1: self.v1 / other.v1,
                       v2: self.v2 / other.v2,
                       v3: self.v3 / other.v3 }
    }
}

impl ops::DivAssign<f32> for Vec3  {
    fn div_assign(&mut self, other: f32) {
        let k = 1.0 / other;

        *self = Vec3 { v1: self.v1 * k,
                       v2: self.v2 * k,
                       v3: self.v3 * k }
    }
}

impl Vec3 {

    pub fn new(v1: f32, v2: f32, v3: f32) -> Vec3 {
        Vec3 { v1: v1, v2: v2, v3: v3 }
    }

    pub fn x(self: &Vec3) ->  f32 {
        self.v1
    }
    pub fn y(self: &Vec3) ->  f32 {
        self.v2
    }
    pub fn z(self: &Vec3) ->  f32 {
        self.v3
    }
    pub fn r(self: &Vec3) ->  f32 {
        self.v1
    }
    pub fn g(self: &Vec3) ->  f32 {
        self.v2
    }
    pub fn b(self: &Vec3) ->  f32 {
        self.v3
    }

    pub fn length(self: &Vec3) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(self: &Vec3) -> f32 {
        self.v1*self.v1 + self.v2*self.v2 + self.v3*self.v3
    }

    pub fn make_unit_vector(self: &mut Vec3) {
        let k = 1.0 / self.length();
        self.v1 *= k;
        self.v2 *= k;
        self.v3 *= k;
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        a.v1 * b.v1 + a.v2 * b.v2 + a.v3 * b.v3
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 { v1: a.v2 * b.v3 - a.v3 * b.v2,
               v2: a.v3 * b.v1 - a.v1 * b.v3,
               v3: a.v1 * b.v2 - a.v2 * b.v1 }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        let l = v.length();
        v / l
    }
}
