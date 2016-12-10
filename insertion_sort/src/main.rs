fn insertionSort(mut array: Vec<i32>) -> Vec<i32>{
  for i in 1..array.len(){
    let key = array[i];
    let mut j=i-1;
    while j >= 0 && array[j] > key{
      array[j+1] = array[j];
      j = j-1;
    }
    array[j+1] = key;
  }
  array
}

fn main() {
    let mut array = vec![1,5,3,22,43,6,2,7];
    println!("Original array:");
    for i in array.iter(){
      print!("{} ",i);
    }
    println!("");

    array=insertionSort(array);

    println!("Sorted array:");

    for i in array.iter(){
      print!("{} ",i);
    }
    println!("");
}
