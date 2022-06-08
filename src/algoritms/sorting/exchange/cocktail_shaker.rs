use crate::{
    canvas::Canvas,
    utils,
};

pub fn sort(vec: &mut Vec<i32>, canvas: &mut Canvas, data_w: u32) {
    let mut l = 1;
    let mut r = vec.len() - 1;

    let mut was_swapped = true;
    'main: loop {
        utils::canvas_draw!(canvas, {
            canvas.draw_vec(vec, data_w);
        });

        was_swapped = false;
        for i in l..r {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                was_swapped = true;
            }
        }
        r -= 1;
        if !was_swapped {
            break 'main;
        }

        was_swapped = false;
        for i in (l..r).rev() {
            if vec[i] > vec[i + 1] {
                println!("{i}");
                vec.swap(i, i + 1);
                was_swapped = true;
            }
        }

        if !was_swapped {
            break 'main;
        }

        l += 1;
    }
}