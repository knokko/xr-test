use openxr as xr;
use openxr::{ExtensionSet, ApplicationInfo, Version};

fn main() {
    let xr_entry = xr::Entry::load().expect("Should be able to load OpenXR");

    let app_info = ApplicationInfo {
        application_name: "xr tester",
        application_version: 1,
        engine_name: "",
        engine_version: 0
    };

    let mut extension_set = ExtensionSet::default();
    extension_set.khr_opengl_enable = false;

    println!("Available extensions are {:?}", xr_entry.enumerate_extensions());

    println!("Create instance...");
    let xr_instance = xr_entry.create_instance(&app_info, &extension_set, &[]).expect("Should be able to create instance");
    println!("Created instance");
    println!("properties are {:?}", xr_instance.properties());
}
