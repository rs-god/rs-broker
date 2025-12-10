use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct KafkaConfig {
    pub brokers: String,
    pub username: String,
    pub password: String,

    // kafka SASL/SCRAM auth,eg: "SCRAM-SHA-512" or "SCRAM-SHA-256"
    // default:"SCRAM-SHA-256"
    pub sasl_type_scram_sha: String,

    // default:PLAIN
    pub sasl_mechanism: String,

    // kafka security protocol,eg: PLAINTEXT,SASL_SSL,SASL_PLAINTEXT
    pub security_protocol: String,

    // kafka client connection ca_cert path,if security_protocol is SASL_SSL,it must not be empty
    pub cert_path: String,

    // kafka insecure_skip_verify,default:false
    pub insecure_skip_verify: bool,

    // producer publish timeout,default:10s
    // kafka message.timeout.ms
    pub message_timeout: Duration,

    // kafka message.max.bytes,default:1024 * 1024
    pub message_max_bytes: usize,

    // kafka message.send.max.retries,default:3
    pub message_send_max_retries: usize,

    // kafka auto.offset.reset,eg:latest,earliest
    pub auto_offset_reset: String,

    // kafka enable.auto.commit,default:false
    pub enable_auto_commit: bool,

    // kafka client graceful timeout,default:3s
    pub graceful_wait_timeout: Duration,
}

impl KafkaConfig {
    pub fn builder(brokers: &str) -> KafkaConfigBuilder {
        KafkaConfigBuilder::new(brokers)
    }
}

#[derive(Debug, Default)]
pub struct KafkaConfigBuilder {
    brokers: String,
    username: String,
    password: String,

    // kafka SASL/SCRAM auth,eg: "SCRAM-SHA-512" or "SCRAM-SHA-256"
    // default:"SCRAM-SHA-256"
    sasl_type_scram_sha: Option<String>,

    // default:PLAIN
    sasl_mechanism: Option<String>,

    // kafka security protocol,eg: PLAINTEXT,SASL_SSL,SASL_PLAINTEXT
    // default:PLAINTEXT
    security_protocol: Option<String>,

    // kafka client connection ca_cert path,if security_protocol is SASL_SSL,it must not be empty
    cert_path: String,
    // kafka insecure_skip_verify default:false
    insecure_skip_verify: Option<bool>,

    // producer publish timeout,default:10s
    message_timeout: Option<Duration>,

    // kafka message.max.bytes,default:1024 * 1024
    message_max_bytes: Option<usize>,

    // kafka message.send.max.retries,default:3
    message_send_max_retries: Option<usize>,

    // kafka auto.offset.reset,eg:latest,earliest
    // default:latest
    auto_offset_reset: Option<String>,

    // kafka enable.auto.commit,default:false
    enable_auto_commit: Option<bool>,

    // kafka client graceful timeout,default:3s
    graceful_wait_timeout: Option<Duration>,
}

impl KafkaConfigBuilder {
    pub fn new(brokers: &str) -> Self {
        Self {
            brokers: brokers.to_string(),
            ..Default::default()
        }
    }

    pub fn with_username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn with_password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub fn with_sasl_type_scram_sha(mut self, sasl_type_scram_sha: &str) -> Self {
        self.sasl_type_scram_sha = Some(sasl_type_scram_sha.to_string());
        self
    }

    pub fn with_sasl_mechanism(mut self, mechanism: &str) -> Self {
        self.sasl_mechanism = Some(mechanism.to_string());
        self
    }

    pub fn with_security_protocol(mut self, protocol: &str) -> Self {
        self.security_protocol = Some(protocol.to_string());
        self
    }

    pub fn with_cert_path(mut self, cert_path: &str) -> Self {
        self.cert_path = cert_path.to_string();
        self
    }

    pub fn with_message_timeout(mut self, timeout: Duration) -> Self {
        self.message_timeout = Some(timeout);
        self
    }

    pub fn with_message_max_bytes(mut self, message_max_bytes: usize) -> Self {
        self.message_max_bytes = Some(message_max_bytes);
        self
    }

    pub fn with_auto_offset_reset(mut self, auto_offset_reset: &str) -> Self {
        self.auto_offset_reset = Some(auto_offset_reset.to_string());
        self
    }

    pub fn with_graceful_wait_timeout(mut self, timeout: Duration) -> Self {
        self.graceful_wait_timeout = Some(timeout);
        self
    }

    pub fn with_message_send_max_retries(mut self, max_retries: usize) -> Self {
        self.message_send_max_retries = Some(max_retries);
        self
    }

    pub fn with_enable_auto_commit(mut self, enable_auto_commit: bool) -> Self {
        self.enable_auto_commit = Some(enable_auto_commit);
        self
    }

    pub fn build(self) -> KafkaConfig {
        KafkaConfig {
            brokers: self.brokers,
            username: self.username,
            password: self.password,
            sasl_type_scram_sha: self
                .sasl_type_scram_sha
                .unwrap_or("SCRAM-SHA-256".to_string()),
            sasl_mechanism: self.sasl_mechanism.unwrap_or("PLAIN".to_string()),
            security_protocol: self.security_protocol.unwrap_or("PLAINTEXT".to_string()),
            cert_path: self.cert_path,
            insecure_skip_verify: self.insecure_skip_verify.unwrap_or(false),
            message_timeout: self.message_timeout.unwrap_or(Duration::from_secs(10)),
            message_max_bytes: self.message_max_bytes.unwrap_or(1024 * 1024),
            message_send_max_retries: self.message_send_max_retries.unwrap_or(3),
            enable_auto_commit: self.enable_auto_commit.unwrap_or(false),
            auto_offset_reset: self.auto_offset_reset.unwrap_or("latest".to_string()),
            graceful_wait_timeout: self.graceful_wait_timeout.unwrap_or(Duration::from_secs(3)),
        }
    }
}
