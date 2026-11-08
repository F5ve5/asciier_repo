pub type Errorr = Box<dyn std::error::Error>;
pub type Resultt<T> = std::result::Result<T, Errorr>;

pub fn map_u8_to_x(u8:u8,x:usize) -> usize{
    (u8 as usize * x/256)
}