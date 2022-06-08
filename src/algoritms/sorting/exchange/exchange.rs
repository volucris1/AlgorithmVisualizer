use crate::{
    canvas::Canvas,
    utils,
};

pub fn sort(vec: &mut Vec<i32>, canvas: &mut Canvas, data_w: u32) {
    for i in 0..vec.len() {
        utils::canvas_draw!(canvas, {
            canvas.draw_vec(vec, data_w);
        });

        for j in i + 1..vec.len() {
            if vec[i] > vec[j] {
                vec.swap(i, j);
            }
        }
    }
}
