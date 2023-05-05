// Jared Deaton
// CECS 342-07
// Prog 5 – Rusty Heap Sort
// May/4/2023
//
// I certify that this program is my own original work. I did not copy any part of this program from
 // any other source. I further certify that I typed each and every line of code in this program.

use std::io;
use rand::Rng;

const TOTAL_ITEMS: usize = 32;

fn main() {
    let mut random_items: [i32; TOTAL_ITEMS] = get_random_array();
    print_binary_tree(random_items);
    wait_for_user_input();
    heap_sort(&mut random_items);
    print_binary_tree(random_items);
    wait_for_user_input();
}

fn first_heap_sort(random_items:&mut [i32; TOTAL_ITEMS]){
  let mut index: usize = TOTAL_ITEMS / 2 - 1;
  while index > 0 {
    sort_nodes(random_items, index, TOTAL_ITEMS);
    index -= 1;
  }
}

fn get_random_array() -> [i32; TOTAL_ITEMS] {
    let mut rng = rand::thread_rng();
    let mut random_items: [i32; TOTAL_ITEMS] = [0; TOTAL_ITEMS];
    random_items[0] = -1;
    let mut index: usize = 1;
    while index < TOTAL_ITEMS {
        let _random_value: i32 = rng.gen_range(10..100);
        if !random_items.contains(&_random_value) {
            random_items[index] = _random_value;
            index += 1;
        }
    }
    return random_items;
}

fn heap_sort(random_items:&mut [i32; TOTAL_ITEMS]){
  first_heap_sort(random_items);

  let mut last_index: usize = TOTAL_ITEMS - 1;
     while last_index > 1 {
        swap_values(random_items, 1, last_index);
        sort_nodes(random_items, 1, last_index);
        print_array(random_items);
       last_index -= 1;
      }
  
    if random_items[1] > random_items[2] {
      swap_values(random_items, 1, 2);
      print_array(random_items);
    }
    println!("The tree has been sorted!");
}

fn sort_nodes(random_items:&mut [i32; TOTAL_ITEMS], index:usize, last_index: usize){
  let child1_index: usize = index * 2;
  let child2_index: usize = index * 2 + 1;
  if child1_index < last_index && random_items[child1_index] > random_items[child2_index] {
    if random_items[child1_index] > random_items[index] {
      swap_values(random_items, child1_index, index);
      sort_nodes(random_items, child1_index, last_index);
    }
  }
  else {
    if child2_index < last_index && random_items[child2_index] > random_items[index] {
      swap_values(random_items, child2_index, index);
      sort_nodes(random_items, child2_index, last_index);
    }
  }
}

fn swap_values(random_items:&mut [i32; TOTAL_ITEMS], index1:usize, index2:usize){
    let previous_value1: i32 = random_items[index1];
    random_items[index1] = random_items[index2];
    random_items[index2] = previous_value1;
}

fn print_array(random_items:&mut [i32; TOTAL_ITEMS]){
    let mut index: usize = 1;
    while index < TOTAL_ITEMS - 1 {
        print!("{}, ", random_items[index]);
        index += 1;
    }
    println!("{}", random_items[index]);
}

fn print_binary_tree(random_items: [i32; TOTAL_ITEMS]){
    println!("                                              {}", random_items[1]);
    println!("                                              |");
    println!("                      {}----------------------^----------------------{}", random_items[2], random_items[3]);
    println!("                      |                                               |");
    println!("          {}----------^----------{}                       {}----------^----------{}", random_items[4], random_items[5], random_items[6], random_items[7]);
    println!("          |                       |                       |                       |");
    println!("    {}----^----{}           {}----^----{}           {}----^----{}           {}----^----{}", random_items[8], random_items[9], random_items[10], random_items[11]
                                                                                                        , random_items[12], random_items[13], random_items[14], random_items[15]);
    println!("    |           |           |           |           |           |           |           |");
    println!("{}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}   {}--^--{}", random_items[16], random_items[17], random_items[18], random_items[19], random_items[20],
    random_items[21], random_items[22], random_items[23], random_items[24], random_items[25], random_items[26], random_items[27], random_items[28], random_items[29], random_items[30], random_items[31]);
}

fn wait_for_user_input(){
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
}