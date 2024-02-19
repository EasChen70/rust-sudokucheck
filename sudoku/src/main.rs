use csc411_image::{Read, GrayImage};
use array2::Array2;
use array2::IterRowMajor;
use array2::IterColumnMajor;
use std::env;


fn main() {
    let input = match env::args().nth(1){
        Some(input) => input,
        None => {
            eprintln!("Invalid File/No File Found");
            std::process::exit(1);
        }
    };
    
    let img = match GrayImage::read(Some(&input)){
        Ok(img) => img,
        Err(err) => {
            eprintln!("Error with Image {}", err);
            std::process::exit(1);
        }
    };

    //Check image dimensions
    let width = img.width;
    let height = img.height;
    if width != 9 || height != 9{
        eprintln!("Not valid dimensions for sudoku puzzle");
        std::process::exit(1);
    }

    //create 1d vector of pixels from grayimage
    let mut hold = vec![];
    
    for x in img.pixels.into_iter(){
        let normalized_value = (x.value as f32/ 28.33).round() as u8;
        hold.push(normalized_value);
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