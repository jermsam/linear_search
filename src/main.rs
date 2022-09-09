fn main() {
    println!("Hello, world!");
    let unsorted_list:[i8;10] = [3,2,1,5,0,9,7,8,6,4];
    let target:i8 = 11;
     linear_search(&unsorted_list, &target);
    let target:i8 = 0;
   linear_search(&unsorted_list, &target);

}

// ideal for short unsorted lists
fn linear_search (list:&[i8; 10], target: &i8) {
    /* Returns index position if found else returns None */
 let mut index:  Option<i8> =  None;

    for i in 0..list.len() {
        if list[i] == *target {
            index = Some(i as i8)
        }
    }

    match index {
        None => {
            println!("There is no match for {} in {:?}.", target, *list);
        },
        Some(i) => {
            println!("Position of {} in {:?} is {:?}", target, *list,i);
        },
    }
}