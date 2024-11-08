
mod pandora;
mod steam;
// use pandora::*;
use raylib::prelude::*;
// use steam::*;
// use steamworks as sw;

fn main() {
    let pos = [(1, 100, 100), (1, 200, 200), (0, 300, 300)];
    let (me, _) = steam::steam();
    let (mut rl, thread) = pandora::setup();

    let weight = std::mem::size_of_val(&pos);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for (enabled, x, y) in pos {
            if enabled == 1 {
                d.draw_text(me.name().as_str(), x, y, 20, Color::BLACK);
            }
        }
        d.draw_text(&weight.to_string(), 100, 200, 20, Color::BLACK);
    }
}

// for (i, f) in online_friends.iter().enumerate() {
//     let y_position: i32 = (24 + (18 * i)).try_into().unwrap();
//     d.draw_text(f.as_str(), 12, y_position, 20, Color::BLACK);
// }
// println!("{}", me.name());

// let utils = client.utils();
// println!("Utils:");
// println!("AppId: {:?}", utils.app_id());
// println!("UI Language: {}", utils.ui_language());
//
// let apps = client.apps();
// println!("Apps");
// println!("IsInstalled(480): {}", apps.is_app_installed(AppId(480)));
// println!("InstallDir(480): {}", apps.app_install_dir(AppId(480)));
// println!("BuildId: {}", apps.app_build_id());
// println!("AppOwner: {:?}", apps.app_owner());
// println!("Langs: {:?}", apps.available_game_languages());
// println!("Lang: {}", apps.current_game_language());
// println!("Beta: {:?}", apps.current_beta_name());
//
// let friends = client.friends();
// println!("Friends");
// if f.id().raw() == 76561199190409892 {
//     f.invite_user_to_game("");
// }
// for _ in 0..50 {
//     single.run_callbacks();
//     ::std::thread::sleep(::std::time::Duration::from_millis(100));
// }
//
