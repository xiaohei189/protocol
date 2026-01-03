use prost_build::Config;

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

    // 配置 prost-build 以启用 serde 支持
    let mut config = prost_build::Config::new();

    // 为消息类型添加 serde 注解

    let messages = vec![
        "openim.sdkws.FriendInfo",
        "openim.sdkws.UserInfo",
        "openim.sdkws.MsgData",
        "openim.sdkws.OfflinePushInfo",
    ];
    for message in messages {
        config.type_attribute(message, "#[derive(serde::Serialize, serde::Deserialize)]");
        config.type_attribute(message, "#[serde(rename_all = \"camelCase\")]");
    }

    // 为 FriendInfo.ownerUserID 字段添加 serde 注解
    config.field_attribute(
        "openim.sdkws.FriendInfo.ownerUserID",
        "#[serde(rename = \"ownerUserID\")]",
    );
    config.field_attribute(
        "openim.sdkws.FriendInfo.operatorUserID",
        "#[serde(rename = \"operatorUserID\")]",
    );
    config.field_attribute(
        "openim.sdkws.UserInfo.userID",
        "#[serde(rename = \"userID\")]",
    );

    // 使用 prost-build 编译所有 proto 文件
    config
        .compile_protos(&proto_files, &["."])
        .expect("Failed to compile protos");

    // 设置重新编译触发条件
    for module in &proto_modules {
        println!("cargo:rerun-if-changed={}/{}.proto", module, module);
    }
}
