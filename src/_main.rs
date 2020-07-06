use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
  //SDL構造体の呼び出し
  let sdl_context = sdl2::init().unwrap();
  //ウインドウの作成
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("SDL", 640, 480)
    .position_centered()
    .build()
    .unwrap();

  //キャンバスの生成と塗りつぶし
  let mut canvas = window.into_canvas().build().unwrap();
  //塗りつぶし
  canvas.set_draw_color(Color::RGB(255, 255, 255));
  canvas.clear();
  //レンダリングの結果を反映
  canvas.present();

  // イベントループの作成
  let mut event_pump = sdl_context.event_pump().unwrap();
  'running: loop {
      for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown {
              keycode: Some(Keycode::Escape),
              ..
            } => break 'running,
          _ => {}
        }
      }

    canvas.present();

    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}