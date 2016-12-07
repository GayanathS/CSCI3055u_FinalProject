
 
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
