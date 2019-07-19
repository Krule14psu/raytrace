use nalgebra::Vector3;
use ncollide3d::math::Point;
use ncollide3d::shape::*;
use raytrace::*;

fn main() {
    let xsize = 400;
    let ysize = 400;
    let view = Viewport::new(
        Point::new(0.0, 4.0, 0.5),
        Vector3::new(0.0, -1.0, 0.5 / 4.0),
        Vector3::new(0.0, 0.0, 1.0),
        std::f64::consts::PI / 2.0,
        (xsize, ysize),
    );
    // Note on building polyhedrons:
    // The position of the polyhedron must be noted as an isometry in 3d
    // The RayCast objects needs to be implemented as an f64
    /*
    let cube: Polyhedron<Cuboid<f64>> = Polyhedron::new(
        Cuboid::new(Vector3::new(1.0, 1.0, 1.0)),
        image::Rgb([0, 0, 0]),
    );
    */
    let sphere: Polyhedron<Ball<f64>> =
        Polyhedron::new(Ball::new(2.0), image::Rgb([125, 125, 125]));
    // This is an example scene
    let scene: Scene<Ball<f64>> = Scene::new(vec![sphere], view, image::Rgb([255, 255, 255]));
    scene.render("output.png".to_string());
}
