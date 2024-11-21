use cantypes::CanFrame;

fn main() {
    let frame = CanFrame::new_data_frame(0x3FF, cantypes::IdType::Standard, &[0xFF,2,0xA3,4]);
    println!("{:?}", frame);
}