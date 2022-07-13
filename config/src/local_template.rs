use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Local {
    // version tracks the current version of the defaults so we can migrate old -> new
    // this is specifically important whenever we decide to change the default value
    // for an existing parameter. this field tag must be updated any time we add a new version.
    version: u32,

    // environmental (may be overridden)
    // when enabled, stores blocks indefinitally, otherwise, only the most recents blocks
    // are being kept around. ( the precise number of recent blocks depends on the consensus parameters )
    archival: bool,

    // gossip_node.go
    // how many peers to propagate to?
    gossip_fanout: i32,
    net_address: String,

    // 1 * time.minute = 60000000000 ns
    reconnect_time: Duration,

    // what we should tell people to connect to
    public_address: String,

    max_connections_per_ip: i32,

    // 0 == disable
    peer_ping_period_seconds: i32,

    // for https serving
    tls_cert_file: String,
    tls_key_file: String,

    // logging
    base_logger_debug_level: u32,
    // if this is 0, do not produce agreement.cadaver
    cadaver_size_target: u32,

    // incoming_connections_limit specifies the max number of long-lived incoming
    // connections. 0 means no connections allowed. must be non-negative.
    // estimating 5mb per incoming connection, 5mb*800 = 4gb
    incoming_connections_limit: i32,

    // broadcast_connections_limit specifies the number of connections that
    // will receive broadcast (gossip) messages from this node.  if the
    // node has more connections than this number, it will send broadcasts
    // to the top connections by priority (outgoing connections first, then
    // by money held by peers based on their participation key).  0 means
    // no outgoing messages (not even transaction broadcasting to outgoing
    // peers).  -1 means unbounded (default).
    broadcast_connections_limit: i32,

    // announce_participation_key specifies that this node should announce its
    // participation key (with the largest stake) to its gossip peers.  this
    // allows peers to prioritize our connection, if necessary, in case of a
    // do_s attack.  disabling this means that the peers will not have any
    // additional information to allow them to prioritize our connection.
    announce_participation_key: bool,

    // priority_peers specifies peer ip addresses that should always get
    // outgoing broadcast messages from this node.
    priority_peers: HashMap<String, bool>,

    // to make sure the algod process does not run out of f_ds, algod ensures
    // that rlimit_nofile >= incoming_connections_limit + rest_connections_hard_limit +
    // reserved_f_ds. reserved_f_ds are meant to leave room for short-lived f_ds like
    // dns queries, sq_lite files, etc. this parameter shouldn't be changed.
    reserved_f_ds: u32,

    // local server
    // api endpoint address
    endpoint_address: String,

    // timeouts passed to the rest http.server implementation
    rest_read_timeout_seconds: i32,
    rest_write_timeout_seconds: i32,

    // srv-based phonebook
    dns_bootstrap_id: String,

    // log file size limit in bytes. when set to 0 logs will be written to stdout.
    log_size_limit: u32,

    // text/template for creating log archive filename.
    // available template vars:
    // time at start of log: {{.year}} {{.month}} {{.day}} {{.hour}} {{.minute}} {{.second}}
    // time at end of log: {{.end_year}} {{.end_month}} {{.end_day}} {{.end_hour}} {{.end_minute}} {{.end_second}}
    //
    // if the filename ends with .gz or .bz2 it will be compressed.
    //
    // default: "node.archive.log" (no rotation, clobbers previous archive)
    log_archive_name: String,

    // log_archive_max_age will be parsed by time.parse_duration().
    // valid units are 's' seconds, 'm' minutes, 'h' hours
    log_archive_max_age: String,

    // number of consecutive attempts to catchup after which we replace the peers we're connected to
    catchup_failure_peer_refresh_rate: i32,

    // where should the node exporter listen for metrics
    node_exporter_listen_address: String,

    // enable metric reporting flag
    enable_metric_reporting: bool,

    // enable top accounts reporting flag
    enable_top_accounts_reporting: bool,

    // enable agreement reporting flag. currently only prints additional period events.
    enable_agreement_reporting: bool,

    // enable agreement timing metrics flag
    enable_agreement_time_metrics: bool,

    // the path to the node exporter.
    node_exporter_path: String,

    // the fallback dns resolver address that would be used if the system resolver would fail to retrieve srv records
    fallback_dns_resolver_address: String,

    // exponential increase factor of transaction pool's fee threshold, should always be 2 in production
    tx_pool_exponential_increase_factor: u32,

    suggested_fee_block_history: i32,

    // tx_pool_size is the number of transactions that fit in the transaction pool
    tx_pool_size: i32,

    // number of seconds allowed for syncing transactions
    tx_sync_timeout_seconds: i64,

    // number of seconds between transaction synchronizations
    tx_sync_interval_seconds: i64,

    // the number of incoming message hashes buckets.
    incoming_message_filter_bucket_count: i32,

    // the size of each incoming message hash bucket.
    incoming_message_filter_bucket_size: i32,

    // the number of outgoing message hashes buckets.
    outgoing_message_filter_bucket_count: i32,

    // the size of each outgoing message hash bucket.
    outgoing_message_filter_bucket_size: i32,

    // enable the filtering of outgoing messages
    enable_outgoing_network_message_filtering: bool,

    // enable the filtering of incoming messages
    enable_incoming_message_filter: bool,

    // control enabling / disabling deadlock detection.
    // negative (-1) to disable, positive (1) to enable, 0 for default.
    deadlock_detection: i32,

    // the threshold used for deadlock detection, in seconds.
    deadlock_detection_threshold: i32,

    // prefer to run algod hosted (under algoh)
    // observed by `goal` for now.
    run_hosted: bool,

    // the maximal number of blocks that catchup will fetch in parallel.
    // if less than protocol.seed_lookback, then protocol.seed_lookback will be used as to limit the catchup.
    // setting this variable to 0 would disable the catchup
    catchup_parallel_blocks: u32,

    // generate assemble_block_metrics telemetry event
    enable_assemble_stats: bool,

    // generate process_block_metrics telemetry event
    enable_process_block_stats: bool,

    // suggested_fee_sliding_window_size is number of past blocks that will be considered in computing the suggested fee
    suggested_fee_sliding_window_size: u32,

    // the max size the sync server would return
    tx_sync_serve_response_size: i32,

    // is_indexer_active indicates whether to activate the indexer for fast retrieval of transactions
    // note -- indexer cannot operate on non archival nodes
    is_indexer_active: bool,

    // use_x_forwarded_for_address indicates whether or not the node should use the x-forwarded-for http header when
    // determining the source of a connection.  if used, it should be set to the String "x-forwarded-for", unless the
    // proxy vendor provides another header field.  in the case of cloud_flare proxy, the "cf-connecting-ip" header
    // field can be used.
    use_x_forwarded_for_address_field: String,

    // force_relay_messages indicates whether the network library relay messages even in the case that no net_address was specified.
    force_relay_messages: bool,

    // connections_rate_limiting_window_seconds is being used in conjunction with connections_rate_limiting_count,
    // see connections_rate_limiting_count description for further information. providing a zero value
    // in this variable disables the connection rate limiting.
    connections_rate_limiting_window_seconds: u32,

    // connections_rate_limiting_count is being used along with connections_rate_limiting_window_seconds to determine if
    // a connection request should be accepted or not. the gossip network examine all the incoming requests in the past
    // connections_rate_limiting_window_seconds seconds that share the same origin. if the total count exceed the connections_rate_limiting_count
    // value, the connection is refused.
    connections_rate_limiting_count: u32,

    // enable_request_logger enabled the logging of the incoming requests to the telemetry server.
    enable_request_logger: bool,

    // peer_connections_update_interval defines the interval at which the peer connections information is being sent to the
    // telemetry ( when enabled ). defined in seconds.
    peer_connections_update_interval: i32,

    // enable_profiler enables the go pprof endpoints, should be false if
    // the algod api will be exposed to untrusted individuals
    enable_profiler: bool,

    // enable_runtime_metrics exposes go runtime metrics in /metrics and via node_exporter.
    enable_runtime_metrics: bool,

    // telemetry_to_log records messages to node.log that are normally sent to remote event monitoring
    telemetry_to_log: bool,

    // dns_security_flags instructs algod validating dns responses.
    // possible fla values
    // 0x00 - disabled
    // 0x01 (dnssec_srv) - validate srv response
    // 0x02 (dnssec_relay_addr) - validate relays' names to addresses resolution
    // 0x04 (dnssec_telemetry_addr) - validate telemetry and metrics names to addresses resolution
    // ...
    dns_security_flags: u32,

    // enable_ping_handler controls whether the gossip node would respond to ping messages with a pong message.
    enable_ping_handler: bool,

    // disable_outgoing_connection_throttling disables the connection throttling of the network library, which
    // allow the network library to continuesly disconnect relays based on their relative ( and absolute ) performance.
    disable_outgoing_connection_throttling: bool,

    // network_protocol_version overrides network protocol version ( if present )
    network_protocol_version: String,

    // catchpoint_interval sets the interval at which catchpoint are being generated. setting this to 0 disables the catchpoint from being generated.
    // see catchpoint_tracking for more details.
    catchpoint_interval: u32,

    // catchpoint_file_history_length defines how many catchpoint files we want to store back.
    // 0 means don't store any, -1 mean unlimited and positive number suggest the number of most recent catchpoint files.
    catchpoint_file_history_length: i32,

    // enable_ledger_service enables the ledger serving service. the functionality of this depends on net_address, which must also be provided.
    // this functionality is required for the catchpoint catchup.
    enable_ledger_service: bool,

    // enable_block_service enables the block serving service. the functionality of this depends on net_address, which must also be provided.
    // this functionality is required for the catchup.
    enable_block_service: bool,

    // enable_gossip_block_service enables the block serving service over the gossip network. the functionality of this depends on net_address, which must also be provided.
    // this functionality is required for the relays to perform catchup from nodes.
    enable_gossip_block_service: bool,

    // catchup_http_block_fetch_timeout_sec controls how long the http query for fetching a block from a relay would take before giving up and trying another relay.
    catchup_http_block_fetch_timeout_sec: i32,

    // catchup_gossip_block_fetch_timeout_sec controls how long the gossip query for fetching a block from a relay would take before giving up and trying another relay.
    catchup_gossip_block_fetch_timeout_sec: i32,

    // catchup_ledger_download_retry_attempts controls the number of attempt the ledger fetching would be attempted before giving up catching up to the provided catchpoint.
    catchup_ledger_download_retry_attempts: i32,

    // catchup_ledger_download_retry_attempts controls the number of attempt the block fetching would be attempted before giving up catching up to the provided catchpoint.
    catchup_block_download_retry_attempts: i32,

    // enable_developer_api enables teal/compile, teal/dryrun api endpoints.
    // this functionality is disabled by default.
    enable_developer_api: bool,

    // optimize_accounts_database_on_startup controls whether the accounts database would be optimized
    // on algod startup.
    optimize_accounts_database_on_startup: bool,

    // catchpoint_tracking determines if catchpoints are going to be tracked. the value is interpreted as follows:
    // a value of -1 means "don't track catchpoints".
    // a value of 1 means "track catchpoints as long as catchpoint_interval is also set to a positive non-zero value". if catchpoint_interval <= 0, no catchpoint tracking would be performed.
    // a value of 0 means automatic, which is the default value. in this mode, a non archival node would not track the catchpoints, and an archival node would track the catchpoints as long as catchpoint_interval > 0.
    // other values of catchpoint_tracking would give a warning in the log file, and would behave as if the default value was provided.
    catchpoint_tracking: i64,

    // ledger_synchronous_mode defines the synchronous mode used by the ledger database. the supported options are:
    // 0 - sq_lite continues without syncing as soon as it has handed data off to the operating system.
    // 1 - sq_lite database engine will still sync at the most critical moments, but less often than in full mode.
    // 2 - sq_lite database engine will use the x_sync method of the vfs to ensure that all content is safely written to the disk surface prior to continuing. on mac os, the data is additionally syncronized via fullfsync.
    // 3 - in addition to what being done in 2, it provides additional durability if the commit is followed closely by a power loss.
    // for further information see the description of synchronous_mode in dbutil.go
    ledger_synchronous_mode: i32,

    // accounts_rebuild_synchronous_mode defines the synchronous mode used by the ledger database while the account database is being rebuilt. this is not a typical operational usecase,
    // and is expected to happen only on either startup ( after enabling the catchpoint interval, or on certain database upgrades ) or during fast catchup. the values specified here
    // and their meanings are identical to the ones in ledger_synchronous_mode.
    accounts_rebuild_synchronous_mode: i32,

    // max_catchpoint_download_duration defines the maximum duration a client will be keeping the outgoing connection of a catchpoint download request open for processing before
    // shutting it down. networks that have large catchpoint files, slow connection or slow storage could be a good reason to increase this value. note that this is a client-side only
    // configuration value, and it's independent of the actual catchpoint file size.
    max_catchpoint_download_duration: Duration,

    // min_catchpoint_file_download_bytes_per_second defines the minimal download speed that would be considered to be "acceptable" by the catchpoint file fetcher, measured in bytes per seconds. if the
    // provided stream speed drops below this threshold, the connection would be recycled. note that this field is evaluated per catchpoint "chunk" and not on it's own. if this field is zero,
    // the default of 20480 would be used.
    min_catchpoint_file_download_bytes_per_second: u32,

    // trace_server is a host:port to report graph propagation trace info to.
    network_message_trace_server: String,

    // verified_transcations_cache_size defines the number of transactions that the verified transactions cache would hold before cycling the cache storage in a round-robin fashion.
    verified_transcations_cache_size: i32,

    // enable_catchup_from_archive_servers controls which peers the catchup service would use in order to catchup.
    // when enabled, the catchup service would use the archive servers before falling back to the relays.
    // on networks that doesn't have archive servers, this becomes a no-op, as the catchup service would have no
    // archive server to pick from, and therefore automatically selects one of the relay nodes.
    enable_catchup_from_archive_servers: bool,

    // disable_localhost_connection_rate_limit controls whether the incoming connection rate limit would apply for
    // connections that are originating from the local machine. setting this to "true", allow to create large
    // local-machine networks that won't trip the incoming connection limit observed by relays.
    disable_localhost_connection_rate_limit: bool,

    // block_service_custom_fallback_endpoints is a comma delimited list of endpoints which the block service uses to
    // redirect the http requests to in case it does not have the round. if it is not specified, will check
    // enable_block_service_fallback_to_archiver.
    block_service_custom_fallback_endpoints: String,

    // enable_block_service_fallback_to_archiver controls whether the block service redirects the http requests to
    // an archiver or return status_not_found (404) when in does not have the requested round, and
    // block_service_custom_fallback_endpoints is empty.
    // the archiver is randomly selected, if none is available, will return status_not_found (404).
    enable_block_service_fallback_to_archiver: bool,

    // catchup_block_validate_mode is a development and testing configuration used by the catchup service.
    // it can be used to omit certain validations to speed up the catchup process, or to apply extra validations which are redundant in normal operation.
    // this field is a bit-field with:
    // bit 0: (default 0) 0: verify the block certificate, 1: skip this validation
    // bit 1: (default 0) 0: verify payset committed hash in block header matches payset hash, 1: skip this validation
    // bit 2: (default 0) 0: don't verify the transaction signatures on the block are valid, 1: verify the transaction signatures on block
    // bit 3: (default 0) 0: don't verify that the hash of the recomputed payset matches the hash of the payset committed in the block header, 1: do perform the above verification
    // note: not all permutations of the above bitset are currently functional. in particular, the ones that are functional are:
    // 0  : default behavior.
    // 3  : speed up catchup by skipping necessary validations
    // 12 : perform all validation methods (normal and additional). these extra tests helps to verify the integrity of the compiled executable against
    //      previously used executabled, and would not provide any additional security guarantees.
    catchup_block_validate_mode: i32,

    // generate account_updates telemetry event
    enable_account_updates_stats: bool,

    // time interval in nanoseconds for generating account_updates telemetry event
    account_updates_stats_interval: Duration,

    // participation_keys_refresh_interval is the duration between two consecutive checks to see if new participation
    // keys have been placed on the genesis directory.
    participation_keys_refresh_interval: Duration,

    // disable_networking disables all the incoming and outgoing communication a node would perform. this is useful
    // when we have a single-node private network, where there is no other nodes that need to be communicated with.
    // features like catchpoint catchup would be rendered completly non-operational, and many of the node inner
    // working would be completly dis-functional.
    disable_networking: bool,

    // force_fetch_transactions allows to explicitly configure a node to retrieve all the transactions
    // into it's transaction pool, even if those would not be required as the node doesn't
    // participate in the consensus or used to relay transactions.
    force_fetch_transactions: bool,

    // enable_verbosed_transaction_sync_logging enables the transaction sync to write extensive
    // message exchange information to the log file. this option is disabled by default,
    // so that the log files would not grow too rapidly.
    enable_verbosed_transaction_sync_logging: bool,

    // transaction_sync_data_exchange_rate overrides the auto-calculated data exchange rate between each
    // two peers. the unit of the data exchange rate is in bytes per second. setting the value to
    // zero implies allowing the transaction sync to dynamically calculate the value.
    transaction_sync_data_exchange_rate: u32,

    // transaction_sync_significant_message_threshold define the threshold used for a transaction sync
    // message before it can be used for calculating the data exchange rate. setting this to zero
    // would use the default values. the threshold is defined in units of bytes.
    transaction_sync_significant_message_threshold: u32,

    // proposal_assembly_time is the max amount of time to spend on generating a proposal block.
    proposal_assembly_time: Duration,

    // when the number of http connections to the rest layer exceeds the soft limit,
    // we start returning http code 429 too many requests.
    rest_connections_soft_limit: u32,
    // the http server does not accept new connections as long we have this many
    // (hard limit) connections already.
    rest_connections_hard_limit: u32,

    // max_api_resources_per_account sets the maximum total number of resources (created assets, created apps,
    // asset holdings, and application local state) per account that will be allowed in account_information
    // rest api responses before returning a 400 bad request. set zero for no limit.
    max_api_resources_per_account: u32,

    // agreement_incoming_votes_queue_length sets the size of the buffer holding incoming votes.
    agreement_incoming_votes_queue_length: u32,

    // agreement_incoming_proposals_queue_length sets the size of the buffer holding incoming proposals.
    agreement_incoming_proposals_queue_length: u32,

    // agreement_incoming_bundles_queue_length sets the size of the buffer holding incoming bundles.
    agreement_incoming_bundles_queue_length: u32,
}

pub fn default_local() -> Local {
    Local {
        version: 22,
        account_updates_stats_interval: Duration::from_nanos(5000000000),
        accounts_rebuild_synchronous_mode: 1,
        agreement_incoming_bundles_queue_length: 7,
        agreement_incoming_proposals_queue_length: 25,
        agreement_incoming_votes_queue_length: 10000,
        announce_participation_key: true,
        archival: false,
        base_logger_debug_level: 4,
        block_service_custom_fallback_endpoints: "".to_string(),
        broadcast_connections_limit: -1,
        cadaver_size_target: 1073741824,
        catchpoint_file_history_length: 365,
        catchpoint_interval: 10000,
        catchpoint_tracking: 0,
        catchup_block_download_retry_attempts: 1000,
        catchup_block_validate_mode: 0,
        catchup_failure_peer_refresh_rate: 10,
        catchup_gossip_block_fetch_timeout_sec: 4,
        catchup_http_block_fetch_timeout_sec: 4,
        catchup_ledger_download_retry_attempts: 50,
        catchup_parallel_blocks: 16,
        connections_rate_limiting_count: 60,
        connections_rate_limiting_window_seconds: 1,
        dns_bootstrap_id: "<network>.algorand.network".to_string(),
        dns_security_flags: 1,
        deadlock_detection: 0,
        deadlock_detection_threshold: 30,
        disable_localhost_connection_rate_limit: true,
        disable_networking: false,
        disable_outgoing_connection_throttling: false,
        enable_account_updates_stats: false,
        enable_agreement_reporting: false,
        enable_agreement_time_metrics: false,
        enable_assemble_stats: false,
        enable_block_service: false,
        enable_block_service_fallback_to_archiver: true,
        enable_catchup_from_archive_servers: false,
        enable_developer_api: false,
        enable_gossip_block_service: true,
        enable_incoming_message_filter: false,
        enable_ledger_service: false,
        enable_metric_reporting: false,
        enable_outgoing_network_message_filtering: true,
        enable_ping_handler: true,
        enable_process_block_stats: false,
        enable_profiler: false,
        enable_request_logger: false,
        enable_runtime_metrics: false,
        enable_top_accounts_reporting: false,
        enable_verbosed_transaction_sync_logging: false,
        endpoint_address: "127.0.0.1:0".to_string(),
        fallback_dns_resolver_address: "".to_string(),
        force_fetch_transactions: false,
        force_relay_messages: false,
        gossip_fanout: 4,
        incoming_connections_limit: 800,
        incoming_message_filter_bucket_count: 5,
        incoming_message_filter_bucket_size: 512,
        is_indexer_active: false,
        ledger_synchronous_mode: 2,
        log_archive_max_age: "".to_string(),
        log_archive_name: "node.archive.log".to_string(),
        log_size_limit: 1073741824,
        max_api_resources_per_account: 100000,
        max_catchpoint_download_duration: Duration::from_nanos(7200000000000),
        max_connections_per_ip: 30,
        min_catchpoint_file_download_bytes_per_second: 20480,
        net_address: "".to_string(),
        network_message_trace_server: "".to_string(),
        network_protocol_version: "".to_string(),
        node_exporter_listen_address: ":9100".to_string(),
        node_exporter_path: "./node_exporter".to_string(),
        optimize_accounts_database_on_startup: false,
        outgoing_message_filter_bucket_count: 3,
        outgoing_message_filter_bucket_size: 128,
        participation_keys_refresh_interval: Duration::from_nanos(60000000000),
        peer_connections_update_interval: 3600,
        peer_ping_period_seconds: 0,
        priority_peers: HashMap::<String, bool>::new(),
        proposal_assembly_time: Duration::from_nanos(250000000),
        public_address: "".to_string(),
        reconnect_time: Duration::from_nanos(60000000000),
        reserved_f_ds: 256,
        rest_connections_hard_limit: 2048,
        rest_connections_soft_limit: 1024,
        rest_read_timeout_seconds: 15,
        rest_write_timeout_seconds: 120,
        run_hosted: false,
        suggested_fee_block_history: 3,
        suggested_fee_sliding_window_size: 50,
        tls_cert_file: "".to_string(),
        tls_key_file: "".to_string(),
        telemetry_to_log: true,
        transaction_sync_data_exchange_rate: 0,
        transaction_sync_significant_message_threshold: 0,
        tx_pool_exponential_increase_factor: 2,
        tx_pool_size: 15000,
        tx_sync_interval_seconds: 60,
        tx_sync_serve_response_size: 1000000,
        tx_sync_timeout_seconds: 30,
        use_x_forwarded_for_address_field: "".to_string(),
        verified_transcations_cache_size: 30000,
    }
}
