use raylib::prelude::*;

pub fn setup() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size(640, 480)
        .title("Juanjito's adventure")
        .build()
}
