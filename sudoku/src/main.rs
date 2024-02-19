use csc411_image::{Read, GrayImage};
use array2::Array2;
use array2::IterRowMajor;
use array2::IterColumnMajor;
use std::env;
use array2::GetC;

//checks 3 by 3 grids 
fn check_grid(board: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let mut grid_values = Vec::new();
    for i in row..row + 3 {
        for j in col..col + 3 {
            //if the vector representing 3 by 3 doesn't have the same element, push it into the vector
            if !grid_values.contains(&Some(board.get_c((i,j)))){
                grid_values.push(Some(board.get_c((i,j))));
            }
            //else return false if there already is the element in the vector 
            else {
                return false
            }
        }
    }
    return true
}


fn main() {
    let input = match env::args().nth(1){
        Some(input) => input,
        None => {
            std::process::exit(1);
        }
    };

    let img = match GrayImage::read(Some(&input)){
        Ok(img) => img,
        Err(_err) => {
            std::process::exit(1);
        }
    };

    //Check image dimensions
    let width = img.width;
    let height = img.height;
    if width != 9 || height != 9{
        std::process::exit(1);
    }

    //create 1d vector of pixels from grayimage
    let mut hold = vec![];

    for x in img.pixels.iter(){
        let normalized_value = x.value as u8;
        if normalized_value > 9 {
            std::process::exit(1)
        }else {
        hold.push(normalized_value);
        }
    }
    //gives data from hold to array2
    let array = Array2::with_data(img.width.try_into().unwrap(),img.height.try_into().unwrap(),hold);

    //makes 2d vector
    let sudoku = array.from_row_major();

    let result = sudoku.iter_row_major();
    if result == None{
        std::process::exit(1);
    }
    let result2 = sudoku.iter_column_major();
    if result2 == None{
        std::process::exit(1);
    }
    for row in 0..3{
        for col in 0..3{
            if !check_grid(&sudoku, row*3, col*3){
                std::process::exit(1);
            }
        }
    }
    std::process::exit(0);

}