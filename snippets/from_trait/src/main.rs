// https://www.reddit.com/r/rust/comments/70xqpw/using_the_from_trait_not_as_easy_as_i_thought/
// https://users.rust-lang.org/t/generic-trait-and-specialization/33745
#![feature(specialization)]

#[derive(Debug)]
pub struct BitBoard {
    state: u64,
}

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard { state: 0u64 }
    }
}

// --- From traits ---------------------------------------

// From anything that can be converted to &[u8] --- DEFAULT IMPLEMENTATION
// This conflicts with From<u64> and From<u8>
impl<'a, T: AsRef<[u8]>> From<T> for BitBoard {
    default fn from(v: T) -> Self {
        let mut bb = BitBoard::new();
        for s in v.as_ref().to_vec() {
            if s < 64 {
                bb.state |= 1 << s;
            }
        }
        bb
    }
}


// From<u8>
//impl From<u8> for BitBoard {
//    fn from(s: u8) -> Self {
//        let mut bb = BitBoard::new();
//        if s < 64 {
//            bb.state = 1 << s;
//        }
//        bb
//    }
//}

//impl<'a, T: AsRef<[u8]>> From<T> for BitBoard {
//    fn from(v: T) -> Self {
//        let mut bb = BitBoard::new();
//        for s in v.as_ref().to_vec() {
//            if s < 64 {
//                bb.state |= 1 << s;
//            }
//        }
//        bb
//    }
//}





// From<u64>
//impl From<u64> for BitBoard {
//    fn from(s: u64) -> Self {
//        let mut bb = BitBoard::new();
//        bb.state = s;
//        bb
//    }
//}
// From<u8>
//impl From<u8> for BitBoard {
//    fn from(s: u8) -> Self {
//        let mut bb = BitBoard::new();
//        if s < 64 {
//            bb.state = 1 << s;
//        }
//        bb
//    }
//}


// From &[u8]
//impl From<&[u8]> for BitBoard {
//    fn from(v: &[u8]) -> Self {
//        let mut bb = BitBoard::new();
//        for s in v {
//            if *s < 64 {
//                bb.state |= 1 << s;
//            }
//        }
//        bb
//    }
//}

// From anything that can be converted into Vec<u8>
// This conflicts with From<u64> and From<u8>
//impl<'a, T: Into<&'a Vec<u8>>> From<T> for BitBoard {
//    fn from(v: T) -> Self {
//        let mut bb = BitBoard::new();
//        for s in v.into() {
//            if *s < 64 {
//                bb.state |= 1 << s;
//            }
//        }
//        bb
//    }
//}



// -------------------------------------------------------

// -------------------------------------------------------
fn main() {
    let bb_new = BitBoard::new();
    println!("Hello, BitBoard: {:?}", bb_new);

    //let bb_from_u64 = BitBoard::from(1u64);
    //println!("Hello, BitBoard: {:?}", bb_from_u64);

    //let bb_from_u8 = BitBoard::from(4u8);
    //println!("Hello, BitBoard: {:?}", bb_from_u8);

    // This is not good, because From<i32> is not defined:
    // let bb_from_unknown = BitBoard::from(8);
    // println!("Hello, BitBoard: {:?}", bb_from_unknown);

    let bb_from_vec_of_u8 = BitBoard::from(&[7u8, 8u8]); // = 384
    println!("Hello, BitBoard: {:?}", bb_from_vec_of_u8);

}
// -------------------------------------------------------
