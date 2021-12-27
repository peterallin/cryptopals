use plotters::prelude::*;

pub fn simple_xy(title: &str, filename: &str, xy_data: &[(usize, usize)]) {
    let max_x = *xy_data
        .iter()
        .map(|(key_size, _score)| key_size)
        .max()
        .unwrap()
        + 10;
    let max_y = *xy_data
        .iter()
        .map(|(_key_size, score)| score)
        .max()
        .unwrap()
        + 10;

    let ks_scores = xy_data.iter().map(|(ks, sc)| (*ks as f32, *sc as f32));

    let root = BitMapBackend::new(filename, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(50)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..max_x as f32, 0f32..max_y as f32)
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    let series = LineSeries::new(ks_scores, &RED);
    chart.draw_series(series).unwrap();
}
