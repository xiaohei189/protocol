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

    // 配置 prost-build，添加 serde 支持
    let mut config = Config::new();

    // 注：prost::Message derive 宏内部已自动实现 Debug，无需额外添加

    // 为所有类型添加 serde 支持，使用 camelCase 作为默认命名规则
    let messages = vec![
        "openim.sdkws.FriendInfo",
        "openim.sdkws.UserInfo",
        "openim.sdkws.PublicUserInfo",
        "openim.sdkws.FriendRequest",
        "openim.sdkws.BlackInfo",
        "openim.sdkws.MsgData",
        "openim.sdkws.OfflinePushInfo",
        "openim.sdkws.PushMessages",
        "openim.sdkws.PullMsgs",
    ];
    for message in messages {
        config.type_attribute(message, "#[derive(serde::Serialize, serde::Deserialize)]");
        config.type_attribute(message, "#[serde(rename_all = \"camelCase\")]");
    }

    // 为 bridge 类型添加 flutter_rust_bridge non_opaque，使 FRB 生成非 opaque 的 Dart 类型
    config.type_attribute(
        "openim.sdkws.MsgData",
        "#[flutter_rust_bridge::frb(non_opaque)]",
    );
    config.type_attribute(
        "openim.sdkws.OfflinePushInfo",
        "#[flutter_rust_bridge::frb(non_opaque)]",
    );

    // 为 sdkws 模块的特殊字段配置 JSON 字段名映射（覆盖 camelCase 规则）
    // 这些字段需要 ID/URL 等全大写，而不是 camelCase

    // FriendInfo 字段
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
    config.field_attribute(
        "openim.sdkws.UserInfo.faceURL",
        "#[serde(rename = \"faceURL\")]",
    );
    config.field_attribute(
        "openim.sdkws.UserInfo.app_manger_level",
        "#[serde(rename = \"appMangerLevel\")]",
    );
    config.field_attribute(
        "openim.sdkws.UserInfo.global_recv_msg_opt",
        "#[serde(rename = \"globalRecvMsgOpt\")]",
    );

    // PublicUserInfo 字段
    config.field_attribute(
        "openim.sdkws.PublicUserInfo.user_id",
        "#[serde(rename = \"userID\")]",
    );
    config.field_attribute(
        "openim.sdkws.PublicUserInfo.face_url",
        "#[serde(rename = \"faceURL\")]",
    );

    // FriendRequest 字段
    config.field_attribute(
        "openim.sdkws.FriendRequest.from_user_id",
        "#[serde(rename = \"fromUserID\")]",
    );
    config.field_attribute(
        "openim.sdkws.FriendRequest.from_face_url",
        "#[serde(rename = \"fromFaceURL\")]",
    );
    config.field_attribute(
        "openim.sdkws.FriendRequest.to_user_id",
        "#[serde(rename = \"toUserID\")]",
    );
    config.field_attribute(
        "openim.sdkws.FriendRequest.to_face_url",
        "#[serde(rename = \"toFaceURL\")]",
    );
    config.field_attribute(
        "openim.sdkws.FriendRequest.handler_user_id",
        "#[serde(rename = \"handlerUserID\")]",
    );

    // BlackInfo 字段
    config.field_attribute(
        "openim.sdkws.BlackInfo.owner_user_id",
        "#[serde(rename = \"ownerUserID\")]",
    );
    config.field_attribute(
        "openim.sdkws.BlackInfo.operator_user_id",
        "#[serde(rename = \"operatorUserID\")]",
    );

    // 编译 proto 文件
    config
        .compile_protos(&proto_files, &["."])
        .expect("Failed to compile protos");

    // 设置重新编译触发条件
    for module in &proto_modules {
        println!("cargo:rerun-if-changed={}/{}.proto", module, module);
    }
}
