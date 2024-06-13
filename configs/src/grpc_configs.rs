use std::sync::OnceLock;

use dependencies_sync::{
    chrono::Duration,
    rust_i18n::{self, t},
};
use serde_derive::{Deserialize, Serialize};

use crate::{get_config, ConfigTrait};

const GRPCCONFIGS_NAME: &str = "grpc";

static GRPCCONFIGS: OnceLock<GrpcConfigs> = OnceLock::new();

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GrpcConfigs {
    pub concurrency_limit_per_connection: usize,
    pub http2_adaptive_window: Option<bool>,
    pub http2_keepalive_interval: Option<Duration>,
    pub http2_keepalive_timeout: Option<Duration>,
    pub http2_max_pending_accept_reset_streams: Option<usize>,
    pub initial_connection_window_size: Option<u32>,
    pub initial_stream_window_size: Option<u32>,
    pub max_concurrent_streams: Option<u32>,
    pub max_frame_size: Option<u32>,
    pub tcp_keepalive: Option<Duration>,
    pub tcp_nodelay: bool,
    pub timeout: Duration,
}

impl ConfigTrait for GrpcConfigs {
    fn name() -> &'static str {
        GRPCCONFIGS_NAME
    }

    fn get() -> &'static Self {
        if let Some(configs) = GRPCCONFIGS.get() {
            return configs;
        } else {
            let configs = get_config::<GrpcConfigs>().expect(t!("取得配置失败").as_str());
            GRPCCONFIGS.set(configs).expect("设置配置失败");
        }

        GRPCCONFIGS.get().unwrap()
    }
}

impl Default for GrpcConfigs {
    fn default() -> Self {
        GrpcConfigs {
            concurrency_limit_per_connection: 256,
            http2_adaptive_window: Some(false),
            http2_keepalive_interval: None,
            http2_keepalive_timeout: Some(Duration::seconds(10)),
            http2_max_pending_accept_reset_streams: None,
            initial_connection_window_size: Some(65535),
            initial_stream_window_size: Some(65535),
            max_concurrent_streams: None,
            max_frame_size: None,
            tcp_keepalive: None,
            tcp_nodelay: true,
            timeout: Duration::seconds(60),
        }
    }
}
