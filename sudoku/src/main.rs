use csc411_image::{Read, GrayImage};
use array2::Array2;
use array2::IterRowMajor;
use array2::IterColumnMajor;
use std::env;


fn main() {
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    //create 1d vector of pixels from grayimage
    let mut hold = vec![];
    for x in img.pixels.into_iter(){
        hold.push(x.value);
    }
    //gives data from hold to array2
    let array = Array2::with_data(img.width.try_into().unwrap(),img.height.try_into().unwrap(),hold);
    
    //makes 2d vector
    let sudoku = array.from_row_major();
    for row in &sudoku {
        println!("{:?}", row);
    }
    let result = sudoku.iter_row_major();
    println!("{:?}", result);
    let result2 = sudoku.iter_column_major();
    println!("{:?}", result2);

    
}