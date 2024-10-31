#![windows_subsystem = "windows"]

use winsafe::{co, gui, prelude::*};

fn main() {
    window_main().run_main(None).unwrap();
}

fn window_main() -> gui::WindowMain {
    let window_options = gui::WindowMainOpts {
        title: "注册机".to_string(),
        size: (430, 120),
        ..Default::default()
    };

    let window = gui::WindowMain::new(window_options);

    let _label_input = gui::Label::new(
        &window,
        gui::LabelOpts {
            text: "机器码".to_string(),
            position: (10, 10),
            ..Default::default()
        },
    );

    let _label_key = gui::Label::new(
        &window,
        gui::LabelOpts {
            text: "键".to_string(),
            position: (23, 40),
            ..Default::default()
        },
    );

    let _label_output = gui::Label::new(
        &window,
        gui::LabelOpts {
            text: "注册码".to_string(),
            position: (10, 75),
            ..Default::default()
        },
    );

    let edit_input = gui::Edit::new(
        &window,
        gui::EditOpts {
            position: (90, 10),
            width: 260,
            height: 20,
            edit_style: co::ES::CENTER,
            ..Default::default()
        },
    );

    let edit_key = gui::Edit::new(
        &window,
        gui::EditOpts {
            position: (90, 40),
            width: 260,
            height: 20,
            edit_style: co::ES::CENTER,
            ..Default::default()
        },
    );

    let edit_output = gui::Edit::new(
        &window,
        gui::EditOpts {
            position: (90, 75),
            width: 260,
            height: 20,
            edit_style: co::ES::CENTER | co::ES::READONLY,

            ..Default::default()
        },
    );

    let button_set_resolution = gui::Button::new(
        &window,
        gui::ButtonOpts {
            text: "生成".to_owned(),
            position: (370, 20),
            width: 50,
            height: 50,
            button_style: co::BS::CENTER | co::BS::VCENTER | co::BS::MULTILINE | co::BS::PUSHBUTTON,
            ..Default::default()
        },
    );

    button_set_resolution.on().bn_clicked(move || {
        if edit_input.text().is_empty() {
            return Ok(());
        }

        let machine_code = edit_input.text().trim_end().to_string();

        let key = edit_key.text().trim_end().to_string();

        let license = format!("{:x}", md5::compute(format!("{machine_code}{key}")));

        edit_output.set_text(&license);

        Ok(())
    });

    window
}
