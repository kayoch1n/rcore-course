use std::fs::{read_dir, File};
use std::io::{Result, Write};

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release";

// https://doc.rust-lang.org/cargo/reference/build-scripts.html

fn main() {
    println!("cargo:rerun-if-changed=../user/src");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);

    insert_app_data().unwrap();
}

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.asm")?;
    let mut apps: Vec<_> = read_dir("../user/src/bin")?
        .map(|dir| {
            let mut name = dir.unwrap().file_name().into_string().unwrap();
            name.drain(name.find('.').unwrap()..name.len());
            name
        })
        .collect();
    apps.sort();

    // 把数量先放好好
    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global __num_app
    .global __end_app
__num_app:
    .quad {}"#,
        apps.len()
    )?;

    for (idx, _) in apps.iter().enumerate() {
        writeln!(f, r#".quad app_{}_start"#, idx)?;
    }

    writeln!(
        f,
        r#"
    .section .data"#
    )?;

    for (idx, app) in apps.iter().enumerate() {
        println!("writing app[{}] - {}", idx, app);
        writeln!(
            f,
            r#"
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{1}/{2}.bin"
app_{0}_end:"#,
            idx, TARGET_PATH, app
        )?;
    }

    writeln!(
        f,
        r#"
__end_app:
    .quad 0
    "#
    )?;

    Ok(())
}
