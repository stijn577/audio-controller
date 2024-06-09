fn main() {
    // std::fs::write(
    //     "./controller-buttons.conf",
    //     serde_cbor::to_vec(&vec![
    //         // discord update and run like the normal binding does
    //         vec![
    //             "C",
    //             "C:/Users/Stijn_Admin/AppData/Local/Discord/Update.exe",
    //             "--processStart",
    //             "Discord.exe",
    //         ],
    //         // firefox program
    //         vec!["C", "firefox.exe"],
    //         // spotify.exe
    //         vec!["C", "spotify.exe"],
    //         // Keycommand for volume up
    //         vec!["K", (0x81 as char).to_string().as_ref()],
    //     ])
    //     .unwrap(),
    // )
    // .unwrap();

    println!("{:?}", include_bytes!("../../../images/discord.bmp").len());

    // let x = std::fs::read("../../images/discord.bmp").unwrap();
    // println!("{x:?}");
}
