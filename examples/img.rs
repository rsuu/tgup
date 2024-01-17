use std::thread::sleep_ms;
use tgui::{View, ViewSet, *};

fn main() -> Res<()> {
    let tgui = Tgui::new()?.conn()?;
    let act = Activity::new().conn(&tgui)?;

    let data = act.gen_create().unwrap().set_parent(-1);
    let img_view = Img::new().set_data(data).conn(&tgui)?;

    let ity = ImgTy::open_jpg("test.jpg").unwrap();
    let view = act.gen_view(img_view.res());
    img_view.update(&tgui, &ity, view)?;

    dbg!(img_view);

    sleep_ms(10000);

    Ok(())
}
