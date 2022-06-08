use crate::{
    canvas::Canvas,
    utils,
};

pub fn sort(vec: &mut Vec<i32>, canvas: &mut Canvas, data_w: u32) {
    let mut sorted = false;
    let mut iters = 0;
    let mut vec_len = vec.len() - 1;

    while !sorted && canvas.running() {
        iters += 1;
        sorted = true;
        for i in 0..vec_len {
            utils::canvas_draw!(canvas, {
                canvas.draw_vec(vec, data_w, vec![i]);
            });
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                sorted = false;
            }
        }

        vec_len -= 1;
    }

    println!("Iterations count: {iters}");
}
