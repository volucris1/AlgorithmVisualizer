use crate::{
    canvas::Canvas,
    utils,
};

pub fn sort(vec: &mut Vec<i32>, canvas: &mut Canvas, data_w: u32) {
    let mut l = 0;
    let mut r = vec.len() - 1;

    while canvas.running() {
        let mut was_swapped = false;
        for i in l..r {
            utils::canvas_draw!(canvas, {
                canvas.draw_vec(vec, data_w, vec![i]);
            });
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                was_swapped = true;
            }
        }
        r -= 1;
        if !was_swapped {
            break;
        }

        was_swapped = false;
        for i in (l..r).rev() {
            utils::canvas_draw!(canvas, {
                canvas.draw_vec(vec, data_w, vec![i]);
            });
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                was_swapped = true;
            }
        }

        if !was_swapped {
            break;
        }

        l += 1;
    }
}
