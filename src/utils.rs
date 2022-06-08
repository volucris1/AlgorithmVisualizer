use rand::Rng;

pub fn generate_vec(size: u32, min_v: i32, max_v: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    let mut rng = rand::thread_rng();

    for _i in 0..size {
        let rn = rng.gen_range(min_v.into()..max_v.into());
        vec.push(rn);
    }

    vec
}

macro_rules! time_execution {
    ($name:literal, $f:block) => {
        println!("Starting \"{}\"", $name);
        let time_start = time::SystemTime::now();
        println!("Strat at: {:?}", time_start);

        $f

        let time_total = time_start.elapsed();
        println!("Time total: {:?}", time_total);
        println!("End of \"{}\"", $name);
    };
}
pub(crate) use time_execution;

/// Handle events -> clear screen -> draw -> present
macro_rules! canvas_draw {
    ($canvas: ident, $draw_f: block) => {
        ($canvas).handle_events();
        ($canvas).clear();

        $draw_f

        $canvas.present();
    };
}

pub(crate) use canvas_draw;
