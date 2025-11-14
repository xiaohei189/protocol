fn main() {
    // 定义所有需要编译的 proto 模块
    let proto_modules = vec![
        "auth",
        "conversation",
        "errinfo",
        "group",
        "jssdk",
        "msg",
        "msggateway",
        "push",
        "relation",
        "rtc",
        "sdkws",
        "third",
        "user",
        "wrapperspb",
    ];

    let mut proto_files = Vec::new();
    for module in &proto_modules {
        proto_files.push(format!("{}/{}.proto", module, module));
    }

    // 使用 prost-build 编译所有 proto 文件
    prost_build::compile_protos(&proto_files, &["."]).expect("Failed to compile protos");

    // 设置重新编译触发条件
    for module in proto_modules {
        println!("cargo:rerun-if-changed={}/{}.proto", module, module);
    }
}

