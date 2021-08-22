#[derive(Debug, Copy, Clone)]
enum BroomIntent {FetchWater, DumpWater}

#[derive(Debug, Clone, Copy)]
struct BroomStaticData {
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent
}
impl BroomStaticData {
    fn new(height: u32, health: u32, position: (f32, f32, f32), intent: BroomIntent ) -> BroomStaticData {
        BroomStaticData { height, health, position, intent}
    }
    fn chop(self) -> (BroomStaticData, BroomStaticData) {
        let mut b1 = self;  // self moved on b1
        b1.height /= 2;  // b1 is chopped... reduce height
        let b2 = b1.clone();  // b1 copied on b2
        (b1, b2)
    }
}

#[derive(Debug, Clone)]
struct Broom {
    name: String,
    static_data: BroomStaticData,
}
impl Broom {
    fn new(name: &str, height: u32, health: u32, position: (f32, f32, f32), intent: BroomIntent ) -> Broom {
        Broom { name: name.to_string(), static_data: BroomStaticData {height, health, position, intent} }
    }
    fn chop(self) -> (Broom, Broom) {
        // chop the static data
        let (bsd1, bsd2) = self.static_data.chop();
        let mut b1 = self;  // self moved on b1
        b1.static_data = bsd1;
        let mut b2 = b1.clone();  // b1 copied on b2
        b2.static_data = bsd2;
        b1.name.push_str("_1");
        b2.name.push_str("_2");
        (b1, b2)
    }
}

fn main() {

    // 1. Broom struct does not implement copy, so we have to
    //    respect the move rust semantic
    println!("----------------------------------------------------");
    // We can avoid to specify the Vec type, rust type inference will do
    // the work for us:
    let mut brooms = Vec::new();
    // Otherwise, we can use the turbofish operator ::<>
    // let mut brooms = Vec::<Broom>::new();
    // another way to define the vector:
    // let mut brooms : Vec<Broom> = Vec::new();

    let b = Broom::new("Broom", 60, 100, (100.0, 200.0, 0.0), BroomIntent::FetchWater);
    let (b1, b2) = b.chop();  // <-- move, b no more exists
    // Add brooms to brooms vector
    brooms.push(b1);   // <--- move. After this we cannot use b1 anymore
    brooms.push(b2);   // <--- move. After this we cannot use b2 anymore
    println!("Brooms: {:?}", brooms);

    // 2. With BroomStaticData things works differently,
    //    because it implements copy
    println!("----------------------------------------------------");
    let mut ubrooms : Vec<BroomStaticData> = Vec::new();
    let ub1 = BroomStaticData::new( 60, 100, (100.0, 200.0, 0.0), BroomIntent::DumpWater);
    let ub2 = ub1; // <-- copy, ub1 continue to exists
    ubrooms.push(ub1);   // <--- copy, ub1 continue to exists
    ubrooms.push(ub2);   // <--- copy, ub2 continue to exists
    println!("Unnamed Broom 1 & 2: {:?}, {:?}", ub1, ub2);
    println!("Unnamed Brooms: {:?}", ubrooms);

    // 3. Returns to Broom. Suppose that we want to chop the first element
    //    of the brooms vector. We have to extract it before, chop it and then
    //    then add back the new chopped brooms
    println!("----------------------------------------------------");
    let (b1_1, b1_2) = brooms.remove(0).chop();
    println!("Broom 1_1: {:?}", b1_1);
    println!("Broom 1_2: {:?}", b1_2);
    brooms.push(b1_1);   // <--- move. After this we cannot use b1_1 anymore
    brooms.push(b1_2);   // <--- move. After this we cannot use b1_2 anymore
    println!("Brooms: {:?}", brooms);


}
