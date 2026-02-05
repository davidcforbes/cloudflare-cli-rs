// Integration tests for CFAD
// These tests use wiremock to mock the Cloudflare API

mod integration {
    mod cache_tests;
    mod client_tests;
    mod d1_tests;
    mod dns_import_tests;
    mod dns_tests;
    mod error_tests;
    mod r2_tests;
    mod token_tests;
    mod zone_tests;
}
