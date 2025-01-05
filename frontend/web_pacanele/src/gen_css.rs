pub fn make_animation_string(keyframes_name: &str, pic_count: u32) -> String {
    let mut css = "".to_string();
    css.push_str("@keyframes ");
    //  spin { ");
    css.push_str(keyframes_name);
    css.push_str(" { ");
    const NUM_KEYFRAMES: u32 = 33;
    for index in 0..=NUM_KEYFRAMES {
        let rad = 2. * std::f64::consts::PI * index as f64 / NUM_KEYFRAMES as f64;
        let percent = index as f64 / NUM_KEYFRAMES as f64 * 100.0;
        let line_rule = make_transform_string(rad, pic_count);
        let line_css = format!("{percent}% {{ {line_rule} }} ");
        css.push_str(&line_css);
    }
    css.push_str(" }");
    css
}

pub fn make_transform_string(rad: f64, pic_count: u32) -> String {
    let size_coef = 2.0;
    let y = rad.sin() * size_coef * 100.0;
    let z = (rad.cos() - 2.0) * size_coef * 100.0;
    let z_index = ((rad.cos() - 2.0) * 100.0).round() as i32;
    let scale = 2.0 * std::f64::consts::PI / pic_count as f64 * 1.05 * size_coef;
    // info!("transform c={pic_count}   scale={scale}");
    format!("
        transform: perspective(555cqmin)
            translate3d(120cqmin, {y}cqmin, {z}cqmin) 
            rotate3d(1, 0, 0, {rad}rad) scale3d({scale},{scale},{scale});
        z-index: {z_index};
    ")
}
