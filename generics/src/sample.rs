pub trait Summarizable2 {
    fn summarize() -> ();
}

#[derive(Debug)]
pub struct SampleStruct2 {
    value: i32
}

impl SampleStruct2 {
    pub fn new(value: i32) -> SampleStruct2 {
        SampleStruct2 {
            value
        }
    }
}

impl Summarizable2 for SampleStruct2 {
    fn summarize() -> () {
        println!("Alternative summarize.");
    }
}