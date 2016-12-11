
fn merge(list1: Vec<i32>, list2: Vec<i32>) -> Vec<i32>{
  let mut i=0;
  let mut j=0;
  let mut list3=vec![0;list1.len()+list2.len()];
  let mut index=0;
  while i<list1.len() && j<list2.len(){
    if list1[i]<=list2[j]{
      list3[index]=list1[i];
      i = i+1;
    }else{
      list3[index]=list2[j];
      j=j+1;
    }

    index +=1;
  }
  if i < list1.len(){
    for k in i..list1.len(){
      list3[index]=list1[k];
      index +=1;
    }
  }
  if j < list2.len(){
    for k in j..list2.len(){
      list3[index]=list2[k];
      index +=1;
    }
  }
  list3
}

fn mergeSort(mut list: Vec<i32>) -> Vec<i32>{
  let mut sortedness = 1;

  while sortedness < list.len(){

    let mut i=0;
    while i <list.len(){

      let mut list1=vec![0; sortedness];
      for j in i..i+sortedness{
        list1[j-i]=list[j];
      }

      let mut list2=vec![0;sortedness];
      for j in i+sortedness..i+2*sortedness{
        list2[j-i-sortedness]=list[j];
      }

      let list3 = merge(list1, list2);

      for j in i..i+list3.len(){
        list[j]=list3[j-i];
      }

      i=i+(2*sortedness);
    }
    sortedness = 2 * sortedness;
  }
  list
}

fn main() {
    let mut array= vec![1,5,3,22,43,6,2,7];
    println!("Original array:");
    for i in array.iter(){
      print!("{} ",i);
    }
    println!("");

    println!("Sorted array:");
    array=mergeSort(array);
    for i in array.iter(){
      print!("{} ",i);
    }
    println!("");

}
