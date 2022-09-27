extern crate kiss3d;
extern crate nalgebra as na;

use client_server::corner_memory::CornerMemory;
use client_server::standard_scale;
use client_server::GroupToRotate;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::scene::Object;
use kiss3d::scene::SceneNode;

use std::path::Path;
use std::time::{Duration, Instant};
use std::{thread, time};
use std::fmt::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

use na::{Vector3, Point3, Isometry3, UnitQuaternion, Translation2, Translation3};

fn main() {

    //Creating window-------------------------------------------------------------------------------

    let mut win = Window::new("model");
    win.set_light(Light::StickToCamera);

    //Creating paths for objects--------------------------------------------------------------------

    let path1 = Path::new("objects/object1.obj");
    let path2 = Path::new("objects/object2.obj");
    let path3 = Path::new("objects/object3.obj");
    let path4 = Path::new("objects/object4.obj");
    let path5 = Path::new("objects/object5.obj");

    let dir = Path::new("objects/");

    //Creating camera-------------------------------------------------------------------------------

    let eye = Point3::new(75.0, 100.0, 75.0);
    let at = Point3::new(0.0, 40.0, 0.0);
    let mut arc_ball = ArcBall::new(eye, at);

    //Creating objects------------------------------------------------------------------------------

    let mut obj1 = win.add_obj(path1, dir, standard_scale());
    let mut obj2 = win.add_obj(path2, dir, standard_scale());
    let mut obj3 = win.add_obj(path3, dir, standard_scale());
    //let mut obj4 = win.add_obj(path4, dir, standard_scale());
    //let mut obj5 = win.add_obj(path5, dir, standard_scale());

    let mut top = win.add_group();
    top.add_obj(path4, dir, standard_scale());
    top.add_obj(path5, dir, standard_scale());

    //Creating CornerMemory object------------------------------------------------------------------

    let mut mem = CornerMemory::new();

    //Rendering-------------------------------------------------------------------------------------

    while true {

        let current_corners_file = File::open("current_corners.txt").unwrap();
        let buffered = BufReader::new(current_corners_file);

        let mut current_corners:Vec<f32> = Vec::new();

        for line in buffered.lines() {
            let output = match line {
                Ok(out) => out,
                Err(err) => panic!("{}", err),
            };
            if output.len() > 1 {}
            let number = match output.parse::<f32>() {
                Ok(out) => out,
                Err(err) => 0.0,
            };
            current_corners.push(number);
        }
        if current_corners.len() > 3 {

            let current_corner_a = current_corners[0];
            let current_corner_b = current_corners[1];
            let current_corner_c = current_corners[2];
            let current_corner_d = current_corners[3];

            if &mem.axis_corner_a != &current_corner_a {
                let corner = &current_corner_a - &mem.axis_corner_a;
                let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
                let mut group = GroupToRotate::from(vec, "A", &mut mem);
                group.rotate(corner);
            }

            if &mem.axis_corner_b != &current_corner_b {
                let corner = &current_corner_b - &mem.axis_corner_b;
                let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
                let mut group = GroupToRotate::from(vec, "B", &mut mem);
                group.rotate(corner);
            }

            if &mem.axis_corner_c != &current_corner_c {
                let corner = &current_corner_c - &mem.axis_corner_c;
                let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
                let mut group = GroupToRotate::from(vec, "C", &mut mem);
                group.rotate(corner);
            }

            if &mem.axis_corner_d != &current_corner_d {
                let corner = &current_corner_d - &mem.axis_corner_d;
                let mut vec = vec![&mut top, &mut obj3, &mut obj2, &mut obj1];
                let mut group = GroupToRotate::from(vec, "D", &mut mem);
                group.rotate(corner);
            }
        }

        win.render_with_camera(&mut arc_ball);

    }
}
