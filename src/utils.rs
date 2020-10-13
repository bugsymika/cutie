pub fn subdivide(length: (i32, i32), width: (i32, i32)) -> [(i32, i32); 4] {
    let vertical_midpoint = calculate_midpoint(width);
    let horizontal_midpoint = calculate_midpoint(length);

    let sw = (length.0, vertical)
}

fn calculate_midpoint(line: (i32, i32)) -> i32 {
    let difference = line.1 as f32 - line.0 as f32;
    return line.0 + difference.round() as i32;
}
