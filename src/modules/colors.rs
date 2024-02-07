use palette::{FromColor, Lab, Srgb};
pub fn convert_colors(mut lab_colors: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    lab_colors
        .iter_mut()
        .map(|lab_color| {
            let lab = Lab::new(lab_color[0], lab_color[1], lab_color[2]);
            let srgb = Srgb::from_color(lab);
            let (r, g, b) = srgb.into_format::<f32>().into_components();
            vec![r, g, b]
        })
        .collect::<Vec<Vec<f32>>>()
}

pub fn extract_colors(data: &str) -> Vec<Vec<f32>> {
    let mut in_data_block = false;
    let mut lab_colors = Vec::new();

    for line in data.lines() {
        if line.trim() == "BEGIN_DATA" {
            in_data_block = true;
        } else if line.trim() == "END_DATA" {
            in_data_block = false;
        } else if in_data_block {
            let values: Vec<f32> = line
                .split('\t') 
                .skip(5)
                .filter_map(|s| s.parse().ok())
                .collect();
            lab_colors.push(values);
        }
    }
    lab_colors
}
