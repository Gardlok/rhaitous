// use_all_functions.rhai

// Use Point functions
let p = create_point(3, 4);
p.x = 6; // testing setter
let point_length = p.length(); // testing method

// Use SpatialVector functions
let v = new_spatial_vector(3.0, 4.0);
v.x = 1.0; // testing setter
let vector_magnitude = v.magnitude(); // testing method
let vector_angle = v.angle(); // testing method

// Use File concatenation function (Assuming there's at least one file in the current directory)
let concatenated_contents = cat_files(".");

// Validation
if point_length > 0.0 && vector_magnitude > 0.0 && vector_angle >= -180.0 && vector_angle <= 180.0 && !concatenated_contents.is_empty() {
    true
} else {
    false
}
