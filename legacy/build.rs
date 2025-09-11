use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let cube_path = Path::new("thirdparty/STM32CubeF1");
    let hal_path = cube_path.join("Drivers/STM32F1xx_HAL_Driver");
    let rtos_path = cube_path.join("Middlewares/Third_Party/FreeRTOS");

    let includes = [
        Path::new("blinky/include"),
        &rtos_path.join("Source/portable/GCC/ARM_CM3"),
        &rtos_path.join("Source/CMSIS_RTOS"),
        &rtos_path.join("Source/include"),
        &cube_path.join("Drivers/CMSIS/Device/ST/STM32F1xx/Include"),
        &cube_path.join("Drivers/CMSIS/Include"),
        &cube_path.join("Drivers/CMSIS/RTOS/Template"),
        &hal_path.join("Inc"),
    ];

    cc::Build::new()
        .define("STM32F103xB", None)
        .define("USE_HAL_LIBRARY", None)
        .includes(includes)
        .files(
            ["system_stm32f1xx.c", "gcc/startup_stm32f103xb.s"]
                .into_iter()
                .map(|path| {
                    cube_path
                        .join("Drivers/CMSIS/Device/ST/STM32F1xx/Source/Templates/")
                        .join(path)
                }),
        )
        .files(
            hal_path
                .join("Src")
                .read_dir()
                .unwrap()
                .filter_map(|entry| {
                    let path = entry.unwrap().path();

                    if path.extension().is_some_and(|ext| ext == "c")
                        && path
                            .file_stem()
                            .is_some_and(|stem| !stem.to_str().unwrap().ends_with("_template"))
                    {
                        Some(path)
                    } else {
                        None
                    }
                }),
        )
        .files(
            rtos_path
                .join("Source")
                .read_dir()
                .unwrap()
                .filter_map(|entry| {
                    let path = entry.unwrap().path();

                    if path.extension().is_some_and(|ext| ext == "c") {
                        Some(path)
                    } else {
                        None
                    }
                }),
        )
        .files(
            [
                "CMSIS_RTOS/cmsis_os.c",
                "portable/MemMang/heap_4.c",
                "portable/GCC/ARM_CM3/port.c",
            ]
            .into_iter()
            .map(|path| rtos_path.join("Source").join(path)),
        )
        .file(PathBuf::from(env::var("DEP_FREERTOS_SHIM").unwrap()).join("shim.c"))
        .compile("legacy");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .use_core()
        .clang_macro_fallback()
        .clang_args(
            includes
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .clang_arg("-DSTM32F103xB")
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();
}
