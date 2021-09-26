mod frominteger {

    #[derive(Debug)]
    struct MySimpleStruct {
        elem: i32,
    }

    impl From<i32> for MySimpleStruct {
        fn from(v: i32) -> Self {
            MySimpleStruct { elem: v }
        }
    }

    #[test]
    fn init_simple_struct() {
        let s = MySimpleStruct::from(42);
        assert_eq!(s.elem, 42);
        println!("my simple struct: {:?}", s)
    }

}

mod fromvector {
    #[derive(Debug)]
    struct MyVectorStruct {
        elem: Vec<i32>,
    }

    impl <'a, T: AsRef<[i32]>> From<T> for MyVectorStruct {
        fn from(v: T) -> Self {
            MyVectorStruct { elem: v.as_ref().to_vec()}
        }
    }

    #[test]
    fn init_vector_struct() {
        let s1 = MyVectorStruct::from(vec![1, 2, 3]);
        assert_eq!(s1.elem.len(), 3);
        println!("Vector struct: {:?}", s1);

        let s2 = MyVectorStruct::from([4, 5, 6]);
        assert_eq!(s2.elem.len(), 3);
        println!("Vector struct: {:?}", s2);

    }

}

mod fromvectortobitmask {
    #[derive(Debug)]
    struct MyBitMap {
        bm: u64,
    }

    impl MyBitMap {
        pub fn set_bit(&mut self, v: usize) {
            if v < 64 {
                self.bm |= 1 << v;
            }
        }
    }

    impl <'a, T: AsRef<[usize]>> From<T> for MyBitMap {
        fn from(v: T) -> Self {
            let mut bm = MyBitMap { bm: 0};
            for bitn in v.as_ref().to_vec() {
                bm.set_bit(bitn);
            }
            bm
        }
    }

    #[test]
    fn init_mybitmap() {
        let b5 = MyBitMap::from(vec![0, 1, 2, 3]);
        assert_eq!(b5.bm, 0x00_00_00_00_00_00_00_0F);
    }
}