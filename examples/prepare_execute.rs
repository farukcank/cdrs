// in feature="ssl" imports are unused until examples are implemented
#![allow(unused_imports, unused_variables)]
extern crate cdrs;

use cdrs::client::CDRS;
use cdrs::query::QueryParamsBuilder;
use cdrs::authenticators::PasswordAuthenticator;
use cdrs::compression::Compression;
use cdrs::consistency::Consistency;
#[cfg(not(feature = "ssl"))]
use cdrs::transport::Transport;
#[cfg(feature = "ssl")]
use cdrs::transport_ssl::Transport;

// default credentials
const _USER: &'static str = "cassandra";
const _PASS: &'static str = "cassandra";
const _ADDR: &'static str = "127.0.0.1:9042";

#[cfg(not(feature = "ssl"))]
fn main() {
    let authenticator = PasswordAuthenticator::new(_USER, _PASS);
    let tcp_transport = Transport::new(_ADDR).unwrap();
    let client = CDRS::new(tcp_transport, authenticator);
    let mut session = client.start(Compression::None).unwrap();

    // NOTE: keyspace "keyspace" should already exist
    let create_table_cql = "USE keyspace;".to_string();
    let with_tracing = false;
    let with_warnings = false;

    let prepared = session.prepare(create_table_cql, with_tracing, with_warnings)
        .unwrap()
        .get_body()
        .into_prepared()
        .unwrap();

    println!("prepared:\n{:?}", prepared);

    let execution_params = QueryParamsBuilder::new(Consistency::One).finalize();
    let query_id = prepared.id;
    let executed = session.execute(query_id, execution_params, false, false)
        .unwrap()
        .get_body()
        .into_set_keyspace()
        .unwrap();

    println!("executed:\n{:?}", executed);
}

#[cfg(feature = "ssl")]
fn main() {
    unimplemented!()
}