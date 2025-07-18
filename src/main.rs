use std::{
    fs::{File, remove_file},
    io::Write,
    path::Path,
    time::Instant,
};

use mock_rust::{
    mock::{address, boolean, cname, color, csentence, image, integer, ip4},
    model::{Room, User},
};

fn main() {
    let path_url = Path::new("user.log");
    if Path::exists(path_url) {
        remove_file(path_url).unwrap();
    }
    let mut user_log = File::create(path_url).unwrap();
    let start = Instant::now();
    let imgs: Vec<User> = (0..5000000)
        .map(|_| {
            let user = User {
                id: integer(1000000..9999999),
                address: address(),
                ip: ip4(),
                sex: boolean(1, 2),
                avatar: image(
                    (90, 90),
                    color().as_str(),
                    color().as_str(),
                    Some("jpeg"),
                    Some(csentence(1..3).as_str()),
                ),
                name: cname(),
                room: if boolean(3, 10) {
                    Some(Room {
                        id: integer(100000..999999),
                        room_img_url: image(
                            (120, 90),
                            color().as_str(),
                            color().as_str(),
                            Some("png"),
                            Some(csentence(1..3).as_str()),
                        ),
                        is_live: boolean(1, 5),
                        title: csentence(5..12),
                        hot: integer(1000..9999),
                        tags: (0..integer(0..3))
                            .map(|_| {
                                let num = integer((0..5).into());
                                num.into()
                            })
                            .collect(),
                    })
                } else {
                    None
                },
            };
            user_log.write(format!("{:#?}\n", user).as_bytes()).unwrap();
            user
        })
        .collect();
    println!(
        "generate {} users elapsed: {}",
        imgs.len(),
        start.elapsed().as_secs_f32()
    );
}
