use csc411_image::{Read, GrayImage};
use array2::Array2;
use std::env;


fn main() {
    let input = env::args().nth(1);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let mut hold = vec![];
    for x in img.pixels.into_iter(){
        hold.push(x.value);
    }
    let array = Array2::with_data(img.width.try_into().unwrap(),img.height.try_into().unwrap(),hold);
   array.printwidth();
   let check = array.from_row_major();
   for row in &check {
    println!("{:?}", row);
}

}