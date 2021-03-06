fn main() {
    tonic_build::configure()
    .build_client(true)
    .build_server(false)
    .out_dir("src/protos")
    .include_file("mod.rs")
    .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
    //.client_attribute("ConfigController", "#[derive(PartialEq)]")
    .compile(
        &["../../../../src/github.com/michaelhenkel/config-controller/pkg/apis/v1/controller.proto"],
        &["../../../../src"],
    ).unwrap();
}