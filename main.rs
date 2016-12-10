
fn main() {
   

    let mut strings = ["Bob", "Cat", "Dog", "Freind"];
    println!("Before: {:?}", strings);

    quick_sort(&mut strings, &|x,y| x < y);
    println!("After:  {:?}\n", strings);
 
 
}


fn quick_sort<T,G>(v: &mut [T], g: &G) 
    where G: Fn(&T,&T) -> bool 
{	
    let len = v.len();
    if len >= 2 {
    
 
    let index = partition(v, g);
 
    
    quick_sort(&mut v[0..index], g);
 
 
    quick_sort(&mut v[index + 1..len], g); 
    }
}
 
fn partition<T,G>(v: &mut [T], g: &G) -> usize 
    where G: Fn(&T,&T) -> bool
{
    let len = v.len();
    let index = len / 2;
 
    v.swap(index, len - 1);
 
    let mut store = 0;
    for i in 0..len - 1 {
        if g(&v[i], &v[len - 1]) {
            v.swap(i, store);
            store += 1;
        }
    }
 
    v.swap(store, len - 1);
    store
}
