use crate::obj_parser::parse_obj;
use crate::raster::{draw_3d_triangle_with_texture, mat4_mul, mat4_mul_vec4, to_homogeneous};

pub fn camera_transformation() -> [[f32; 4]; 4] {
    let near: f32 = 0.5;
    let far: f32 = 4.0;
    let frustrum_near_size: f32 = 0.30;

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
    let theta = 135.0f32.to_radians();
    let rot_y = [
        [theta.cos(), 0.0, theta.sin(), 0.0],
        [0.0f32, 1.0, 0.0, 0.0],
        [-theta.sin(), 0.0, theta.cos(), 0.0],
        [0.0f32, 0.0, 0.0, 1.0],
    ];
    let world_to_camera = mat4_mul(transl, rot_y);
    mat4_mul(transform, world_to_camera)
}

fn compute_tri2ndcz(vtx2ndcw: &[[f32; 4]], tri2vtx: &[usize]) -> Vec<f32> {
    assert_eq!(tri2vtx.len() % 3, 0);

    let num_tri = tri2vtx.len() / 3;
    let mut tri2ndcz: Vec<f32> = Vec::with_capacity(num_tri);
    for i_tri in 0..num_tri {
        let i0_vtx = tri2vtx[i_tri * 3];
        let i1_vtx = tri2vtx[i_tri * 3 + 1];
        let i2_vtx = tri2vtx[i_tri * 3 + 2];

        let z0 = vtx2ndcw[i0_vtx][2] / vtx2ndcw[i0_vtx][3];
        let z1 = vtx2ndcw[i1_vtx][2] / vtx2ndcw[i1_vtx][3];
        let z2 = vtx2ndcw[i2_vtx][2] / vtx2ndcw[i2_vtx][3];
        tri2ndcz.push((z0 + z1 + z2) / 3.0);
    }

    tri2ndcz
}

pub fn problem2() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let obj_path = std::path::Path::new(manifest_dir).join("spot_triangulated.obj");

    let (vtx2xyz, vt2uv, tri2vtx, tri2vt) = parse_obj(&obj_path);
    assert_eq!(tri2vtx.len(), tri2vt.len());
    assert_eq!(tri2vtx.len() % 3, 0);
    let num_triangles = tri2vtx.len() / 3;

    let transform_xyz2ndc = camera_transformation();
    let vtx2ndcw: Vec<[f32; 4]> = vtx2xyz
        .iter()
        .map(|&xyz| mat4_mul_vec4(transform_xyz2ndc, to_homogeneous(xyz)))
        .collect();

    let input_tex_path = std::path::Path::new(manifest_dir).join("spot_texture.png");
    let tex_img = image::open(&input_tex_path)
        .expect("Failed to load texture image")
        .to_rgb8();
    let tex_width = tex_img.width() as usize;
    let tex_height = tex_img.height() as usize;
    let img_data_tex: Vec<u8> = tex_img.into_raw(); // packed RGB bytes

    // ── Output image (RGB, initialised black) ────────────────────────────────
    const IMG_SHAPE: (usize, usize) = (300, 300); // w, h
    let mut img_data = vec![0u8; IMG_SHAPE.0 * IMG_SHAPE.1 * 3];

    let mut num_test = 0;

    let mut idx2tri: Vec<usize> = (0..num_triangles).collect(); // lexicographical order
    let tri2ndcz = compute_tri2ndcz(&vtx2ndcw, &tri2vtx);
    // edit from here

    // end of the edit
    // --------------------------------------------

    for &i_tri in &idx2tri {
        let i0_vtx = tri2vtx[i_tri * 3];
        let i1_vtx = tri2vtx[i_tri * 3 + 1];
        let i2_vtx = tri2vtx[i_tri * 3 + 2];

        let i0_vt = tri2vt[i_tri * 3];
        let i1_vt = tri2vt[i_tri * 3 + 1];
        let i2_vt = tri2vt[i_tri * 3 + 2];

        num_test += draw_3d_triangle_with_texture(
            vtx2ndcw[i0_vtx],
            vtx2ndcw[i1_vtx],
            vtx2ndcw[i2_vtx],
            vt2uv[i0_vt],
            vt2uv[i1_vt],
            vt2uv[i2_vt],
            IMG_SHAPE,
            &mut img_data,
            (tex_width, tex_height),
            &img_data_tex,
        );
    }

    println!("number of in/out test: {}", num_test);

    let output_path = std::path::Path::new(manifest_dir).join("output2.png");
    image::save_buffer(
        &output_path,
        &img_data,
        IMG_SHAPE.0 as u32,
        IMG_SHAPE.1 as u32,
        image::ColorType::Rgb8,
    )
    .unwrap_or_else(|e| panic!("Failed to write {}: {e}", output_path.display()));
    assert_eq!(tri2ndcz.len(), num_triangles);

    println!(
        "Rasterized {} triangles from {} -> {} (tri2ndc_z: {} values)",
        num_triangles,
        obj_path.display(),
        output_path.display(),
        tri2ndcz.len()
    );
}
