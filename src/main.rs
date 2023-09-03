use rand::Rng;
use plotters::prelude::*;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug)]
struct Point (f64, f64, f64);

// implementing a rotation matrix about the x-, y- and z-axis for Point struct
impl Point {
    fn rotate_x(&self, theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Point(
            self.0,
            self.1 * cos - self.2 * sin,
            self.1 * sin + self.2 * cos
        )
    }
    fn rotate_y(&self, theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Point(
            self.0 * cos + self.2 * sin,
            self.1,
            -self.0 * sin + self.2 * cos
        )
    }
    fn rotate_z(&self, theta: f64) -> Self {
        let sin = theta.to_radians().sin();
        let cos = theta.to_radians().cos();
        Point(
            self.0 * cos - self.1 * sin,
            self.0 * sin + self.1 * cos,
            self.2
        )
    }
}

struct Tetrahedron {
    p0: Point,
    p1: Point,
    p2: Point,
    p3: Point
}

impl Tetrahedron {
    fn get_points(&self) -> [&Point; 4] {
        [&self.p0, &self.p1, &self.p2, &self.p3]
    }

    // transformation matrix to convert points into barycentric coordinate system 
    fn barycentric_matrix(&self) -> Array2<f64> {
        array![
        [self.p1.0-self.p0.0, self.p2.0-self.p0.0, self.p3.0-self.p0.0],
        [self.p1.1-self.p0.1, self.p2.1-self.p0.1, self.p3.1-self.p0.1],
        [self.p1.2-self.p0.2, self.p2.2-self.p0.2, self.p3.2-self.p0.2]]
    }
}

fn random_point_on_sphere() -> Point {
    let mut start_vector = Point(0.0, 0.0, 1.0);
    
    // we rotate the point around all three axes for a uniform distribution
    start_vector = start_vector.rotate_x(rand::thread_rng().gen_range(0.0..360.0));
    start_vector = start_vector.rotate_y(rand::thread_rng().gen_range(0.0..360.0));
    start_vector = start_vector.rotate_z(rand::thread_rng().gen_range(0.0..360.0));

    start_vector    
}

fn tetrahedron_on_sphere() -> Tetrahedron {
    Tetrahedron {
        p0: random_point_on_sphere(),
        p1: random_point_on_sphere(),
        p2: random_point_on_sphere(),
        p3: random_point_on_sphere()
    }
}

fn includes_origin (tetra: &Tetrahedron) -> bool{
    let tetra_barycentric = tetra.barycentric_matrix();
    let origin_barycentric = array![-tetra.p0.0, -tetra.p0.1, -tetra.p0.2];

    // solving system of linear equations
    let x = tetra_barycentric.solve_into(origin_barycentric).unwrap();

    x[0] >= 0.0 && x[0] <= 1.0 && 
    x[1] >= 0.0 && x[1] <= 1.0 - x[0] &&
    x[2] >= 0.0 && x[2] <= 1.0 - x[0] - x[1]
}

fn main() {
    let mut inside = 0.0;
    for _ in 0..100_000 {
        let tetra = tetrahedron_on_sphere();
        if includes_origin(&tetra) { inside += 1.0}
    }
    println!("Out of 100,000 tetrahedrons, {} included the origin. That's {}%. \n", inside, inside/100_000.0 * 100.0);

    println!("Visualizing one of the tetrahedrons:");
    let tetra = tetrahedron_on_sphere();

    if includes_origin(&tetra) {
        println!("The origin is inside of the tetrahedron. \n");
    } else {
        println!("The origin is outside of the tetrahedron. \n");
    }
    println!("Plotting tetrahedron on graph...");
    
    // initializing the surface polygons of the tetrahedron for the 3d graph
    let tetra_points = tetra.get_points();
    let mut tetra_array = [(0.0, 0.0, 0.0); 4];
    for i in 0..tetra_array.len() {
        tetra_array[i] = (tetra_points[i].0, tetra_points[i].1, tetra_points[i].2);
    }
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
    
    // drawing coordinate grid and axes
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .margin(30)
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
    .unwrap();
    chart.configure_axes().draw().unwrap();

    // draws 128 frames for a full rotation of the tetrahedron 
    for i in 0..127 {
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
        chart.draw_series(tetra_array.iter().map(|point| Circle::new(*point, 3, &BLUE))).unwrap();
        chart.draw_series(tetra_surfaces.iter().map(|point| Polygon::new(*point, &BLUE.mix(0.2)))).unwrap();

        root.present().unwrap();
    }

    println!("Done! GIF saved in examples folder.");   
}