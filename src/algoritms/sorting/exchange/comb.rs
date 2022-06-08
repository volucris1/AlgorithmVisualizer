use crate::{
    canvas::Canvas,
    utils,
};

pub fn sort(vec: &mut Vec<i32>, canvas: &mut Canvas, data_w: u32) {
    let gap_shrink_factor = 1.3;
    let mut gap = vec.len() as f32;

    let mut sorted = false;
    while !sorted && canvas.running() {
        gap = gap / gap_shrink_factor;
        if gap < 1.0 {
            gap = 1.0;
            sorted = true;
        }

        for l in 0..vec.len() {
            let r = l + gap as usize;

            utils::canvas_draw!(canvas, {
                canvas.draw_vec(vec, data_w, vec![l, r]);
            });

            if r > vec.len() - 1 {
                break;
            }

            if vec[l] > vec[r] {
                vec.swap(l, r);
                sorted = false;
            }
        }
    }
}
