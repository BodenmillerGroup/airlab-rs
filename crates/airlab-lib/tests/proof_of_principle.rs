use airlab_lib::b64::{b64u_decode_to_string, b64u_encode};

#[test]
#[ignore = "temporarily ignored in CI while act/container test execution is unstable"]
fn b64u_roundtrip_proof_of_principle() -> airlab_lib::b64::Result<()> {
    let input = "airlab-proof-of-principle";

    let encoded = b64u_encode(input);
    let decoded = b64u_decode_to_string(&encoded)?;

    assert_eq!(decoded, input);
    Ok(())
}
