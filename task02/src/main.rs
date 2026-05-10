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

/// Find real roots of a quadratic equation ax^2 + bx + c = 0 within [0, 1].
fn roots_of_quadratic_function_between_0_1(a: f32, b: f32, c: f32) -> Vec<f32> {
    let mut roots = Vec::new();
    // manage lineal case (a aprox. 0)
    if a.abs() < 1e-8 {
        if b.abs() > 1e-8 {
            let t = -c / b;
            if t >= 0.0 && t <= 1.0 { roots.push(t); }
        }
        return roots;
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant >= 0.0 {
        let sqrt_d = discriminant.sqrt();
        let t1 = (-b + sqrt_d) / (2.0 * a);
        let t2 = (-b - sqrt_d) / (2.0 * a);
        
        for t in [t1, t2] {
            if t >= 0.0 && t <= 1.0 {
                roots.push(t);
            }
        }
    }
    // opcional: sort and delete near duplicates
    roots
}

/// Find real roots of a cubic equation at^3 + bt^2 + ct + d = 0 via bisection within [0, 1].
fn roots_of_cubic_function_between_0_1(a: f32, b: f32, c: f32, d: f32) -> Vec<f32> {
    // Si 'a' es 0, es una cuadrática.
    if a.abs() < 1e-8 { 
        return roots_of_quadratic_function_between_0_1(b, c, d);
    }

    // divide into equal segments using square roots of derivative (3at^2 + 2bt + c = 0)
    let mut xs = vec![0.0f32];
    for r in roots_of_quadratic_function_between_0_1(3.0 * a, 2.0 * b, c) {
        xs.push(r);
    }
    xs.push(1.0f32);
    // sort for windows(2)
    xs.sort_by(|x, y| x.total_cmp(y));

    let eval = |x: f32| -> f32 { ((a * x + b) * x + c) * x + d }; // Horner's
    let mut roots: Vec<f32> = Vec::new();

    for w in xs.windows(2) {
        let (mut t0, mut t1) = (w[0], w[1]);
        let f0 = eval(t0);
        let f1 = eval(t1);

        // bisection for any change of sign
        if f0 * f1 <= 0.0 {
            for _ in 0..24 { // iterations for f32 precision
                let mid = (t0 + t1) * 0.5;
                if eval(t0) * eval(mid) <= 0.0 {
                    t1 = mid;
                } else {
                    t0 = mid;
                }
            }
            roots.push((t0 + t1) * 0.5);
        }
    }
    // optional root management
    // roots.sort_by(|x, y| x.total_cmp(y));
    roots
}

/// Evaluate a quadratic Bezier curve at parameter `t` in [0, 1].
fn eval_quad_bezier(ps: [f32; 2], pc: [f32; 2], pe: [f32; 2], t: f32) -> [f32; 2] {
    let mt = 1.0 - t;
    [
        mt * mt * ps[0] + 2.0 * mt * t * pc[0] + t * t * pe[0],
        mt * mt * ps[1] + 2.0 * mt * t * pc[1] + t * t * pe[1],
    ]
}

/// Number of intersections of the ray (org, org+dir) against line segment (ps, pe).
fn number_of_intersection_ray_against_edge(
    org: [f32; 2], dir: [f32; 2], ps: [f32; 2], pe: [f32; 2],
) -> i32 {
    let org_dir = [org[0] + dir[0], org[1] + dir[1]];
    let a = area(org, org_dir, ps);
    let b = area(org, pe, org_dir);
    let c = area(org, ps, pe);
    let d = area([dir[0] + ps[0], dir[1] + ps[1]], ps, pe);
    if a * b > 0.0 && d * c < 0.0 { 1 } else { 0 }
}

/// Number of intersections of the ray against a quadratic Bézier (ps, pc, pe).
fn number_of_intersection_ray_against_quadratic_bezier(
    org: [f32; 2], dir: [f32; 2], ps: [f32; 2], pc: [f32; 2], pe: [f32; 2],
) -> i32 {
    // use implicit ray form n · (P - Org) = 0 and solve quadratic
    // normalize ray (exchange components and neglect until 90°)
    let n = [-dir[1], dir[0]]; 
    
    // quadratic coefficient B(t) = at^2 + bt + c
    // a = Ps - 2Pc + Pe
    // b = 2(Pc - Ps)
    // c = Ps
    let a_vec = [ps[0] - 2.0 * pc[0] + pe[0], ps[1] - 2.0 * pc[1] + pe[1]];
    let b_vec = [2.0 * (pc[0] - ps[0]), 2.0 * (pc[1] - ps[1])];
    let c_vec = [ps[0] - org[0], ps[1] - org[1]]; // (P - Org)

    // substitute on n · P = 0 => (n·a)t^2 + (n·b)t + (n·c) = 0
    let qa = dot(&n, &a_vec);
    let qb = dot(&n, &b_vec);
    let qc = dot(&n, &c_vec);

    let roots = roots_of_quadratic_function_between_0_1(qa, qb, qc);
    let mut count = 0;
    for t in roots {
        // verify if intersection occurs "forward" on ray
        let p = eval_quad_bezier(ps, pc, pe, t);
        let to_p = [p[0] - org[0], p[1] - org[1]];
        if dot(&to_p, &dir) > 0.0 {
            count += 1;
        }
    }
    count
}

/// Return the closest point on segment `(ps, pe)` to `org` and its distance.
fn nearest_point_on_edge(org: [f32; 2], ps: [f32; 2], pe: [f32; 2]) -> ([f32; 2], f32) {
    let e = [pe[0] - ps[0], pe[1] - ps[1]];
    let ee = dot(&e, &e);
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
    org: [f32; 2], ps: [f32; 2], pc: [f32; 2], pe: [f32; 2],
) -> ([f32; 2], f32) {
    // coefficients of B(t) = at^2 + bt + c, where c is Ps - org
    let a = [ps[0] - 2.0 * pc[0] + pe[0], ps[1] - 2.0 * pc[1] + pe[1]];
    let b = [2.0 * (pc[0] - ps[0]), 2.0 * (pc[1] - ps[1])];
    let c = [ps[0] - org[0], ps[1] - org[1]];

    // f(t) = |at^2 + bt + c|^2. derivative f'(t)=0 to find minimum
    // f'(t) = 2(2at + b) · (at^2 + bt + c) = 0
    // (4a·a)t^3 + (6a·b)t^2 + (4a·c + 2b·b)t + (2b·c) = 0
    // all is divided by 2 to simplify coefficients of cubic (A, B, C, D):
    let ca = 2.0 * dot(&a, &a);
    let cb = 3.0 * dot(&a, &b);
    let cc = 2.0 * dot(&a, &c) + dot(&b, &b);
    let cd = dot(&b, &c);

    let mut min_dist_sq = f32::MAX;
    let mut best_p = ps; // initial fallback

    // candidates: cubic roots E extremes (t=0, t=1)
    let mut candidates = roots_of_cubic_function_between_0_1(ca, cb, cc, cd);
    candidates.push(0.0);
    candidates.push(1.0);

    for t in candidates {
        let p = eval_quad_bezier(ps, pc, pe, t);
        let dist_sq = (p[0] - org[0]).powi(2) + (p[1] - org[1]).powi(2);
        if dist_sq < min_dist_sq {
            min_dist_sq = dist_sq;
            best_p = p;
        }
    }
    (best_p, min_dist_sq.sqrt())
}

fn main() {
    let input_path = std::path::Path::new("r.svg");
    let (width, height, shape) = parse_svg::get_image_size_and_shape(input_path);
    if width == 0 {
        eprintln!("file open failure");
        std::process::abort();
    }

    let outline_path = parse_svg::outline_path_from_shape(&shape);
    let loops = parse_svg::loops_from_outline_path(&outline_path);
    let mut img_data = vec![255u8; width * height];

    for i_pix in 0..height * width {
        let ih = i_pix / width;
        let iw = i_pix % width;
        let org = [iw as f32 + 0.5, ih as f32 + 0.5];
        let dir = [60.0f32, 20.0f32];
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