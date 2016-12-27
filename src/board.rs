// The array that allows binary to ternary conversion
pub static mut BT: [u32; 65536] = [0; 65536];

// The arrays for move lookup
pub static mut MOVES: [[(u8,u8); 15]; 14348907] = [[(0,0); 15]; 14348907];

// The array for win lookup
pub static mut WON: [u8; 65536] = [0; 65536]; 

pub struct Board {
  pub multi: [[u8; 15]; 15],

  pub horiz_y: [u16; 15],
  pub horiz_o: [u16; 15],
  pub verti_y: [u16; 15],
  pub verti_o: [u16; 15],

  pub diagr_y: [u16; 19],
  pub diagr_o: [u16; 19],
  pub diagl_y: [u16; 19],
  pub diagl_o: [u16; 19]
}



pub static HORIZ: [(usize, usize); 225] = [
  (0,  14),  (0, 13),  (0, 12),  (0, 11),  (0, 10),  (0, 9),  (0, 8),  (0, 7),  (0, 6),  (0, 5),  (0, 4),  (0, 3),  (0, 2),  (0, 1),  (0, 0),
  (1,  14),  (1, 13),  (1, 12),  (1, 11),  (1, 10),  (1, 9),  (1, 8),  (1, 7),  (1, 6),  (1, 5),  (1, 4),  (1, 3),  (1, 2),  (1, 1),  (1, 0),
  (2,  14),  (2, 13),  (2, 12),  (2, 11),  (2, 10),  (2, 9),  (2, 8),  (2, 7),  (2, 6),  (2, 5),  (2, 4),  (2, 3),  (2, 2),  (2, 1),  (2, 0),
  (3,  14),  (3, 13),  (3, 12),  (3, 11),  (3, 10),  (3, 9),  (3, 8),  (3, 7),  (3, 6),  (3, 5),  (3, 4),  (3, 3),  (3, 2),  (3, 1),  (3, 0),
  (4,  14),  (4, 13),  (4, 12),  (4, 11),  (4, 10),  (4, 9),  (4, 8),  (4, 7),  (4, 6),  (4, 5),  (4, 4),  (4, 3),  (4, 2),  (4, 1),  (4, 0),
  (5,  14),  (5, 13),  (5, 12),  (5, 11),  (5, 10),  (5, 9),  (5, 8),  (5, 7),  (5, 6),  (5, 5),  (5, 4),  (5, 3),  (5, 2),  (5, 1),  (5, 0),
  (6,  14),  (6, 13),  (6, 12),  (6, 11),  (6, 10),  (6, 9),  (6, 8),  (6, 7),  (6, 6),  (6, 5),  (6, 4),  (6, 3),  (6, 2),  (6, 1),  (6, 0),
  (7,  14),  (7, 13),  (7, 12),  (7, 11),  (7, 10),  (7, 9),  (7, 8),  (7, 7),  (7, 6),  (7, 5),  (7, 4),  (7, 3),  (7, 2),  (7, 1),  (7, 0),
  (8,  14),  (8, 13),  (8, 12),  (8, 11),  (8, 10),  (8, 9),  (8, 8),  (8, 7),  (8, 6),  (8, 5),  (8, 4),  (8, 3),  (8, 2),  (8, 1),  (8, 0),
  (9,  14),  (9, 13),  (9, 12),  (9, 11),  (9, 10),  (9, 9),  (9, 8),  (9, 7),  (9, 6),  (9, 5),  (9, 4),  (9, 3),  (9, 2),  (9, 1),  (9, 0),
  (10, 14), (10, 13), (10, 12), (10, 11), (10, 10), (10, 9), (10, 8), (10, 7), (10, 6), (10, 5), (10, 4), (10, 3), (10, 2), (10, 1), (10, 0),
  (11, 14), (11, 13), (11, 12), (11, 11), (11, 10), (11, 9), (11, 8), (11, 7), (11, 6), (11, 5), (11, 4), (11, 3), (11, 2), (11, 1), (11, 0),
  (12, 14), (12, 13), (12, 12), (12, 11), (12, 10), (12, 9), (12, 8), (12, 7), (12, 6), (12, 5), (12, 4), (12, 3), (12, 2), (12, 1), (12, 0),
  (13, 14), (13, 13), (13, 12), (13, 11), (13, 10), (13, 9), (13, 8), (13, 7), (13, 6), (13, 5), (13, 4), (13, 3), (13, 2), (13, 1), (13, 0),
  (14, 14), (14, 13), (14, 12), (14, 11), (14, 10), (14, 9), (14, 8), (14, 7), (14, 6), (14, 5), (14, 4), (14, 3), (14, 2), (14, 1), (14, 0)
];

pub static VERTI: [(usize, usize); 225] = [
 (0, 14), (1, 14), (2, 14), (3, 14), (4, 14), (5, 14), (6, 14), (7, 14), (8, 14), (9, 14), (10, 14), (11, 14), (12, 14), (13, 14), (14, 14),
 (0, 13), (1, 13), (2, 13), (3, 13), (4, 13), (5, 13), (6, 13), (7, 13), (8, 13), (9, 13), (10, 13), (11, 13), (12, 13), (13, 13), (14, 13),
 (0, 12), (1, 12), (2, 12), (3, 12), (4, 12), (5, 12), (6, 12), (7, 12), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (14, 12),
 (0, 11), (1, 11), (2, 11), (3, 11), (4, 11), (5, 11), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11), (11, 11), (12, 11), (13, 11), (14, 11),
 (0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (5, 10), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (13, 10), (14, 10),
 (0, 09), (1, 09), (2, 09), (3, 09), (4, 09), (5, 09), (6, 09), (7, 09), (8, 09), (9, 09), (10, 09), (11, 09), (12, 09), (13, 09), (14, 09),
 (0, 08), (1, 08), (2, 08), (3, 08), (4, 08), (5, 08), (6, 08), (7, 08), (8, 08), (9, 08), (10, 08), (11, 08), (12, 08), (13, 08), (14, 08),
 (0, 07), (1, 07), (2, 07), (3, 07), (4, 07), (5, 07), (6, 07), (7, 07), (8, 07), (9, 07), (10, 07), (11, 07), (12, 07), (13, 07), (14, 07),
 (0, 06), (1, 06), (2, 06), (3, 06), (4, 06), (5, 06), (6, 06), (7, 06), (8, 06), (9, 06), (10, 06), (11, 06), (12, 06), (13, 06), (14, 06),
 (0, 05), (1, 05), (2, 05), (3, 05), (4, 05), (5, 05), (6, 05), (7, 05), (8, 05), (9, 05), (10, 05), (11, 05), (12, 05), (13, 05), (14, 05),
 (0, 04), (1, 04), (2, 04), (3, 04), (4, 04), (5, 04), (6, 04), (7, 04), (8, 04), (9, 04), (10, 04), (11, 04), (12, 04), (13, 04), (14, 04),
 (0, 03), (1, 03), (2, 03), (3, 03), (4, 03), (5, 03), (6, 03), (7, 03), (8, 03), (9, 03), (10, 03), (11, 03), (12, 03), (13, 03), (14, 03),
 (0, 02), (1, 02), (2, 02), (3, 02), (4, 02), (5, 02), (6, 02), (7, 02), (8, 02), (9, 02), (10, 02), (11, 02), (12, 02), (13, 02), (14, 02),
 (0, 01), (1, 01), (2, 01), (3, 01), (4, 01), (5, 01), (6, 01), (7, 01), (8, 01), (9, 01), (10, 01), (11, 01), (12, 01), (13, 01), (14, 01),
 (0, 00), (1, 00), (2, 00), (3, 00), (4, 00), (5, 00), (6, 00), (7, 00), (8, 00), (9, 00), (10, 00), (11, 00), (12, 00), (13, 00), (14, 00),
];


pub static HORIZ_ARRS:[[u8; 15]; 15] = [
  [000,001,002,003,004,005,006,007,008,009,010,011,012,013,014],
  [015,016,017,018,019,020,021,022,023,024,025,026,027,028,029],
  [030,031,032,033,034,035,036,037,038,039,040,041,042,043,044],
  [045,046,047,048,049,050,051,052,053,054,055,056,057,058,059],
  [060,061,062,063,064,065,066,067,068,069,070,071,072,073,074],
  [075,076,077,078,079,080,081,082,083,084,085,086,087,088,089],
  [090,091,092,093,094,095,096,097,098,099,100,101,102,103,104],
  [105,106,107,108,109,110,111,112,113,114,115,116,117,118,119],
  [120,121,122,123,124,125,126,127,128,129,130,131,132,133,134],
  [135,136,137,138,139,140,141,142,143,144,145,146,147,148,149],
  [150,151,152,153,154,155,156,157,158,159,160,161,162,163,164],
  [165,166,167,168,169,170,171,172,173,174,175,176,177,178,179],
  [180,181,182,183,184,185,186,187,188,189,190,191,192,193,194],
  [195,196,197,198,199,200,201,202,203,204,205,206,207,208,209],
  [210,211,212,213,214,215,216,217,218,219,220,221,222,223,224]
];

pub static VERTI_ARRS:[[u8; 15]; 15] = [
  [000,015,030,045,060,075,090,105,120,135,150,165,180,195,210],
  [001,020,031,046,061,076,091,106,121,136,151,166,181,196,211],
  [002,018,032,047,062,077,092,107,122,137,152,167,182,197,212],
  [003,018,033,048,063,078,093,108,123,138,153,168,183,198,213],
  [004,018,034,049,064,079,094,109,124,139,154,169,184,199,214],
  [005,028,035,050,065,080,095,110,125,140,155,170,185,200,215],
  [006,028,036,051,066,081,096,111,126,141,156,171,186,201,216],
  [007,028,037,052,067,082,097,112,127,142,157,172,187,202,217],
  [008,028,038,053,068,083,098,113,128,143,158,173,188,203,218],
  [009,028,039,054,069,084,099,114,129,144,159,174,189,204,219],
  [010,028,040,055,070,085,100,115,130,145,160,175,190,205,220],
  [011,028,041,056,071,086,101,116,131,146,161,176,191,206,221],
  [012,028,042,057,072,087,102,117,132,147,162,177,192,207,222],
  [013,028,043,058,073,088,103,118,133,148,163,178,193,208,223],
  [014,029,044,059,074,089,104,119,134,149,164,179,194,209,224]
];

impl Board {
  pub fn won (&self, you: bool) -> bool {
    unsafe {
      if you {
        for i in 0..15usize {
          if WON[self.verti_y[i] as usize] != 0 || WON[self.horiz_y[i] as usize] != 0 {
            return true;
          }
        }
      } else {
        for i in 0..15usize {
          if WON[self.verti_o[i] as usize] != 0 || WON[self.horiz_o[i] as usize] != 0 {
            return true;
          }
        }
      }
    }
    return false;
  }
  pub fn gen_moves (&self) -> Vec<(u8,u8)> {
    let mut movs:Vec<(u8,u8)> = vec![];
    unsafe {
      for i in 0..15usize {
        let v_state = (2 * BT[self.verti_y[i] as usize]) + BT[self.verti_o[i] as usize];
        let h_state = (2 * BT[self.horiz_y[i] as usize]) + BT[self.horiz_o[i] as usize];

        let v_movs: Vec<(u8,u8)> = MOVES[v_state as usize].iter().map(|el| (el.1, VERTI_ARRS[i][14usize - el.0 as usize])).filter(|x| x.0 != 0).collect();
        movs.extend(&v_movs);
        let h_movs: Vec<(u8,u8)> = MOVES[h_state as usize].iter().map(|el| (el.1, HORIZ_ARRS[i][14usize - el.0 as usize])).filter(|x| x.0 != 0).collect();
        movs.extend(&h_movs);
      }
    }
    return movs;
  }

  pub fn place_piece (&mut self, place: usize, you: bool) {
    unsafe {
      if you {
        self.place_horiz_you(place);
        self.place_verti_you(place);
      } else {
        self.place_horiz_opp(place);
        self.place_verti_opp(place);
      }
    }
  }

  pub fn remove_piece (&mut self, place: usize, you: bool) {
    unsafe {
      if you {
        self.remove_horiz_you(place);
        self.remove_verti_you(place);
      } else {
        self.remove_horiz_opp(place);
        self.remove_verti_opp(place);
      }
    }
  }

  pub unsafe fn place_horiz_you (&mut self, place: usize) {
    let mov = HORIZ[place];
    self.horiz_y[mov.0] |= (1 << mov.1); 
  }
  pub unsafe fn place_verti_you (&mut self, place: usize) {
    let mov = VERTI[place];
    self.verti_y[mov.0] |= (1 << mov.1); 
  }
  pub unsafe fn place_horiz_opp (&mut self, place: usize) {
    let mov = HORIZ[place];
    self.horiz_o[mov.0] |= (1 << mov.1); 
  }
  pub unsafe fn place_verti_opp (&mut self, place: usize) {
    let mov = VERTI[place];
    self.verti_o[mov.0] |= (1 << mov.1); 
  }

  pub unsafe fn remove_horiz_you (&mut self, place: usize) {
    let mov = HORIZ[place];
    self.horiz_y[mov.0] ^= (1 << mov.1); 
  }
  pub unsafe fn remove_verti_you (&mut self, place: usize) {
    let mov = VERTI[place];
    self.verti_y[mov.0] ^= (1 << mov.1); 
  }
  pub unsafe fn remove_horiz_opp (&mut self, place: usize) {
    let mov = HORIZ[place];
    self.horiz_o[mov.0] ^= (1 << mov.1); 
  }
  pub unsafe fn remove_verti_opp (&mut self, place: usize) {
    let mov = VERTI[place];
    self.verti_o[mov.0] ^= (1 << mov.1); 
  }
}

