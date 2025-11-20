// Copyright © 2023 OpenIM. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// rust 语言实现 constant.go 文件
use std::collections::HashMap;
    /// ContentType
    /// UserRelated.
    pub const CONTENT_TYPE_BEGIN: i32 = 100;
    pub const TEXT: i32 = 101;
    pub const PICTURE: i32 = 102;
    pub const VOICE: i32 = 103;
    pub const VIDEO: i32 = 104;
    pub const FILE: i32 = 105;
    pub const AT_TEXT: i32 = 106;
    pub const MERGER: i32 = 107;
    pub const CARD: i32 = 108;
    pub const LOCATION: i32 = 109;
    pub const CUSTOM: i32 = 110;
    pub const REVOKE: i32 = 111;
    pub const TYPING: i32 = 113;
    pub const QUOTE: i32 = 114;

    pub const ADVANCED_TEXT: i32 = 117;
    pub const MARKDOWN_TEXT: i32 = 118;
    pub const CUSTOM_NOT_TRIGGER_CONVERSATION: i32 = 119;
    pub const CUSTOM_ONLINE_ONLY: i32 = 120;
    pub const REACTION_MESSAGE_MODIFIER: i32 = 121;
    pub const REACTION_MESSAGE_DELETER: i32 = 122;
    pub const COMMON: i32 = 200;
    pub const GROUP_MSG: i32 = 201;
    pub const SIGNAL_MSG: i32 = 202;
    pub const CUSTOM_NOTIFICATION: i32 = 203;

    /// SysRelated.
    pub const NOTIFICATION_BEGIN: i32 = 1000;

    pub const FRIEND_APPLICATION_APPROVED_NOTIFICATION: i32 = 1201; // add_friend_response
    pub const FRIEND_APPLICATION_REJECTED_NOTIFICATION: i32 = 1202; // add_friend_response
    pub const FRIEND_APPLICATION_NOTIFICATION: i32 = 1203; // add_friend
    pub const FRIEND_ADDED_NOTIFICATION: i32 = 1204;
    pub const FRIEND_DELETED_NOTIFICATION: i32 = 1205; // delete_friend
    pub const FRIEND_REMARK_SET_NOTIFICATION: i32 = 1206; // set_friend_remark?
    pub const BLACK_ADDED_NOTIFICATION: i32 = 1207; // add_black
    pub const BLACK_DELETED_NOTIFICATION: i32 = 1208; // remove_black
    pub const FRIEND_INFO_UPDATED_NOTIFICATION: i32 = 1209;
    pub const FRIENDS_INFO_UPDATE_NOTIFICATION: i32 = 1210; // update friend info

    pub const CONVERSATION_CHANGE_NOTIFICATION: i32 = 1300; // change conversation opt

    pub const USER_NOTIFICATION_BEGIN: i32 = 1301;
    pub const USER_INFO_UPDATED_NOTIFICATION: i32 = 1303; // SetSelfInfoTip = 204
    pub const USER_STATUS_CHANGE_NOTIFICATION: i32 = 1304;
    pub const USER_COMMAND_ADD_NOTIFICATION: i32 = 1305;
    pub const USER_COMMAND_DELETE_NOTIFICATION: i32 = 1306;
    pub const USER_COMMAND_UPDATE_NOTIFICATION: i32 = 1307;

    pub const USER_SUBSCRIBE_ONLINE_STATUS_NOTIFICATION: i32 = 1308;

    pub const USER_NOTIFICATION_END: i32 = 1399;
    pub const OA_NOTIFICATION: i32 = 1400;

    pub const GROUP_NOTIFICATION_BEGIN: i32 = 1500;

    pub const GROUP_CREATED_NOTIFICATION: i32 = 1501;
    pub const GROUP_INFO_SET_NOTIFICATION: i32 = 1502;
    pub const JOIN_GROUP_APPLICATION_NOTIFICATION: i32 = 1503;
    pub const MEMBER_QUIT_NOTIFICATION: i32 = 1504;
    pub const GROUP_APPLICATION_ACCEPTED_NOTIFICATION: i32 = 1505;
    pub const GROUP_APPLICATION_REJECTED_NOTIFICATION: i32 = 1506;
    pub const GROUP_OWNER_TRANSFERRED_NOTIFICATION: i32 = 1507;
    pub const MEMBER_KICKED_NOTIFICATION: i32 = 1508;
    pub const MEMBER_INVITED_NOTIFICATION: i32 = 1509;
    pub const MEMBER_ENTER_NOTIFICATION: i32 = 1510;
    pub const GROUP_DISMISSED_NOTIFICATION: i32 = 1511;
    pub const GROUP_MEMBER_MUTED_NOTIFICATION: i32 = 1512;
    pub const GROUP_MEMBER_CANCEL_MUTED_NOTIFICATION: i32 = 1513;
    pub const GROUP_MUTED_NOTIFICATION: i32 = 1514;
    pub const GROUP_CANCEL_MUTED_NOTIFICATION: i32 = 1515;
    pub const GROUP_MEMBER_INFO_SET_NOTIFICATION: i32 = 1516;
    pub const GROUP_MEMBER_SET_TO_ADMIN_NOTIFICATION: i32 = 1517;
    pub const GROUP_MEMBER_SET_TO_ORDINARY_USER_NOTIFICATION: i32 = 1518;
    pub const GROUP_INFO_SET_ANNOUNCEMENT_NOTIFICATION: i32 = 1519;
    pub const GROUP_INFO_SET_NAME_NOTIFICATION: i32 = 1520;

    // SignalingNotificationBegin = 1600
    // SignalingNotification = 1601
    // SignalingNotificationEnd = 1649

    pub const SUPER_GROUP_NOTIFICATION_BEGIN: i32 = 1650;
    pub const SUPER_GROUP_UPDATE_NOTIFICATION: i32 = 1651;
    pub const MSG_DELETE_NOTIFICATION: i32 = 1652;
    pub const SUPER_GROUP_NOTIFICATION_END: i32 = 1699;

    pub const CONVERSATION_PRIVATE_CHAT_NOTIFICATION: i32 = 1701;
    pub const CONVERSATION_UNREAD_NOTIFICATION: i32 = 1702;
    pub const CLEAR_CONVERSATION_NOTIFICATION: i32 = 1703;
    pub const CONVERSATION_DELETE_NOTIFICATION: i32 = 1704;

    pub const BUSINESS_NOTIFICATION_BEGIN: i32 = 2000;
    pub const BUSINESS_NOTIFICATION: i32 = 2001;
    pub const BUSINESS_NOTIFICATION_END: i32 = 2099;

    pub const MSG_REVOKE_NOTIFICATION: i32 = 2101;
    pub const DELETE_MSGS_NOTIFICATION: i32 = 2102;

    pub const HAS_READ_RECEIPT: i32 = 2200;

    pub const NOTIFICATION_END: i32 = 5000;

    /// status.
    pub const MSG_NORMAL: i32 = 1;
    pub const MSG_DELETED: i32 = 4;

    /// MsgFrom.
    pub const USER_MSG_TYPE: i32 = 100;
    pub const SYS_MSG_TYPE: i32 = 200;

    /// SessionType.
    pub const SINGLE_CHAT_TYPE: i32 = 1;
    /// WriteGroupChatType Not enabled temporarily
    pub const WRITE_GROUP_CHAT_TYPE: i32 = 2;
    pub const READ_GROUP_CHAT_TYPE: i32 = 3;
    pub const NOTIFICATION_CHAT_TYPE: i32 = 4;
    /// token.
    pub const NORMAL_TOKEN: i32 = 0;
    pub const INVALID_TOKEN: i32 = 1;
    pub const KICKED_TOKEN: i32 = 2;
    pub const EXPIRED_TOKEN: i32 = 3;

    /// MultiTerminalLogin.
    pub const DEFAULT_NOT_KICK: i32 = 0;
    /// Full-end login, but the same end is mutually exclusive.
    pub const ALL_LOGIN_BUT_SAME_TERM_KICK: i32 = 1;
    /// The PC side is mutually exclusive, and the mobile side is mutually exclusive, but the web side can be online at
    /// the same time.
    pub const ALL_LOGIN_BUT_SAME_CLASS_KICK: i32 = 4;
    /// The PC terminal can be online at the same time,but other terminal only one of the endpoints can login.
    pub const PC_AND_OTHER: i32 = 5;

    // OnlineStatus = "online"
    // OfflineStatus = "offline"
    // Registered = "registered"
    // UnRegistered = "unregistered"

    pub const ONLINE: i32 = 1;
    pub const OFFLINE: i32 = 0;

    pub const REGISTERED: i32 = 1;
    pub const UNREGISTERED: i32 = 0;

    /// MsgReceiveOpt.
    pub const RECEIVE_MESSAGE: i32 = 0;
    pub const NOT_RECEIVE_MESSAGE: i32 = 1;
    pub const RECEIVE_NOT_NOTIFY_MESSAGE: i32 = 2;

    /// OptionsKey.
    pub const IS_HISTORY: &str = "history";
    pub const IS_PERSISTENT: &str = "persistent";
    pub const IS_OFFLINE_PUSH: &str = "offlinePush";
    pub const IS_UNREAD_COUNT: &str = "unreadCount";
    pub const IS_CONVERSATION_UPDATE: &str = "conversationUpdate";
    pub const IS_SENDER_SYNC: &str = "senderSync";
    pub const IS_NOT_PRIVATE: &str = "notPrivate";
    pub const IS_SENDER_CONVERSATION_UPDATE: &str = "senderConversationUpdate";
    pub const IS_SENDER_NOTIFICATION_PUSH: &str = "senderNotificationPush";
    pub const IS_REACTION_FROM_CACHE: &str = "reactionFromCache";
    pub const IS_NOT_NOTIFICATION: &str = "isNotNotification";
    pub const IS_SEND_MSG: &str = "isSendMsg";

    /// GroupStatus.
    pub const GROUP_OK: i32 = 0;
    pub const GROUP_BAN_CHAT: i32 = 1;
    pub const GROUP_STATUS_DISMISSED: i32 = 2;
    pub const GROUP_STATUS_MUTED: i32 = 3;

    /// GroupType.
    pub const NORMAL_GROUP: i32 = 0;
    pub const SUPER_GROUP: i32 = 1;
    pub const WORKING_GROUP: i32 = 2;

    pub const GROUP_BANED: i32 = 3;
    pub const GROUP_BAN_PRIVATE_CHAT: i32 = 4;

    /// UserJoinGroupSource.
    pub const JOIN_BY_ADMIN: i32 = 1;

    pub const JOIN_BY_INVITATION: i32 = 2;
    pub const JOIN_BY_SEARCH: i32 = 3;
    pub const JOIN_BY_QR_CODE: i32 = 4;

    /// Minio.
    pub const MINIO_DURATION_TIMES: i32 = 3600;
    /// Aws.
    pub const AWS_DURATION_TIMES: i32 = 3600;

    /// callbackCommand.
    pub const CALLBACK_BEFORE_SEND_SINGLE_MSG_COMMAND: &str = "callbackBeforeSendSingleMsgCommand";
    pub const CALLBACK_AFTER_SEND_SINGLE_MSG_COMMAND: &str = "callbackAfterSendSingleMsgCommand";
    pub const CALLBACK_BEFORE_SEND_GROUP_MSG_COMMAND: &str = "callbackBeforeSendGroupMsgCommand";
    pub const CALLBACK_AFTER_SEND_GROUP_MSG_COMMAND: &str = "callbackAfterSendGroupMsgCommand";
    pub const CALLBACK_MSG_MODIFY_COMMAND: &str = "callbackMsgModifyCommand";
    pub const CALLBACK_USER_ONLINE_COMMAND: &str = "callbackUserOnlineCommand";
    pub const CALLBACK_USER_OFFLINE_COMMAND: &str = "callbackUserOfflineCommand";
    pub const CALLBACK_USER_KICK_OFF_COMMAND: &str = "callbackUserKickOffCommand";
    pub const CALLBACK_OFFLINE_PUSH_COMMAND: &str = "callbackOfflinePushCommand";
    pub const CALLBACK_ONLINE_PUSH_COMMAND: &str = "callbackOnlinePushCommand";
    pub const CALLBACK_SUPER_GROUP_ONLINE_PUSH_COMMAND: &str = "callbackSuperGroupOnlinePushCommand";
    pub const CALLBACK_BEFORE_ADD_FRIEND_COMMAND: &str = "callbackBeforeAddFriendCommand";
    pub const CALLBACK_BEFORE_UPDATE_USER_INFO_COMMAND: &str = "callbackBeforeUpdateUserInfoCommand";
    pub const CALLBACK_BEFORE_CREATE_GROUP_COMMAND: &str = "callbackBeforeCreateGroupCommand";
    pub const CALLBACK_BEFORE_MEMBER_JOIN_GROUP_COMMAND: &str = "callbackBeforeMemberJoinGroupCommand";
    pub const CALLBACK_BEFORE_SET_GROUP_MEMBER_INFO_COMMAND: &str = "CallbackBeforeSetGroupMemberInfoCommand";
    pub const CALLBACK_BEFORE_SET_MESSAGE_REACTION_EXTENSION_COMMAND: &str = "callbackBeforeSetMessageReactionExtensionCommand";
    pub const CALLBACK_BEFORE_DELETE_MESSAGE_REACTION_EXTENSIONS_COMMAND: &str = "callbackBeforeDeleteMessageReactionExtensionsCommand";
    pub const CALLBACK_GET_MESSAGE_LIST_REACTION_EXTENSIONS_COMMAND: &str = "callbackGetMessageListReactionExtensionsCommand";
    pub const CALLBACK_ADD_MESSAGE_LIST_REACTION_EXTENSIONS_COMMAND: &str = "callbackAddMessageListReactionExtensionsCommand";

    /// callback actionCode.
    pub const ACTION_ALLOW: i32 = 0;
    pub const ACTION_FORBIDDEN: i32 = 1;
    /// callback callbackHandleCode.
    pub const CALLBACK_HANDLE_SUCCESS: i32 = 0;
    pub const CALLBACK_HANDLE_FAILED: i32 = 1;

    /// minioUpload.
    pub const OTHER_TYPE: i32 = 1;
    pub const VIDEO_TYPE: i32 = 2;
    pub const IMAGE_TYPE: i32 = 3;

    /// sendMsgStatus.
    pub const MSG_STATUS_NOT_EXIST: i32 = 0;
    pub const MSG_IS_SENDING: i32 = 1;
    pub const MSG_SEND_SUCCESSED: i32 = 2;
    pub const MSG_SEND_FAILED: i32 = 3;

    pub const WRITE_DIFFUSION: i32 = 0;
    pub const READ_DIFFUSION: i32 = 1;

    pub const UNRELIABLE_NOTIFICATION: i32 = 1;
    pub const RELIABLE_NOTIFICATION_NO_MSG: i32 = 2;
    pub const RELIABLE_NOTIFICATION_MSG: i32 = 3;

    pub const AT_ALL_STRING: &str = "AtAllTag";
    pub const AT_NORMAL: i32 = 0;
    pub const AT_ME: i32 = 1;
    pub const AT_ALL: i32 = 2;
    pub const AT_ALL_AT_ME: i32 = 3;
    pub const GROUP_NOTIFICATION: i32 = 4;

    lazy_static::lazy_static! {
        pub static ref CONTENT_TYPE_2_PUSH_CONTENT: HashMap<i32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(PICTURE, "[PICTURE]");
            m.insert(VOICE, "[VOICE]");
            m.insert(VIDEO, "[VIDEO]");
            m.insert(FILE, "[File]");
            m.insert(TEXT, "[TEXT]");
            m.insert(AT_TEXT, "[@TEXT]");
            m.insert(GROUP_MSG, "[GROUPMSG]]");
            m.insert(COMMON, "[NEWMSG]");
            m.insert(SIGNAL_MSG, "[SIGNALINVITE]");
            m
        };
    }

    pub const FIELD_RECV_MSG_OPT: i32 = 1;
    pub const FIELD_IS_PINNED: i32 = 2;
    pub const FIELD_ATTACHED_INFO: i32 = 3;
    pub const FIELD_IS_PRIVATE_CHAT: i32 = 4;
    pub const FIELD_GROUP_AT_TYPE: i32 = 5;
    pub const FIELD_EX: i32 = 7;
    pub const FIELD_UNREAD: i32 = 8;
    pub const FIELD_BURN_DURATION: i32 = 9;
    pub const FIELD_HAS_READ_SEQ: i32 = 10;

    pub const IM_ORDINARY_USER: i32 = 0;
    pub const APP_ORDINARY_USERS: i32 = 1;
    pub const APP_ADMIN: i32 = 2;
    pub const APP_NOTIFICATION_ADMIN: i32 = 3;
    pub const APP_ROBOT_ADMIN: i32 = 4;

    pub const GROUP_OWNER: i32 = 100;
    pub const GROUP_ADMIN: i32 = 60;
    pub const GROUP_ORDINARY_USERS: i32 = 20;

    pub const GROUP_RESPONSE_AGREE: i32 = 1;
    pub const GROUP_RESPONSE_REFUSE: i32 = -1;

    pub const FRIEND_RESPONSE_NOT_HANDLE: i32 = 0;
    pub const FRIEND_RESPONSE_AGREE: i32 = 1;
    pub const FRIEND_RESPONSE_REFUSE: i32 = -1;

    pub const MALE: i32 = 1;
    pub const FEMALE: i32 = 2;

    pub const OPERATION_ID: &str = "operationID";
    pub const OP_USER_ID: &str = "opUserID";
    pub const CONN_ID: &str = "connID";
    pub const OP_USER_PLATFORM: &str = "platform";
    pub const TOKEN: &str = "token";
    pub const RPC_CUSTOM_HEADER: &str = "customHeader"; // rpc中间件自定义ctx参数
    pub const CHECK_KEY: &str = "CheckKey";
    pub const TRIGGER_ID: &str = "triggerID";
    pub const REMOTE_ADDR: &str = "remoteAddr";

    pub const BECOME_FRIEND_BY_IMPORT: i32 = 1; // 管理员导入
    pub const BECOME_FRIEND_BY_APPLY: i32 = 2; // 申请添加

    pub const APPLY_NEED_VERIFICATION_INVITE_DIRECTLY: i32 = 0; // 申请需要同意 邀请直接进
    pub const ALL_NEED_VERIFICATION: i32 = 1; // 所有人进群需要验证，除了群主管理员邀请进群
    pub const DIRECTLY: i32 = 2; // 直接进群

    pub const GROUP_RPC_RECV_SIZE: i32 = 30;
    pub const GROUP_RPC_SEND_SIZE: i32 = 30;

    pub const FRIEND_ACCEPT_TIP: &str = "You have successfully become friends, so start chatting";

    pub fn group_is_ban_chat(status: i32) -> bool {
        status == GROUP_STATUS_MUTED
    }

    pub fn group_is_ban_private_chat(status: i32) -> bool {
        status == GROUP_BAN_PRIVATE_CHAT
    }

    pub const LOG_FILE_NAME: &str = "OpenIM.log";

    pub const LOCAL_HOST: &str = "0.0.0.0";

    /// flag parse.
    pub const FLAG_PORT: &str = "port";
    pub const FLAG_WS_PORT: &str = "ws_port";
    pub const FLAG_TRANSFER_PROGRESS_INDEX: &str = "transferProgressIndex";
    pub const FLAG_PROMETHEUS_PORT: &str = "prometheus_port";
    pub const FLAG_CONF: &str = "config_folder_path";

    pub const OPEN_IM_COMMON_CONFIG_KEY: &str = "OpenIMServerConfig";

    pub const CALLBACK_COMMAND: &str = "command";

    pub const BATCH_NUM: i32 = 100;

    /// Subscribe to user constants
    pub const SUBSCRIBER_USER: i32 = 1;
    pub const UNSUBSCRIBE: i32 = 2;

    pub const GROUP_SEARCH_POSITION_HEAD: i32 = 1;
    pub const GROUP_SEARCH_POSITION_ANY: i32 = 2;

    pub const FIRST_PAGE_NUMBER: i32 = 1;
    pub const MAX_SYNC_PULL_NUMBER: i32 = 500;

    pub const MSG_STATUS_SENDING: i32 = 1;
    pub const MSG_STATUS_SEND_SUCCESS: i32 = 2;
    pub const MSG_STATUS_SEND_FAILED: i32 = 3;
    pub const MSG_STATUS_HAS_DELETED: i32 = 4;
    pub const MSG_STATUS_FILTERED: i32 = 5;

