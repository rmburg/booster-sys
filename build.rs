fn main() {
    let mut bridge = cxx_build::bridge("src/lib.rs");

    bridge
        .flag_if_supported("-std=c++17")
        .file("src/cpp/shim/b1_loco_client.cpp")
        .include("src/cpp")
        .include("booster_robotics_sdk/include")
        .include("booster_robotics_sdk/third_party/include")
        .compile("booster_sys");

    // Link your precompiled static library
    println!("cargo::rerun-if-changed=src/lib.rs");
    println!("cargo::rustc-link-search=native=booster_robotics_sdk/lib/x86_64");
    println!("cargo::rustc-link-search=native=booster_robotics_sdk/third_party/lib/x86_64");
    println!("cargo::rustc-link-lib=static=booster_robotics_sdk");
    println!("cargo::rustc-link-lib=static=foonathan_memory-0.7.3");
    println!("cargo::rustc-link-lib=fastcdr");
    println!("cargo::rustc-link-lib=fastrtps");
}
