#![feature(never_type)]
use anyhow::Context;
use futures_lite::future::block_on;
use log::info;
use rusb::Device;
use rusb::DeviceDescriptor;
use rusb::DeviceHandle;
use rusb::DeviceList;
use rusb::GlobalContext;
use rusb::Language;
use rusb::UsbContext;
use shared_data::action::Action;
use shared_data::message::Message;
use tokio::time::Duration;

mod hardware_rx;
mod os_commands;

const TIMEOUT: Duration = Duration::from_secs(1);
#[derive(Debug)]
pub struct UsbDevice<T: UsbContext> {
    pub dev_desc: DeviceDescriptor,
    pub handle: DeviceHandle<T>,
    pub language: Language,
}

impl<T: UsbContext> UsbDevice<T> {
    fn match_name(&self, name: &str) -> bool {
        if let Ok(p_name) = self
            .handle
            .read_product_string(self.language, &self.dev_desc, TIMEOUT)
        {
            if name == &p_name {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_name(&self) -> Option<String> {
        self.handle
            .read_product_string(self.language, &self.dev_desc, TIMEOUT)
            .ok()
    }

    fn create(dev: Device<GlobalContext>) -> Option<UsbDevice<GlobalContext>> {
        if let Ok(dev_desc) = dev.device_descriptor() {
            match dev.open() {
                Ok(handle) => match handle.read_languages(TIMEOUT) {
                    Ok(lang) => {
                        if !lang.is_empty() {
                            Some(UsbDevice {
                                dev_desc,
                                handle,
                                language: lang[0],
                            })
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                },
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    loop {
        if let Some(Some(serial)) = DeviceList::new()?
            .iter()
            .map(|dev| UsbDevice::<GlobalContext>::create(dev))
            .find(|dev| {
                if let Some(dev) = dev {
                    dev.match_name("audio-controller")
                } else {
                    false
                }
            })
        {
            info!("{:?}", serial.get_name());

            let msg = Message::Action(Action::Command(vec![String::from("firefox.exe")]));
            let raw = msg.serialize()?;

            serial.handle.claim_interface(0)?;

            info!("{:?}", serial.handle.write_interrupt(0x81, &raw, TIMEOUT));
        }
        loop {}
    }

    Ok(())
}

// fn list_devices() -> anyhow::Result<()> {
//     let TIMEOUT = Duration::from_secs(1);

//     for device in DeviceList::new()?.iter() {
//         let device_desc = match device.device_descriptor() {
//             Ok(d) => d,
//             Err(_) => continue,
//         };

//         let mut usb_device = {
//             match device.open() {
//                 Ok(h) => match h.read_languages(TIMEOUT) {
//                     Ok(l) => {
//                         if !l.is_empty() {
//                             Some(UsbDevice {
//                                 handle: h,
//                                 language: l[0],
//                                 TIMEOUT,
//                             })
//                         } else {
//                             None
//                         }
//                     }
//                     Err(_) => None,
//                 },
//                 Err(_) => None,
//             }
//         };

//         println!(
//             "Bus {:03} Device {:03} ID {:04x}:{:04x} {}",
//             device.bus_number(),
//             device.address(),
//             device_desc.vendor_id(),
//             device_desc.product_id(),
//             get_speed(device.speed())
//         );
//         print_device(&device_desc, &mut usb_device);

//         for n in 0..device_desc.num_configurations() {
//             let config_desc = match device.config_descriptor(n) {
//                 Ok(c) => c,
//                 Err(_) => continue,
//             };

//             print_config(&config_desc, &mut usb_device);

//             for interface in config_desc.interfaces() {
//                 for interface_desc in interface.descriptors() {
//                     print_interface(&interface_desc, &mut usb_device);

//                     for endpoint_desc in interface_desc.endpoint_descriptors() {
//                         print_endpoint(&endpoint_desc);
//                     }
//                 }
//             }
//         }
//     }

//     Ok(())
// }

// fn print_device<T: UsbContext>(device_desc: &DeviceDescriptor, handle: &mut Option<UsbDevice<T>>) {
//     let vid = device_desc.vendor_id();
//     let pid = device_desc.product_id();

//     let vendor_name = match usb_ids::Vendor::from_id(device_desc.vendor_id()) {
//         Some(vendor) => vendor.name(),
//         None => "Unknown vendor",
//     };
// let product_name =
// match Device::from_vid_pid(device_desc.vendor_id(), device_desc.product_id()) {
//         Some(product) => product.name(),
//         None => "Unknown product",
//     };
//     println!("Device Descriptor:");
//     println!("  bLength              {:3}", device_desc.length());
//     println!("  bDescriptorType      {:3}", device_desc.descriptor_type());
//     println!(
//         "  bcdUSB             {:2}.{}{}",
//         device_desc.usb_version().major(),
//         device_desc.usb_version().minor(),
//         device_desc.usb_version().sub_minor()
//     );
//     println!("  bDeviceClass        {:#04x}", device_desc.class_code());
//     println!(
//         "  bDeviceSubClass     {:#04x}",
//         device_desc.sub_class_code()
//     );
//     println!("  bDeviceProtocol     {:#04x}", device_desc.protocol_code());
//     println!("  bMaxPacketSize0      {:3}", device_desc.max_packet_size());
//     println!("  idVendor          {vid:#06x} {vendor_name}",);
//     println!("  idProduct         {pid:#06x} {product_name}",);
//     println!(
//         "  bcdDevice          {:2}.{}{}",
//         device_desc.device_version().major(),
//         device_desc.device_version().minor(),
//         device_desc.device_version().sub_minor()
//     );
//     println!(
//         "  iManufacturer        {:3} {}",
//         device_desc.manufacturer_string_index().unwrap_or(0),
//         handle.as_mut().map_or(String::new(), |h| h
//             .handle
//             .read_manufacturer_string(h.language, device_desc, h.TIMEOUT)
//             .unwrap_or_default())
//     );
//     println!(
//         "  iProduct             {:3} {}",
//         device_desc.product_string_index().unwrap_or(0),
//         handle.as_mut().map_or(String::new(), |h| h
//             .handle
//             .read_product_string(h.language, device_desc, h.TIMEOUT)
//             .unwrap_or_default())
//     );
//     println!(
//         "  iSerialNumber        {:3} {}",
//         device_desc.serial_number_string_index().unwrap_or(0),
//         handle.as_mut().map_or(String::new(), |h| h
//             .handle
//             .read_serial_number_string(h.language, device_desc, h.TIMEOUT)
//             .unwrap_or_default())
//     );
//     println!(
//         "  bNumConfigurations   {:3}",
//         device_desc.num_configurations()
//     );
// }

// fn print_config<T: UsbContext>(config_desc: &ConfigDescriptor, handle: &mut Option<UsbDevice<T>>) {
//     println!("  Config Descriptor:");
//     println!("    bLength              {:3}", config_desc.length());
//     println!(
//         "    bDescriptorType      {:3}",
//         config_desc.descriptor_type()
//     );
//     println!("    wTotalLength      {:#06x}", config_desc.total_length());
//     println!(
//         "    bNumInterfaces       {:3}",
//         config_desc.num_interfaces()
//     );
//     println!("    bConfigurationValue  {:3}", config_desc.number());
//     println!(
//         "    iConfiguration       {:3} {}",
//         config_desc.description_string_index().unwrap_or(0),
//         handle.as_mut().map_or(String::new(), |h| h
//             .handle
//             .read_configuration_string(h.language, config_desc, h.TIMEOUT)
//             .unwrap_or_default())
//     );
//     println!("    bmAttributes:");
//     println!("      Self Powered     {:>5}", config_desc.self_powered());
//     println!("      Remote Wakeup    {:>5}", config_desc.remote_wakeup());
//     println!("    bMaxPower           {:4}mW", config_desc.max_power());

//     if !config_desc.extra().is_empty() {
//         println!("    {:?}", config_desc.extra());
//     } else {
//         println!("    no extra data");
//     }
// }

// fn print_interface<T: UsbContext>(
//     interface_desc: &InterfaceDescriptor,
//     handle: &mut Option<UsbDevice<T>>,
// ) {
//     println!("    Interface Descriptor:");
//     println!("      bLength              {:3}", interface_desc.length());
//     println!(
//         "      bDescriptorType      {:3}",
//         interface_desc.descriptor_type()
//     );
//     println!(
//         "      bInterfaceNumber     {:3}",
//         interface_desc.interface_number()
//     );
//     println!(
//         "      bAlternateSetting    {:3}",
//         interface_desc.setting_number()
//     );
//     println!(
//         "      bNumEndpoints        {:3}",
//         interface_desc.num_endpoints()
//     );
//     println!(
//         "      bInterfaceClass     {:#04x}",
//         interface_desc.class_code()
//     );
//     println!(
//         "      bInterfaceSubClass  {:#04x}",
//         interface_desc.sub_class_code()
//     );
//     println!(
//         "      bInterfaceProtocol  {:#04x}",
//         interface_desc.protocol_code()
//     );
//     println!(
//         "      iInterface           {:3} {}",
//         interface_desc.description_string_index().unwrap_or(0),
//         handle.as_mut().map_or(String::new(), |h| h
//             .handle
//             .read_interface_string(h.language, interface_desc, h.TIMEOUT)
//             .unwrap_or_default())
//     );

//     if interface_desc.extra().is_empty() {
//         println!("    {:?}", interface_desc.extra());
//     } else {
//         println!("    no extra data");
//     }
// }

// fn print_endpoint(endpoint_desc: &EndpointDescriptor) {
//     println!("      Endpoint Descriptor:");
//     println!("        bLength              {:3}", endpoint_desc.length());
//     println!(
//         "        bDescriptorType      {:3}",
//         endpoint_desc.descriptor_type()
//     );
//     println!(
//         "        bEndpointAddress    {:#04x} EP {} {:?}",
//         endpoint_desc.address(),
//         endpoint_desc.number(),
//         endpoint_desc.direction()
//     );
//     println!("        bmAttributes:");
//     println!(
//         "          Transfer Type          {:?}",
//         endpoint_desc.transfer_type()
//     );
//     println!(
//         "          Synch Type             {:?}",
//         endpoint_desc.sync_type()
//     );
//     println!(
//         "          Usage Type             {:?}",
//         endpoint_desc.usage_type()
//     );
//     println!(
//         "        wMaxPacketSize    {:#06x}",
//         endpoint_desc.max_packet_size()
//     );
//     println!(
//         "        bInterval            {:3}",
//         endpoint_desc.interval()
//     );
// }

// fn get_speed(speed: Speed) -> &'static str {
//     match speed {
//         Speed::SuperPlus => "10000 Mbps",
//         Speed::Super => "5000 Mbps",
//         Speed::High => " 480 Mbps",
//         Speed::Full => "  12 Mbps",
//         Speed::Low => " 1.5 Mbps",
//         _ => "(unknown)",
//     }
// }

// // #[tokio::main]
// async fn main() -> anyhow::Result<!> {
//     env_logger::init();

//     Ok(loop {})

    // let com = std::env::stdin();

    let port = tokio_serial::available_ports()
        .context("Could not list ports!")?
        .into_iter()
        .map(|dev| {
            info!("Found port: {:#?}", dev);
            dev
        });

    let mut usb_cfg = tokio_serial::new("COM8", 115200)
        .baud_rate(115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .timeout(Duration::from_millis(1000));

    info!("Usb made");

// let msg = Message::Action(Action::Command(vec![String::from("firefox.exe")]));
// let msg_cbor = msg.serialize().context("Failed to serialize")?;
// info!("Message ready!");

// Ok(loop {
//     if let Ok(mut usb) = usb_cfg.clone().open_native_async() {
//         // wait for USB to be available to write
//         if (usb.writable().await).is_ok() {
//             if let Ok(n) = usb.try_write(&msg_cbor) {
//                 info!("Message sent!");
//             } else {
//                 warn!("Failed to write to serial port");
//             }
//         }

//         let mut buf = [0u8; 1024];
//         // usb.flush();
//         // info!("usb flushed, waiting for message!");

//         sleep(Duration::from_millis(1000)).await;

            // wait for the USB to be available to read
            if (usb.readable().await).is_ok() {
                if let Ok(n) = usb.try_read(&mut buf) {
                    info!("Message received: {:?}", Message::deserialize(&buf));
                } else {
                    warn!("Failed to read from serial port");
                }
            }
        } else {
            warn!("Failed to open serial port");
        }
        sleep(Duration::from_millis(1000)).await;
    })

    // if cfg!(target_os = "windows") {
    // TODO: receive slot messages from controller, instead of hardcoding here
    //
    // _audio_control().await;
    //
    // let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["spotify.exe".to_string()]));
    // let out = msg.serialize().context("Failed to serialize")?;
    // let out = Message::deserialize(&out).context("Failed to deserialize")?;
    // info!("{:?}", out);
    // out.execute_entry()
    //     .await
    // .context("Failed to launch application")?;
    //
    // let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["firefox.exe".to_string()]));
    // let out = msg.serialize().context("Failed to serialize")?;
    // let out = Message::deserialize(&out).context("Failed to deserialize")?;
    // println!("{:?}", out);
    //
    // out.execute_entry()
    //     .await
    //     .context("Failed to launch application")?;
    //
    // let thread0 = tokio::spawn(process(Message::));
    // let thread1 = tokio::spawn(process(Message::));
    // let thread2 = tokio::spawn(process(Message::));
    // let thread3 = tokio::spawn(process(Message::));
    // let thread4 = tokio::spawn(process(Message::));
    // let thread5 = tokio::spawn(process(Message::));
    // let _ = join!(thread0, thread1, thread2, thread3, thread4, thread5);
    //
    // let x = tokio::spawn(keypress(Button::Slot3));
    // let thread6c = tokio::spawn(handle_message(Message::AudioLevels());
    //
    // let launch_discord_fut = tokio::spawn(handle_message(Message::App(App::Slot0)));
    // let launch_spotify_fut = tokio::spawn(handle_message(Message::App(App::Slot1)));
    // let launch_firefox_fut = tokio::spawn(handle_message(Message::App(App::Slot2)));
    // } else {
    // todo!("Linux implementation here")
    // }
}

pub async fn reconnect(mut usb: &mut SerialStream, usb_cfg: &dyn SerialPortBuilderExt) {}
