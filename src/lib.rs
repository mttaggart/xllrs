use std::process::Command;

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn xlAutoOpen() {
    Command::new("calc.exe")
        .spawn()
        .unwrap();
}
