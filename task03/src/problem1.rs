use crate::raster::{draw_3d_triangle_with_texture, mat4_mul, mat4_mul_vec4, to_homogeneous};

pub fn camera_transformation() -> [[f32; 4]; 4] {
    let near: f32 = 0.5;
    let far: f32 = 4.0;
    let frustrum_near_size: f32 = 0.55;

    let mut transform = [[0.0f32; 4]; 4];
    transform[0][0] = near / frustrum_near_size;
    transform[1][1] = near / frustrum_near_size;
    transform[2][2] = (far + near) / (far - near);
    transform[2][3] = 2.0 * far * near / (far - near);
    transform[3][2] = -1.0;

    let transl = [
        [1.0f32, 0.0, 0.0, 0.0],
        [0.0f32, 1.0, 0.0, 0.0],
        [0.0f32, 0.0, 1.0, -2.0],
        [0.0f32, 0.0, 0.0, 1.0],
    ];
    mat4_mul(transform, transl)
}

pub fn problem1() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // ── Load texture ─────────────────────────────────────────────────────────
    let input_tex_path = std::path::Path::new(manifest_dir).join("uv.png");
    let tex_img = image::open(&input_tex_path)
        .expect("Failed to load texture image")
        .to_rgb8();
    let tex_width = tex_img.width() as usize;
    let tex_height = tex_img.height() as usize;
    let img_data_tex: Vec<u8> = tex_img.into_raw(); // packed RGB bytes

    // ── 3D geometry ──────────────────────────────────────────────────────────
    let xyz0: [f32; 3] = [-1.0, -1.0, 1.0];
    let xyz1: [f32; 3] = [1.0, -1.0, 1.0];
    let xyz2: [f32; 3] = [1.0, 1.0, -1.0];
    let xyz3: [f32; 3] = [-1.0, 1.0, -1.0];

    let uv0: [f32; 2] = [0.0, 0.0]; // bottom-left
    let uv1: [f32; 2] = [1.0, 0.0]; // bottom-right
    let uv2: [f32; 2] = [1.0, 1.0]; // top-right
    let uv3: [f32; 2] = [0.0, 1.0]; // top-left

    // ── Camera transform: world → NDC ────────────────────────────────────────
    let transform_xyz2ndc = camera_transformation();
    let ndcw0 = mat4_mul_vec4(transform_xyz2ndc, to_homogeneous(xyz0));
    let ndcw1 = mat4_mul_vec4(transform_xyz2ndc, to_homogeneous(xyz1));
    let ndcw2 = mat4_mul_vec4(transform_xyz2ndc, to_homogeneous(xyz2));
    let ndcw3 = mat4_mul_vec4(transform_xyz2ndc, to_homogeneous(xyz3));

    // ── Output image (RGB, initialised black) ────────────────────────────────
    const IMG_SHAPE: (usize, usize) = (300, 300); // w, h
    let mut img_data = vec![0u8; IMG_SHAPE.0 * IMG_SHAPE.1 * 3];

    // Triangle 0–1–2
    draw_3d_triangle_with_texture(
        ndcw0,
        ndcw1,
        ndcw2,
        uv0,
        uv1,
        uv2,
        IMG_SHAPE,
        &mut img_data,
        (tex_width, tex_height),
        &img_data_tex,
    );
    // Triangle 0–2–3
    draw_3d_triangle_with_texture(
        ndcw0,
        ndcw2,
        ndcw3,
        uv0,
        uv2,
        uv3,
        IMG_SHAPE,
        &mut img_data,
        (tex_width, tex_height),
        &img_data_tex,
    );

    // ── Write output ─────────────────────────────────────────────────────────
    let output_path = std::path::Path::new(manifest_dir).join("output1.png");
    image::save_buffer(
        &output_path,
        &img_data,
        IMG_SHAPE.0 as u32,
        IMG_SHAPE.1 as u32,
        image::ColorType::Rgb8,
    )
    .expect("Failed to write output.png");

    println!("Output written to {}", output_path.display());
}
