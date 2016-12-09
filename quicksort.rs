fn main() {
   

    let mut strings = ["Bob", "Cat", "Dog", "Freind"];
    println!(strings);

    quick_sort(&mut strings, &lessthan);
    println!("After sorted", values);
 
 
}


fn lessthan<T: Ord>(x: &T, y: &T) -> bool {
    x < y
}

fn quick_sort<T>(v: &mut [T], f: &OrderFunc<T>) {
 
    let len = v.len();
    if len < 2 {
        return;
    }
 
    let INDEX = partition(v, f);
 
    
    quick_sort(&mut v[0..INDEX], f);
 
 
    quick_sort(&mut v[INDEX + 1..len], f);
}
 
fn partition<T>(v: &mut [T], f: &OrderFunc<T>) -> usize {
    let len = v.len();
    let INDEX = len / 2;
 
    v.swap(INDEX, len - 1);
 
    let mut store = 0;
    for i in 0..len - 1 {
        if f(&v[i], &v[len - 1]) {
            v.swap(i, store);
            store += 1;
        }
    }
 
    v.swap(store, len - 1);
    store
}
