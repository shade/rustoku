use std::boxed::Box;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::{self, BufReader};
use std::io::prelude::*;

use std::mem;
use std;

use std::cmp;

use std::thread;
use std::time::Duration;

pub const THREE_VALUE: i8 = 63;
pub const FOUR_VALUE: i8 = 127;
pub const FOUR_STATE: i32 = 10000;

pub static mut counter:u32 = 0;
static mut tf:[u8; 430467210] = [0; 430467210];
pub fn all () {
  unsafe {
    // This is for benching
    if counter > 0 {
      return;
    }

    println!("COMMENT: BUILDING WON TABLE");
    for state in 0..65536 {
      let mut shift = 0;
      while shift < 11 {
        if (state >> shift) & 0b11111 == 0b11111 {
          super::board::WON[state as usize] |= super::board::FIVE_FLAG;
          break;
        }

        // Don't break out of these, because it's possible a five could exist in the same line.
        else if (state >> shift) & 0b11111 == 0b11110 && shift != 10 {
          super::board::WON[state as usize] |= super::board::FIVE_FLAG; shift += 5; continue;
        }
        else if (state >> shift) & 0b11111 == 0b01111 && shift != 0 {
          super::board::WON[state as usize] |= super::board::FIVE_FLAG; shift += 5; continue;
        }/*
        else if (state >> shift) & 0b11111 == 0b11101 {
          super::board::WON[state as usize] |= super::board::FOUR_FLAG; shift += 5; continue;
        }
        else if (state >> shift) & 0b11111 == 0b11011 {
          super::board::WON[state as usize] |= super::board::FOUR_FLAG; shift += 5; continue;
        }
        else if (state >> shift) & 0b11111 == 0b10111 {
          super::board::WON[state as usize] |= super::board::FOUR_FLAG; shift += 5; continue;
        }
        else if (state >> shift) & 0b11111 == 0b01111 {
          super::board::WON[state as usize] |= super::board::FOUR_FLAG; shift += 5; continue;
        }
        else if (state >> shift) & 0b11111 == 0b11110{
          super::board::WON[state as usize] |= super::board::FOUR_FLAG; shift += 5; continue;
        }*/

        shift += 1;
      }
    }

    // Build the BT Table.
    println!("COMMENT: BUILDING BINARY - TERNARY TABLE");
    for state in 0..65536{
      super::board::BT[state as usize] = u32::from_str_radix(&format!("{:b}", state),3).unwrap();
      super::board::BT2[state as usize] = 2 * u32::from_str_radix(&format!("{:b}", state),3).unwrap();
    }

    // Build the Move and WON Table.
    // We might want to cache this.
    println!("COMMENT: NO CACHE FOUND. GENERATING...");
    let mut threads = vec![];
    threads.push(thread::spawn( || {
      binary_recurse(1 << 14,0,13);
    }));
    threads.push(thread::spawn(|| {
      binary_recurse(0,1 << 14,13);
    }));
    threads.push(thread::spawn(|| {
      binary_recurse(0,0,13);
    }));

    for t in threads {
      t.join();
    }
    super::board::MOVES[0] = [(0,0); 20];
  }
}


/**
 * Recurses through all the places.
 */
unsafe fn binary_recurse(you: u16, opp: u16, depth: i32) {
  // Trim :)
  if (super::board::WON[you as usize] | super::board::WON[opp as usize]) != 0 {
    return;
  }
  if depth < 0 {
    counter += 1;
    if counter % 1000000 == 0{
      println!("COMMENT: {:}% FINISHED", (((counter as f32) / 14348907f32) * 100f32).round());
    }
    let state: u32 = ((2 * (super::board::BT[opp as usize] as u32)) + (super::board::BT[you as usize] as u32));
    build_state(you, opp, state as usize);
    return;
  }


  binary_recurse(you | (1 << (depth as u16)), opp, depth - 1);
  binary_recurse(you, opp | (1 << (depth as u16)), depth - 1);
  binary_recurse(you, opp, depth - 1);
}


/**
 * Builds WON_TABLE and STATE_TABLE together
 */
// There are 11 places to check.
unsafe fn build_state(you: u16, opp: u16, state: usize) {
  let mut checking_3_y = false;
  let mut checking_3_o = false;


  let mut real_movs: Vec<(u8, i8)> = vec![];

  let mut contains = [16; 15];
  for i in 0..15 {
    if ((you >> i) | (opp >> i)) & 1 == 1 {
      contains[i as usize] = 1u8;
    }
  }

  

  let mut i = 0;
  while i < 15 {
    let you_state = (you >> i) & 0b11111;
    let opp_state = (opp >> i) & 0b11111;


      // Check for starting fours
      if you_state == 0b11110 && opp_state == 0 && i <= 10 {
        super::board::MOVES[state][0] = (i, FOUR_VALUE);
        super::board::VALUES[0][state] = 100000;
        super::board::VALUES[1][state] = 100000;
        return;
      }
      if opp_state == 0b11110 && you_state == 0 && i <= 10 {
        super::board::MOVES[state][0] = (i, -FOUR_VALUE);
        super::board::VALUES[0][state] = -100000;
        super::board::VALUES[1][state] = -100000;
        return;
      }

      // Check for starting fours
      if you_state == 0b01111 && opp_state == 0 && i <= 10 {
        super::board::MOVES[state][0] = (i + 4, FOUR_VALUE);
        super::board::VALUES[0][state] = 100000;
        super::board::VALUES[1][state] = 100000;
        return;
      }
      if opp_state == 0b01111 && you_state == 0 && i <= 10 {
        super::board::MOVES[state][0] = (i + 4, -FOUR_VALUE);
        super::board::VALUES[0][state] = -100000;
        super::board::VALUES[1][state] = -100000;
        return;
      }
    /* This is for you */
    // First check for 3s.
    if opp_state == 0 && you_state == 0b01110 && i <= 10 {
      if !(checking_3_y || checking_3_o) {
        real_movs.truncate(0);
      }
      real_movs.push((i, THREE_VALUE));
      real_movs.push((i+4, THREE_VALUE));
      checking_3_y = true;
      i += 5;
      continue;
    }
    /* This is for OPP */
    // First check for 3s
    if you_state == 0 && opp_state == 0b01110 && i <= 10 {
      if !(checking_3_y || checking_3_o) {
        real_movs.truncate(0);
      }
      real_movs.push((i, -THREE_VALUE));
      real_movs.push((i+4, -THREE_VALUE));
      checking_3_o = true;
      i += 5;
      continue;
    }

    if !(checking_3_y || checking_3_o) {
      if (you >> i) & 1 == 1 {
        let c = get_five(you, opp, i as u8, true);
        for i in c { real_movs.push(i);}
      } else if (opp >> i) & 1 == 1 {
        let c = get_five(you, opp, i as u8, false);
        for i in c { real_movs.push(i);}  
      }
    }

    i += 1;
  }

  if real_movs.len() == 0 {
    return;
  }

  real_movs.sort_by(|a,b| (b.0).cmp(&a.0));
  
  let mut you_mov = vec![];
  let mut value: i32 = 0;
  let mut i = 0;

  let a = real_movs.into_iter().fold((0,0i8 ), |cur, next| {
    if cur.0 == next.0{
      value += cur.1 as i32;
      return (cur.0, (cur.1.abs() + next.1.abs()));
    } else {
      you_mov.push(cur);
      value += next.1 as i32;
      return (next.0, next.1.abs());
    }
  });
  you_mov.push(a);
  you_mov.remove(0);
  you_mov.sort_by(|a,b| (b.1.abs()).cmp(&a.1.abs()));

  super::board::VALUES[0][state] = value;
  super::board::VALUES[1][state] = value;

  let mut i = 0;
  for mov in you_mov {
    super::board::MOVES[state][i] = mov;
    i += 1;
  }

}

unsafe fn get_five (you: u16, opp: u16, i: u8, your_move: bool) -> Vec<(u8, i8)> {
  let mut movs: Vec<(u8, i8)> = vec![];

  let mut index_right: u8 = 1;
  let mut index_left: u8 = 1;
  if your_move {
    while index_left < 5 {
      if (i + index_left) > 14 {break;}
      if (opp >> (i + index_left)) & 1 == 1 {break;}
      if (you >> (i + index_left)) & 1 == 0 {
        movs.push((i + index_left, (5 - index_left as i8)));
      }
      index_left += 1;
    } 
    if i > 0 {
      while index_right < 5 {
        if (i as i8 - index_right as i8) < 0 {break;}
        if (opp >> (i - index_right)) & 1 == 1 {break;}
        if (you >> (i - index_right)) & 1 == 0 {
          movs.push((i - index_right, (5 - index_right as i8)));
        }
        index_right += 1;
      }
    }
  } else {
    while index_left < 5 {
      if (i + index_left) > 14 {break;}
      if (you >> (i + index_left)) & 1 == 1 {break;}
      if (opp >> (i + index_left)) & 1 == 0 {
        movs.push((i + index_left, -(5 - index_left as i8)));
      }
      index_left += 1;
    }
    if i > 0 {
      while index_right < 5 {
        if (i as i8 - index_right as i8) < 0 {break;}
        if (you >> (i - index_right)) & 1 == 1 {break;}
        if (opp >> (i - index_right)) & 1 == 0 {
          movs.push((i - index_right, -(5 - index_right as i8)));
        }
        index_right += 1;
      }
    }
  }


  return movs;
}



/*

const LONGEST:[u8; 32] = [0,1,1,2,1,1,2,3,1,1,1,2,2,2,3,4,1,1,1,2,1,1,2,3,2,2,2,2,3,3,4,5];
const LENGTH:[u8; 32] = [0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4,1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5];

fn get_five(binary: u16, cur_shift: u16, you: bool) -> Vec<(u8, u8, bool)>{
  let mut movs: Vec<u8> = vec![];

  for shift in 0..5u16 {
    if (shift+cur_shift) > 14 {
      break;
    }
    if (binary >> shift) & 1 == 0 {
      movs.push((shift + cur_shift) as u8); 
    }
  }

  let mut value = 0;
  value = LONGEST[binary as usize];


  let mut mov_urg: Vec<(u8, u8, bool)> = vec![];

  for mov in &movs {
    mov_urg.push((*mov, value, you));
  }

  return mov_urg;
}

/**/
unsafe fn build_state(you: u16, opp: u16, state: usize) {
  let mut you_movs: Vec<(u8, u8, bool)> = vec![];

  for shift in 0..15u16{
    let you_state = (you >> shift) & 0b11111;
    let opp_state = (opp >> shift) & 0b11111;

    if (you_state == 0) && (opp_state == 0) {
      continue;
    }
    if you_state == 0 {
      let five_movs: Vec<(u8,u8, bool)> = get_five(opp_state, shift, true);
      if LENGTH[opp_state as usize] == 4 {
        super::board::MOVES[state][0] = (five_movs[0].0, FOUR_VALUE);
        super::board::VALUES[state] = -FOUR_STATE;
        return;
      }
      let mut five_iter = five_movs.iter();
      loop {
        match five_iter.next() {
          Some (x) => {
            you_movs.push(*x);
          }
          None => {break;}
        }
      }
    }
    if opp_state == 0 {
      let five_movs: Vec<(u8,u8, bool)> = get_five(you_state, shift, false);
      if LENGTH[you_state as usize] == 4 {
        super::board::MOVES[state][0] = (five_movs[0].0, FOUR_VALUE);
        super::board::VALUES[state] = FOUR_STATE;
        return;
      }
      let mut five_iter = five_movs.iter();
      loop {
        match five_iter.next() {
          Some (x) => {
            you_movs.push(*x);
          }
          None => {break;}
        }
      }
    }
  }

  let mut total_val = 0i32;
  you_movs.sort_by(|a,b| (a.1).cmp(&b.1));
  /*** FINISH HERE!!*/
  if you_movs.len() == 0 {
    return;
  }
  if you_movs[0].1 == 3 {
    let mut i = 0;
    for mov in you_movs {
      if mov.1 < 3 {
        return;
      }
      super::board::MOVES[state][i as usize] = (mov.0, THREE_VALUE);
      super::board::VALUES[state] += (THREE_VALUE as i32 / 2);
      i += 1;
    }
  } else {
    you_movs.sort_by(|a,b| (a.0).cmp(&b.0));

    let mut real_movs: Vec<(u8, u8)> = vec![];
    let mut first = false;
    let mut cur_mov = (15,15);

    for mov in &you_movs {
      if mov.0 == cur_mov.0 {
        cur_mov.1 += mov.1;
        total_val += (if mov.2 {1} else {-1}) * (mov.1 as i32);
      } else {
        total_val += (if mov.2 {1} else {-1}) * (mov.1 as i32);
        if first {
          real_movs.push(cur_mov);
          cur_mov = (mov.0, mov.1);
        }else {
          first = true;
          cur_mov = (mov.0, mov.1);
        }
      }
    }

    if first {
      real_movs.push(cur_mov);
    }

    let mut i:u8 = 0;
    for mov in &real_movs {
      super::board::MOVES[state][i as usize] = *mov;
      i += 1;
    }
    super::board::VALUES[state] = total_val;
  }
}


/**
 * Accepts a binary, returns a vector of tuples, with moves and urgency.
 */

const LONGEST:[u8; 32] = [0,1,1,2,1,1,2,3,1,1,1,2,2,2,3,4,1,1,1,2,1,1,2,3,2,2,2,2,3,3,4,5];
const LENGTH:[u8; 32] = [0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4,1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5];

fn get_five(binary: u16, cur_shift: u16, you: bool) -> Vec<(u8, u8, bool)>{
  let mut movs: Vec<u8> = vec![];

  for shift in 0..5u16 {
    if (shift+cur_shift) > 14 {
      break;
    }
    if (binary >> shift) & 1 == 0 {
      movs.push((shift + cur_shift) as u8); 
    }
  }

  let mut value = 0;/*
  match (movs.len() as u8) {
    1 => {
      value = 200;
    },
    2 => {
      value = 20;
    },
    3 => {
      value = 5;
    }
    4 => {
      value = 1;
    }
    _ => {}
  }
  */
  value = LONGEST[binary as usize];


  let mut mov_urg: Vec<(u8, u8, bool)> = vec![];

  for mov in &movs {
    mov_urg.push((*mov, value, you));
  }

  return mov_urg;
}


*/

    /*
    
     FILE READING CODE
    match File::open("WON_TABLE_CACHE.bin") {
      Ok(mut won_file) => {
        println!("COMMENT: CACHES FOUND, READING WON CACHE");
        let mut move_file = File::open("MOVE_TABLE_CACHE.bin").ok().unwrap();
        won_file.read(&mut super::board::WON);
        println!("COMMENT: CACHES FOUND, READING MOVE CACHE");

        unsafe {
          move_file.read(&mut tf);
          let mut i = 0;
          while i < 14348907 {
            super::board::MOVES[i] = [(tf[i+0],tf[i+1]),(tf[i+2],tf[i+3]),(tf[i+4],tf[i+5]),(tf[i+6],tf[i+7]),(tf[i+8],tf[i+9]),(tf[i+10],tf[i+11]),(tf[i+12],tf[i+13]),(tf[i+14],tf[i+15]),(tf[i+16],tf[i+17]),(tf[i+18],tf[i+19]),(tf[i+20],tf[i+21]),(tf[i+22],tf[i+23]),(tf[i+24],tf[i+25]),(tf[i+26],tf[i+27]),(tf[28],tf[29])];
            i += 30;
          }
        }
      }
      Err(e) => {
        println!("COMMENT: NO CACHE FOUND. GENERATING...");
        binary_recurse(0,0,14);
        println!("COMMENT: FINISHED GENERATING, WRITING FILES.");

        //let mut won_file = File::create("WON_TABLE_CACHE.bin").ok().unwrap();
        //won_file.write(&super::board::WON);
        let mut move_file = File::create("MOVE_TABLE_CACHE.bin").ok().unwrap();

        unsafe {
          let c = std::mem::transmute::<&[[(u8,u8); 15]; 14348907], &[u8; 430467210]>(&super::board::MOVES);
          move_file.write(c);
        }
      }
    }
    */