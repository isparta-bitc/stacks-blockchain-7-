// Copyright (C) 2013-2020 Blockstack PBC, a public benefit corporation
// Copyright (C) 2020 Stacks Open Internet Foundation
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use prometheus::{Histogram, IntCounter, IntCounterVec, IntGauge, IntGaugeVec};

lazy_static! {
    pub static ref RPC_REQUEST_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_rpc_request_total",
        "Total number of RPC requests made"
    )).unwrap();

    pub static ref STX_BLOCKS_RECEIVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_blocks_received_total",
        "Total number of Stacks blocks received"
    )).unwrap();

    pub static ref STX_MICRO_BLOCKS_RECEIVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_micro_blocks_received_total",
        "Total number of Stacks micro blocks received"
    )).unwrap();

    pub static ref STX_BLOCKS_SERVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_blocks_served_total",
        "Total number of Stacks blocks served"
    )).unwrap();

    pub static ref STX_MICRO_BLOCKS_SERVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_micro_blocks_served_total",
        "Total number of Stacks micro blocks served"
    )).unwrap();

    pub static ref STX_CONFIRMED_MICRO_BLOCKS_SERVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_confirmed_micro_blocks_served_total",
        "Total number of Stacks blocks served"
    )).unwrap();

    pub static ref TXS_RECEIVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_transactions_received_total",
        "Total number of transactions received and relayed"
    )).unwrap();

    pub static ref BTC_BLOCKS_RECEIVED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_btc_blocks_received_total",
        "Total number of blocks processed from the burnchain"
    )).unwrap();

    pub static ref BTC_OPS_SENT_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_btc_ops_sent_total",
        "Total number of ops (key registrations, block commits, user burn supports) submitted to the burnchain"
    )).unwrap();

    pub static ref STX_BLOCKS_PROCESSED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_blocks_processed_total",
        "Total number of stacks blocks processed"
    )).unwrap();

    pub static ref STX_BLOCKS_MINED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_stx_blocks_mined_total",
        "Total number of stacks blocks mined by node"
    )).unwrap();

    pub static ref WARNING_EMITTED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_warning_emitted_total",
        "Total number of warning logs emitted by node"
    )).unwrap();

    pub static ref ERRORS_EMITTED_COUNTER: IntCounter = register_int_counter!(opts!(
        "stacks_node_errors_emitted_total",
        "Total number of error logs emitted by node"
    )).unwrap();

    pub static ref ACTIVE_MINERS_COUNT_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_active_miners_total",
        "Total number of active miners"
    )).unwrap();

    pub static ref STACKS_TIP_HEIGHT_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_stacks_tip_height",
        "Stacks chain tip height"
    )).unwrap();

    pub static ref BURNCHAIN_HEIGHT_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_burn_block_height",
        "Burnchain tip height"
    )).unwrap();

    pub static ref INBOUND_NEIGHBORS_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_neighbors_inbound",
        "Total count of current known inbound neighbors"
    )).unwrap();

    pub static ref OUTBOUND_NEIGHBORS_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_neighbors_outbound",
        "Total count of current known outbound neighbors"
    )).unwrap();

    pub static ref INBOUND_BANDWIDTH_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_bandwidth_inbound",
        "Total inbound bandwidth total in bytes"
    )).unwrap();

    pub static ref OUTBOUND_BANDWIDTH_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_bandwidth_outbound",
        "Total outbound bandwidth total in bytes"
    )).unwrap();

    pub static ref INBOUND_RPC_BANDWIDTH_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_rpc_bandwidth_inbound",
        "Total RPC inbound bandwidth in bytes"
    )).unwrap();

    pub static ref OUTBOUND_RPC_BANDWIDTH_GAUGE: IntGauge = register_int_gauge!(opts!(
        "stacks_node_rpc_bandwidth_outbound",
        "Total RPC outbound bandwidth in bytes"
    )).unwrap();

    pub static ref MSG_COUNTER_VEC: IntCounterVec = register_int_counter_vec!(
        "stacks_node_message_count",
        "Stacks message count by type of message",
        &["name"]
    ).unwrap();

    pub static ref RPC_REQUEST_COUNTER_VEC: IntCounterVec = register_int_counter_vec!(
        "stacks_node_rpc_request",
        "Total number of RPC requests made by path",
        &["path", "method"]
    ).unwrap();

    pub static ref STX_MEMPOOL_GC: IntCounter = register_int_counter!(opts!(
        "stacks_node_mempool_gc_count",
        "Total count of all mempool garbage collections"
    )).unwrap();

    pub static ref STX_SMART_CONTRACT_COUNT: IntCounter = register_int_counter!(opts!(
        "stacks_node_smart_contracts",
        "Total count of smart contracts"
    )).unwrap();

    pub static ref MEMPOOL_OUTSTANDING_TXS: IntGauge = register_int_gauge!(opts!(
        "stacks_node_mempool_outstanding_txs",
        "Number of still-unprocessed transactions received by this node since it started",
        labels! {"handler" => "all",}
    )).unwrap();

    pub static ref MEMPOOL_TX_CONFIRM_TIME: Histogram = register_histogram!(histogram_opts!(
        "stacks_node_mempool_tx_confirm_times",
        "Time (seconds) between when a tx was received by this node's mempool and when a tx was first processed in a block",
        vec![300.0, 600.0, 900.0, 1200.0, 1500.0, 1800.0, 2100.0, 2400.0, 2700.0, 3000.0, 3600.0, 4200.0, 4800.0, 6000.0],
        labels! {"handler".to_string() => "all".to_string(),}
    )).unwrap();
}
