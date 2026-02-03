// Integration tests for CFAD
// These tests use wiremock to mock the Cloudflare API

mod integration {
    mod client_tests;
    mod dns_tests;
    // Add more integration test modules here as they're created
    // mod zone_tests;
    // mod cache_tests;
}
