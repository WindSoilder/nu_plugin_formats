use nu_test_support::nu_with_plugins;

// Note: the tests can only run successfully if nushell binary is in `target/debug/`
#[test]
fn from_eml_get_to_field() {
    let actual = nu_with_plugins!(
        cwd: "tests",
        plugin: ("nu_plugin_formats"),
        format!("open sample.eml | get To.Address")
    );

    assert_eq!(actual.out, "to@example.com");
}
