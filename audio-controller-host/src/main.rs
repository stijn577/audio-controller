use wasapi::Direction;

fn main() {
    wasapi::initialize_mta().ok();
    wasapi::initialize_sta().ok();

    let input = wasapi::DeviceCollection::new(&Direction::Render).unwrap();
    let output = wasapi::DeviceCollection::new(&Direction::Capture).unwrap();

    input
        .into_iter()
        .for_each(|device| println!("{:?}", device.unwrap().get_friendlyname()));

    output
        .into_iter()
        .for_each(|device| println!("{:?}", device.unwrap().get_friendlyname()));
}
