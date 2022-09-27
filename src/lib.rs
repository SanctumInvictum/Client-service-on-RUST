extern crate kiss3d;
extern crate nalgebra as na;

pub mod corner_memory;

use kiss3d::scene::SceneNode;
use na::{Vector3, UnitQuaternion, Translation3, Vector};
use crate::corner_memory::CornerMemory;

pub fn standard_scale() -> Vector3<f32> {

    let scale = Vector3::new(0.1, 0.1, 0.1);
    scale

}

pub struct GroupToRotate<'a> {
    axis_type: &'a str,
    objects: Vec<&'a mut SceneNode>,
    corner_memory: &'a mut CornerMemory,
}

impl<'a> GroupToRotate<'_> {
    pub fn from(objects: Vec<&'a mut SceneNode>, axis_type: &'a str, corner_memory: &'a mut CornerMemory) -> GroupToRotate<'a> {
        GroupToRotate{
            axis_type,
            objects,
            corner_memory,
        }
    }

    pub fn rotate(&mut self, corner: f32) {

        let mut rotate_corner_a = 0.0;
        let mut rotate_corner_b = 0.0;
        let mut rotate_corner_c = 0.0;
        let mut rotate_corner_d = 0.0;

        match self.axis_type {
            "A" => rotate_corner_a = corner,
            "B" => rotate_corner_b = corner,
            "C" => rotate_corner_c = corner,
            "D" => rotate_corner_d = corner,
            _ => (),
        }

        self.start_position();

        //A-axis rotation---------------------------------------------------------------------------

        for i in 0..1 {
            self.objects[i].append_translation(&Translation3::new(0.0, -57.5, 0.0));
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, rotate_corner_a+self.corner_memory.axis_corner_a)));
            self.objects[i].append_translation(&Translation3::new(0.0, 57.5, 0.0));
        }
        self.corner_memory.axis_corner_a += rotate_corner_a;

        //B-axis rotation---------------------------------------------------------------------------

        for i in 0..2 {
            self.objects[i].append_translation(&Translation3::new(0.0, -37.5, 0.0));
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, rotate_corner_b+self.corner_memory.axis_corner_b)));
            self.objects[i].append_translation(&Translation3::new(0.0, 37.5, 0.0));
        }
        self.corner_memory.axis_corner_b += rotate_corner_b;

        //C-axis rotation---------------------------------------------------------------------------

        for i in 0..3 {
            self.objects[i].append_translation(&Translation3::new(0.0, -15.0, 0.0));
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, rotate_corner_c+self.corner_memory.axis_corner_c)));
            self.objects[i].append_translation(&Translation3::new(0.0, 15.0, 0.0));
        }
        self.corner_memory.axis_corner_c += rotate_corner_c;

        //D-axis rotation---------------------------------------------------------------------------

        for i in 0..4 {
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, rotate_corner_d+self.corner_memory.axis_corner_d, 0.0)));
        }
        self.corner_memory.axis_corner_d += rotate_corner_d;
    }

    fn start_position(&mut self) {

        let corner= self.corner_memory.axis_corner_d * (-1.0);
        for i in 0..=3 {
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, corner, 0.0)));
        }

        let corner= self.corner_memory.axis_corner_c * (-1.0);
        for i in 0..=2 {
            self.objects[i].append_translation(&Translation3::new(0.0, -15.0, 0.0));
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, corner)));
            self.objects[i].append_translation(&Translation3::new(0.0, 15.0, 0.0));
        }

        let corner = self.corner_memory.axis_corner_b * (-1.0);
        for i in 0..=1 {
            self.objects[i].append_translation(&Translation3::new(0.0, -37.5, 0.0));
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, corner)));
            self.objects[i].append_translation(&Translation3::new(0.0, 37.5, 0.0));
        }

        let corner = self.corner_memory.axis_corner_a * (-1.0);
        for i in 0..=0 {
            self.objects[i].append_translation(&Translation3::new(0.0, -57.5, 0.0));
            self.objects[i].append_rotation(&UnitQuaternion::new(Vector3::new(0.0, 0.0, corner)));
            self.objects[i].append_translation(&Translation3::new(0.0, 57.5, 0.0));
        }
    }
}
