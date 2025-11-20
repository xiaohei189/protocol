// OpenIM Protocol - Rust bindings for Protocol Buffers
//
// 所有模块通过 build.rs 在编译时自动生成

pub mod auth {
    include!(concat!(env!("OUT_DIR"), "/openim.auth.rs"));
}

pub mod conversation {
    include!(concat!(env!("OUT_DIR"), "/openim.conversation.rs"));
}

pub mod errinfo {
    include!(concat!(env!("OUT_DIR"), "/openim.protobuf.rs"));
}

pub mod group {
    include!(concat!(env!("OUT_DIR"), "/openim.group.rs"));
}

pub mod jssdk {
    include!(concat!(env!("OUT_DIR"), "/openim.jssdk.rs"));
}

pub mod msg {
    include!(concat!(env!("OUT_DIR"), "/openim.msg.rs"));
}

pub mod msggateway {
    include!(concat!(env!("OUT_DIR"), "/openim.msggateway.rs"));
}

pub mod push {
    include!(concat!(env!("OUT_DIR"), "/openim.push.rs"));
}

pub mod relation {
    include!(concat!(env!("OUT_DIR"), "/openim.relation.rs"));
}

pub mod rtc {
    include!(concat!(env!("OUT_DIR"), "/openim.rtc.rs"));
}

pub mod sdkws {
    include!(concat!(env!("OUT_DIR"), "/openim.sdkws.rs"));
}

pub mod third {
    include!(concat!(env!("OUT_DIR"), "/openim.third.rs"));
}

pub mod user {
    include!(concat!(env!("OUT_DIR"), "/openim.user.rs"));
}

pub mod constant {
    include!("../constant/constant.rs");
}



// 导出 protobuf 基础类型（wrapperspb）
pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/openim.protobuf.rs"));
}

// wrapperspb 别名指向 protobuf
pub use protobuf as wrapperspb;

// 重新导出 prost，方便使用
pub use prost;
pub use prost::Message;

