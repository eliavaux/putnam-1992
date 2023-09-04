use rand::Rng;
use plotters::prelude::*;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

// implementing a rotation matrix about the x-, y- and z-axis for Point struct
impl Point {
    fn rotate_x(&self, theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Point{
            x: self.x,
            y: self.y * cos - self.z * sin,
            z: self.y * sin + self.z * cos
        }
    }
    fn rotate_y(&self, theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Point {
            x: self.x * cos + self.z * sin,
            y: self.y,
            z: -self.x * sin + self.z * cos
        }
    }
    fn rotate_z(&self, theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Point {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
            z: self.z
        }
    }
}

struct Tetrahedron (Point, Point, Point, Point);

// transformation matrix to convert points into barycentric coordinate system 
impl Tetrahedron {
    fn barycentric_matrix(&self) -> Array2<f64> {
        array![
            [self.1.x - self.0.x, self.2.x - self.0.x, self.3.x - self.0.x],
            [self.1.y - self.0.y, self.2.y - self.0.y, self.3.y - self.0.y],
            [self.1.z - self.0.z, self.2.z - self.0.z, self.3.z - self.0.z]
        ]
    }
}

fn random_point_on_sphere() -> Point {
    let mut start_vector = Point{x: 0.0, y: 0.0, z: 1.0};
    
    // we rotate the point around all three axes for a uniform distribution
    start_vector = start_vector.rotate_x(rand::thread_rng().gen_range(0.0..360.0));
    start_vector = start_vector.rotate_y(rand::thread_rng().gen_range(0.0..360.0));
    start_vector = start_vector.rotate_z(rand::thread_rng().gen_range(0.0..360.0));

    start_vector    
}

fn tetrahedron_on_sphere() -> Tetrahedron {
    Tetrahedron (random_point_on_sphere(), random_point_on_sphere(), random_point_on_sphere(), random_point_on_sphere())
}

fn includes_origin (tetra: &Tetrahedron) -> bool{
    let tetra_barycentric = tetra.barycentric_matrix();
    let origin_barycentric = array![-tetra.0.x, -tetra.0.y, -tetra.0.z];

    // solving system of linear equations
    let x = tetra_barycentric.solve_into(origin_barycentric).unwrap();

    // conditions to check if the origin is inside the tetrahedron
    x[0] >= 0.0 && x[0] <= 1.0 && 
    x[1] >= 0.0 && x[1] <= 1.0 - x[0] &&
    x[2] >= 0.0 && x[2] <= 1.0 - x[0] - x[1]
}

fn main() {
    let precision = 100_000;

    let mut inside = 0;
    for _ in 0..precision {
        let tetra = tetrahedron_on_sphere();
        if includes_origin(&tetra) {inside += 1}
    }
    println!("Out of {precision} tetrahedrons, {inside} included the origin. That's {:.3}%. \n", (inside as f64/precision as f64) * 100.0);


    println!("Focussing on one of the tetrahedrons:");

    let tetra = tetrahedron_on_sphere();
    if includes_origin(&tetra) {
        println!("The origin is inside of the tetrahedron. \n");
    } else {
        println!("The origin is outside of the tetrahedron. \n");
    }
    println!("Plotting tetrahedron on graph...");
    
    // initializing the surfaces of the tetrahedron for the 3d graph
    let tetra_array = [
        (tetra.0.x, tetra.0.y, tetra.0.z),
        (tetra.1.x, tetra.1.y, tetra.1.z),
        (tetra.2.x, tetra.2.y, tetra.2.z),
        (tetra.3.x, tetra.3.y, tetra.3.z),
    ];
    let tetra_surfaces = [
        [tetra_array[0], tetra_array[1], tetra_array[2]],
        [tetra_array[0], tetra_array[1], tetra_array[3]],
        [tetra_array[0], tetra_array[2], tetra_array[3]],
        [tetra_array[1], tetra_array[2], tetra_array[3]]
    ];

    // root of 3D graph
    let root = BitMapBackend::gif(
        "examples/example-5.gif",
        (512, 512),
        30
    ).unwrap().into_drawing_area();
    
    // drawing 128 frames for a full rotation of the tetrahedron 
    for i in 0..127 {

        // drawing coordinate grid and axes
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .margin(30)
            .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
        .unwrap();
        chart.configure_axes().draw().unwrap();

        // rotating the scene around the origin
        chart.with_projection(|mut pb| {
            pb.pitch = 0.0;
            pb.yaw = 0.05 * i as f64;
            pb.scale = 1.0;
            pb.into_matrix()
        });

        // drawing origin and tetrahedron 
        chart.draw_series((0..1).map(|_| Circle::new((0.0, 0.0, 0.0), 3, &RED))).unwrap();
        chart.draw_series(tetra_array.iter().map(|&point| Circle::new(point, 3, &BLUE))).unwrap();
        chart.draw_series(tetra_surfaces.iter().map(|&point| Polygon::new(point, &BLUE.mix(0.2)))).unwrap();

        root.present().unwrap();
    }

    println!("Done! GIF saved in examples folder.");   
}