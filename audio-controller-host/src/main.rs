use wasapi::Direction;

fn main() {
    let input = wasapi::DeviceCollection::new(&Direction::Capture).unwrap();
    let output = wasapi::DeviceCollection::new(&Direction::Render).unwrap();

    println!("{:?}", input);
    println!("{:?}", output);
}
