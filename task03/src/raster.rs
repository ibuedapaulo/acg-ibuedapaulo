pub fn mat4_mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            out[i][j] =
                a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    out
}

pub fn mat4_mul_vec4(m: [[f32; 4]; 4], v: [f32; 4]) -> [f32; 4] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2] + m[0][3] * v[3],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2] + m[1][3] * v[3],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2] + m[2][3] * v[3],
        m[3][0] * v[0] + m[3][1] * v[1] + m[3][2] * v[2] + m[3][3] * v[3],
    ]
}

pub fn to_homogeneous(v: [f32; 3]) -> [f32; 4] {
    [v[0], v[1], v[2], 1.0]
}

/// Build a 4x4 perspective projection matrix combined with a camera translation,
/// mapping world coordinates to normalised device coordinates (NDC) in [-1,+1]^3.
#[inline]
fn area_of_triangle_2d(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2]) -> f32 {
    let e10 = p1[0] - p0[0];
    let e11 = p1[1] - p0[1];
    let e20 = p2[0] - p0[0];
    let e21 = p2[1] - p0[1];
    e10 * e21 - e11 * e20
}

/// Rasterise one 3D textured triangle into `img_data_out`.
///
/// `q0..q2` are the triangle vertices in homogeneous clip-space coordinates.
/// `uv0..uv2` are the corresponding UV texture coordinates.
///  Output is the number of pixel centers for in/out test
#[allow(clippy::too_many_arguments)]
pub fn draw_3d_triangle_with_texture(
    ndcw0: [f32; 4],
    ndcw1: [f32; 4],
    ndcw2: [f32; 4],
    uv0: [f32; 2],
    uv1: [f32; 2],
    uv2: [f32; 2],
    (width_out, height_out): (usize, usize),
    img_data_out: &mut [u8],
    (width_tex, height_tex): (usize, usize),
    img_data_tex: &[u8],
) -> usize {
    // Project vertices to NDC and keep only xy.
    let ndc0 = [ndcw0[0] / ndcw0[3], ndcw0[1] / ndcw0[3]];
    let ndc1 = [ndcw1[0] / ndcw1[3], ndcw1[1] / ndcw1[3]];
    let ndc2 = [ndcw2[0] / ndcw2[3], ndcw2[1] / ndcw2[3]];

    // Compute bounding box in pixel coordinate
    let aabb = {
        // comment out below
        // write some code below
        let x_min = ndc0[0].min(ndc1[0]).min(ndc2[0]);
        let x_max = ndc0[0].max(ndc1[0]).max(ndc2[0]);
        let y_min = ndc0[1].min(ndc1[1]).min(ndc2[1]);
        let y_max = ndc0[1].max(ndc1[1]).max(ndc2[1]);

        // map ndc [-1, 1] to pixel coords
        let min_iw = (((x_min + 1.0) / 2.0) * width_out as f32).floor().max(0.0) as usize;
        let max_iw = (((x_max + 1.0) / 2.0) * width_out as f32).ceil().min(width_out as f32) as usize;

        // y-axis is inverted between ndc space and screen space
        let min_ih = (((1.0 - y_max) / 2.0) * height_out as f32).floor().max(0.0) as usize;
        let max_ih = (((1.0 - y_min) / 2.0) * height_out as f32).ceil().min(height_out as f32) as usize;

        // end of edit
        [min_iw, min_ih, width_out, height_out] // moved down here and edited zeroes
    };

    for ih in aabb[1]..aabb[3] {
        for iw in aabb[0]..aabb[2] {
            // Pixel centre in NDC [-1,+1]^2
            let s = [
                ((iw as f32 + 0.5) * 2.0) / width_out as f32 - 1.0,
                1.0 - ((ih as f32 + 0.5) * 2.0) / height_out as f32,
            ];

            // Signed triangle areas (2D cross products)
            let area0 = area_of_triangle_2d(s, ndc1, ndc2);
            let area1 = area_of_triangle_2d(s, ndc2, ndc0);
            let area2 = area_of_triangle_2d(s, ndc0, ndc1);

            // Pixel is outside the triangle if any area is negative
            if area0 < 0.0 || area1 < 0.0 || area2 < 0.0 {
                continue;
            }

            // Screen-space barycentric coordinates
            let total = area0 + area1 + area2;
            let bc_scr: [f32; 3] = [area0 / total, area1 / total, area2 / total];
            let w = 1.0 / (bc_scr[0] / ndcw0[3] + bc_scr[1] / ndcw1[3] + bc_scr[2] / ndcw2[3]);
            
            // comment out below
            // edit from here to compute `bc_obj`
            // perspective-correct depth interpolation
            let w = 1.0 / (bc_scr[0] / ndcw0[3] + bc_scr[1] / ndcw1[3] + bc_scr[2] / ndcw2[3]);

            // compute perspective-correct object barycentric coordinates
            let bc_obj = [
                (bc_scr[0] / ndcw0[3]) * w,
                (bc_scr[1] / ndcw1[3]) * w,
                (bc_scr[2] / ndcw2[3]) * w,
            ];
            // end of edit
            // ---------------------------------
            // UV from (screen-space) barycentric interpolation
            let uv = [
                uv0[0] * bc_obj[0] + uv1[0] * bc_obj[1] + uv2[0] * bc_obj[2],
                uv0[1] * bc_obj[0] + uv1[1] * bc_obj[1] + uv2[1] * bc_obj[2],
            ];
            // Texture pixel lookup (wrapping via fractional part)
            let iw_tex = ((uv[0] - uv[0].floor()) * width_tex as f32) as usize;
            let ih_tex = ((1.0 - uv[1] - (1.0 - uv[1]).floor()) * height_tex as f32) as usize;

            let out_idx = (ih * width_out + iw) * 3;
            let tex_idx = (ih_tex * width_tex + iw_tex) * 3;
            img_data_out[out_idx] = img_data_tex[tex_idx];
            img_data_out[out_idx + 1] = img_data_tex[tex_idx + 1];
            img_data_out[out_idx + 2] = img_data_tex[tex_idx + 2];
        }
    }
    return (aabb[2]-aabb[0])*(aabb[3]-aabb[1])
}