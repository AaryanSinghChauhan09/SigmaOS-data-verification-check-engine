//! Integration tests for SigmaOS-data-verification-check-engine.

use sigmaos_data_verification_check_engine_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
