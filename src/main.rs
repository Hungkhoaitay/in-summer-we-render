#![allow(dead_code)]
mod tool;
mod lib;
mod traits;
mod sep;
mod sep_method;

use tool::renderer;
use lib::{ color, coordinate, points, ply_file, ply_dir };
use sep_method::sep_by_y_coord;

#[allow(unused_imports)]
use ply_dir::PlyDir;

fn main() {
    let path = "./plySource";
    //frames are declared as mut since the delta is stored internally  

    let mut data_1051 = ply_file::PlyFile::new(&(path.to_owned() + "/longdress_vox10_1051.ply")).read();
    let mut data_1053 = ply_file::PlyFile::new(&(path.to_owned() + "/longdress_vox10_1053.ply")).read();
    let (a, reference, marked_interpolated_frame) = data_1051.closest_with_ratio_average_points_recovery(data_1053, 0.495, 0.495, 0.01, 0.7); //sum of first 3 must equal 1

    a.render(); //comeplete interpolation and post processing
    reference.render(); //reference frame with unmapped points marked as green
    marked_interpolated_frame.render(); //interpolated frame with points surrounding cracks marked as red

}
