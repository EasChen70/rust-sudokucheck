use std::collections::HashMap;

pub trait IterRowMajor {
    type Item;
    fn iter_row_major(&self) -> Option<Self::Item>;
}

pub trait IterColumnMajor {
    type Item;
    fn iter_column_major(&self) -> Option<Self::Item>;
}

pub trait Get<T> {
    fn get(index: (usize,usize)) -> Result<T,String>;
}

pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}


impl<T: Clone> Array2<T> {
    //Create slate Array2
    pub fn new(width: usize, height: usize, default_value: T) -> Self {
        let data = vec![default_value; width * height];
        Array2 { width, height, data }
    }
    //Allows you to push in data 
    pub fn with_data(width: usize, height: usize, data: Vec<T>) -> Self {
        Array2 { width, height, data }
    }
    
    pub fn printwidth(self){
        println!("{0}",self.width);
    }
    
    //Makes the Array2
    pub fn from_row_major(self) -> Vec<Vec<T>> {
        let mut array: Vec<Vec<T>> = Vec::with_capacity(self.height);
        let mut hold: Vec<T> = Vec::with_capacity(self.width);
        let mut x = 1;
        
        for i in self.data {
            if x <= self.width {
                hold.push(i);
                x += 1;
            } else {
                array.push(hold);
                hold = Vec::with_capacity(self.width); // Clear the vector without deallocating memory
                hold.push(i);
                x = 2;
            }
        }
        array.push(hold); // Push the last row
        return array;
    }
    
}

impl<T: Eq + std::hash::Hash> IterRowMajor for Vec<Vec<T>> {
    type Item = ();

    fn iter_row_major(&self) -> Option<Self::Item>{
        for row in self{
            let mut seen_elements = HashMap::new();
            for item in row{
                if seen_elements.contains_key(&item){
                    return None;
                }else{
                    seen_elements.insert(item, ());
                }
            }
        }
        Some(())
    }
}

impl<T: Eq + std::hash::Hash> IterColumnMajor for Vec<Vec<T>> {
    type Item = ();

    fn iter_column_major(&self) -> Option<Self::Item>{
        //number of columns is equal to number of rows
        let num_cols = self.len(); // = 9
        //from iterate columns
        for col in 0..num_cols{
            let mut seen_elements = HashMap::new();
            //pretty much do row major, but at the defined column, so row 0,1,2,3,4 while col hasn't iterated yet
            for row in self{
                let item = &row[col];
                if seen_elements.contains_key(&item){
                    return None;
                }else{
                    seen_elements.insert(item, ());
                }
                
            }
        }
        Some(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_row_major() {
        // Define test data
        let width = 9;
        let height = 9;
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 5, 6, 7, 8, 9, 1, 2, 3, 7, 8, 9, 1, 2, 3, 4, 5, 6, 2, 3, 4, 5, 6, 7, 8, 9, 1, 5, 6, 7, 8, 9, 1, 2, 3, 4, 8, 9, 1, 2, 3, 4, 5, 6, 7, 3, 4, 5, 6, 7, 8, 9, 1, 2, 6, 7, 8, 9, 1, 2, 3, 4, 5, 9, 1, 2, 3, 4, 5, 6, 7, 8];

        // Create an Array2 instance
        let array = Array2::with_data(width, height, data.clone());

        // Convert to row-major representation
        let result = array.from_row_major();

        // Expected result
        let expected = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,2,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];

        // Assert that the result matches the expected value
        assert_eq!(result, expected);
    }


    #[test]
    fn test_iter_row_major_no_duplicates() {
        // Create a vector of vectors with no duplicates
        let data: Vec<Vec<i32>> = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,2,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];

        // Call iter_row_major and expect Some(())
        assert_eq!(data.iter_row_major(), Some(()));
    }

    #[test]
    fn test_iter_row_major_with_duplicates() {
        // Create a vector of vectors with duplicates
        let data: Vec<Vec<i32>> = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,2,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,3,5,5,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];

        // Call iter_row_major and expect None
        assert_eq!(data.iter_row_major(), None);
    }

    #[test]
    fn test_iter_col_major_no_duplicates(){
        let data: Vec<Vec<i32>> = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![4,5,6,7,8,9,1,2,3],
            vec![7,8,9,1,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];
        assert_eq!(data.iter_column_major(), Some(()));
    }

    #[test]
    fn test_iter_col_major_with_duplicates(){
        let data: Vec<Vec<i32>> = vec![
            vec![1,2,3,4,5,6,7,8,9],
            vec![1,5,6,7,8,9,4,2,3],
            vec![1,8,9,7,2,3,4,5,6],
            vec![2,3,4,5,6,7,8,9,1],
            vec![5,6,7,8,9,1,2,3,4],
            vec![8,9,1,2,3,4,5,6,7],
            vec![3,4,5,6,7,8,9,1,2],
            vec![6,7,8,9,1,2,3,4,5],
            vec![9,1,2,3,4,5,6,7,8],
        ];
        assert_eq!(data.iter_column_major(), None);
    }
}





// impl<T: Clone> IterColumnMajor for Array2<T>{
//     type Item = (usize,usize,T);
//     fn iter_column_major(&self) -> Option<Self::Item>{
//         //iterates through a 2d vector by column returning a tuple(row,column,element)
//     }
// }

// impl<T: Clone> IterRowMajor for Array2<T>{
//     type Item = (usize,usize,T);
//     fn iter_row_major(&self) -> Option<Self::Item>{
//         //iterates through a 2d vector by column returning a tuple(row,column,element)

//     }
// }

// impl<T: Clone> get<T> for Array2<T>{
//     fn get(index:(usize,usize)) -> Result<T,String>{
//         //using the tuple, return a reference to an element in the 2d array at those cords

//     }
// }