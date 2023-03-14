
fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for i in 0..list.len() {
        for x in 0..list.len() - 1 {
            
            if list[x] > list[x + 1] {
                list.swap(x, x + 1);
            }
        }
    }
    list
}

fn main() {
 
    let mut list = vec![1, 39, 513, 101, 74, 51, 251, 100, 65];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
 
    let mut list = vec!['D', 'e', 'A', 'C', 'a', 'W', 'r', 'j', 'F'];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
}
