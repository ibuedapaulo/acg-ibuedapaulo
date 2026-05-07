mod parse_svg;

/// Return the 2D scalar cross product of vectors `a` and `b`.
fn cross(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    a[0] * b[1] - a[1] * b[0]
}

/// Return the 2D dot product of vectors `a` and `b`.
fn dot(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}

/// Signed area of the triangle (p0, p1, p2) in a left-handed coordinate system
/// (pixel y-axis pointing downward).
fn area(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2]) -> f32 {
    let v01 = [p1[0] - p0[0], p1[1] - p0[1]];
    let v02 = [p2[0] - p0[0], p2[1] - p0[1]];
    // left-handed: y increases downward
    -0.5 * cross(&v01, &v02)
}

/// Find real roots of a quadratic equation within the interval [0, 1].
fn roots_of_quadratic_function_between_0_1(a: f32, b: f32, c: f32) -> Vec<f32> {
    // comment out below
    vec!()

    // implement this function

    // end of edit
}

/// Find real roots of a cubic equation within the interval [0, 1].
fn roots_of_cubic_function_between_0_1(a: f32, b: f32, c: f32, d: f32) -> Vec<f32> {

    if a == 0.0 { // this is quadratic function
        return roots_of_quadratic_function_between_0_1(b, c, d);
    }

    // Monotonic segments are split by derivative roots: 3ax^2 + 2bx + c = 0.
    let xs = {
        let mut xs = vec![0.0f32];
        for r in roots_of_quadratic_function_between_0_1(3.0 * a, 2.0 * b, c) {
            xs.push(r);
        }
        xs.push(1.0f32);
        xs
    };

    let eval = |x: f32| -> f32 { ((a * x + b) * x + c) * x + d }; // Horner's method

    // Find one root in each monotonic interval via bisection when signs differ.
    let mut roots: Vec<f32> = Vec::new();
    for w in xs.windows(2) {
        // edit from here


        // end of edit
    }
    roots
}

/// Evaluate a quadratic Bezier curve at parameter `t` in [0, 1].
fn eval_quad_bezier(ps: [f32; 2], pc: [f32; 2], pe: [f32; 2], t: f32) -> [f32; 2] {
    let one_minus_t = 1.0 - t;
    let w0 = one_minus_t * one_minus_t;
    let w1 = 2.0 * one_minus_t * t;
    let w2 = t * t;
    [
        w0 * ps[0] + w1 * pc[0] + w2 * pe[0],
        w0 * ps[1] + w1 * pc[1] + w2 * pe[1],
    ]
}

/// Number of intersections of the ray (org, org+dir) against line segment (ps, pe).
fn number_of_intersection_ray_against_edge(
    org: [f32; 2],
    dir: [f32; 2],
    ps: [f32; 2],
    pe: [f32; 2],
) -> i32 {
    let org_dir = [org[0] + dir[0], org[1] + dir[1]];
    let a = area(org, org_dir, ps);
    let b = area(org, pe, org_dir);
    let c = area(org, ps, pe);
    let d = area([dir[0] + ps[0], dir[1] + ps[1]], ps, pe);
    if a * b > 0.0 && d * c < 0.0 {
        1
    } else {
        0
    }
}

/// Number of intersections of the ray against a quadratic Bézier (ps, pc, pe).
fn number_of_intersection_ray_against_quadratic_bezier(
    org: [f32; 2],
    dir: [f32; 2],
    ps: [f32; 2],
    pc: [f32; 2],
    pe: [f32; 2],
) -> i32 {
    // NOTE: this currently falls back to the straight-edge test.
    // Comment out below and replace the code with a proper
    // Bézier–ray test to complete the task.
    return number_of_intersection_ray_against_edge(org, dir, ps, pe);

    // use the next code
    // let roots = roots_of_quadratic_function_between_0_1(qa, qb, qc);
}

/// Return the closest point on segment `(ps, pe)` to `org` and its distance.
fn nearest_point_on_edge(org: [f32; 2], ps: [f32; 2], pe: [f32; 2]) -> ([f32; 2], f32) {
    // return nearest point and distance
    let e = [pe[0] - ps[0], pe[1] - ps[1]];
    let ee = dot(&e, &e);

    // Degenerate segment: start and end are effectively the same point.
    if ee <= 1.0e-12 {
        let v = [ps[0] - org[0], ps[1] - org[1]];
        return (ps, dot(&v, &v).sqrt());
    }

    let w = [org[0] - ps[0], org[1] - ps[1]];
    let t = (dot(&w, &e) / ee).clamp(0.0, 1.0);
    let p = [ps[0] + t * e[0], ps[1] + t * e[1]];
    let v = [p[0] - org[0], p[1] - org[1]];
    (p, dot(&v, &v).sqrt())
}

/// Return the closest point on a quadratic Bezier `(ps, pc, pe)` to `org` and its distance
fn nearest_point_on_quadratic_bezier(
    org: [f32; 2],
    ps: [f32; 2],
    pc: [f32; 2],
    pe: [f32; 2],
) -> ([f32; 2], f32) {

    // comment out below
    (ps, ((ps[0]-org[0])*(ps[0]-org[0])+(ps[1]-org[1])*(ps[1]-org[1])).sqrt())

    // Minimise |B(t) - org|^2 on [0, 1]. Stationary points satisfy a cubic.
    // implement below

    // end of edit
}

/// Rasterise the parsed SVG path into a grayscale PNG image.
fn main() {
    // Locate the SVG asset relative to this crate's source directory so the
    // binary works regardless of the working directory.
    let input_path = std::path::Path::new("r.svg");
    let (width, height, shape) = parse_svg::get_image_size_and_shape(input_path);
    if width == 0 {
        eprintln!("file open failure");
        std::process::abort();
    }

    let outline_path = parse_svg::outline_path_from_shape(&shape);
    let loops = parse_svg::loops_from_outline_path(&outline_path);

    // Grayscale image initialised to white (255).
    let mut img_data = vec![255u8; width * height];

    for i_pix in 0..height * width {
        let ih = i_pix / width;
        let iw = i_pix % width;
        let org = [iw as f32 + 0.5, ih as f32 + 0.5]; // pixel centre
        let dir = [60.0f32, 20.0f32]; // ray direction
        let mut count_cross = 0i32;

        loops.iter().flatten().for_each(|edge| {
            if edge.is_bezier {
                count_cross += number_of_intersection_ray_against_quadratic_bezier(
                    org, dir, edge.ps, edge.pc, edge.pe,
                );
            } else {
                count_cross += number_of_intersection_ray_against_edge(org, dir, edge.ps, edge.pe);
            }
        });

        if count_cross % 2 == 1 {
            // Jordan curve theorem: odd crossings → inside the shape
            img_data[ih * width + iw] = 0; // paint black
            continue;
        }
        let (_nearest, distance) = loops
            .iter()
            .flatten()
            .map(|edge| {
                if edge.is_bezier {
                    nearest_point_on_quadratic_bezier(org, edge.ps, edge.pc, edge.pe)
                } else {
                    nearest_point_on_edge(org, edge.ps, edge.pe)
                }
            })
            .min_by(|(_, d0), (_, d1)| d0.total_cmp(d1))
            .unwrap();
        if (0..3).contains(&((distance * 0.5) as i32 % 10)) {
            img_data[ih * width + iw] = 128u8;
        }
    }

    let output_path = std::path::Path::new("output.png");
    image::save_buffer(
        output_path,
        &img_data,
        width as u32,
        height as u32,
        image::ColorType::L8,
    )
    .expect("Failed to write output.png");

    println!("Output written to {}", output_path.display());
}
