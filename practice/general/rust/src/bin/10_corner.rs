/// Simple corner detection in Rust.

use image::GenericImageView;
use image::GenericImage;

fn main() {
    let mut img = image::open("data/corgi.jpg").unwrap();
    println!("dimensions: {:?}", img.dimensions());
    println!("color:      {:?}", img.color());

    let v_edge_kernel: [f32; 9] = [
        1f32, 0f32, -1f32,
        1f32, 0f32, -1f32,
        1f32, 0f32, -1f32,
    ];
    let h_edge_kernel: [f32; 9] = [
        -1f32, -1f32, -1f32,
        0f32, 0f32, 0f32,
        1f32, 1f32, 1f32,
    ];
    // TODO(andrei): Use MATH and pre-convolve the two filters.
    img = img.grayscale().blur(2.5);
    let v_edges = img.filter3x3(&v_edge_kernel);
    let h_edges = img.filter3x3(&h_edge_kernel);

    for row in 0..img.dimensions().0 {
        for col in 0..img.dimensions().1 {
            // img[(row, col)] = usize::pow(v_edges[(row, col)], 2);
            let mut v_px = v_edges.get_pixel(row, col);
            let mut h_px = h_edges.get_pixel(row, col);
            let val = ((usize::pow(v_px[0].into(), 2u32) + usize::pow(h_px[0].into(), 2u32)) as f32).sqrt();

            v_px[0] = (val as u8);
            img.put_pixel(row, col, v_px);
        }
    }

    // TODO(andrei): Apply NMS here.
    // for row in 1..img.dimensions().0 - 1{
    //     for col in 1..img.dimensions().1 - 1 {

    //     }
    // }

    // Save the image as “fractal.png”, the format is deduced from the path
    img.save("out.png").unwrap();
}