pub mod client;

pub use client::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_init() {
        let client = VeriVaultClient::new("http://localhost".into(), "contract".into());
        let note = client.create_note(100, ComplianceAttributes { kyc_level: 1, geo_region: "US".into() });
        assert_eq!(note.amount, 100);
    }
}
