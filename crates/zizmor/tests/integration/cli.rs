use crate::common::{OutputMode, input_under_test, zizmor};

/// Test that `--gh-token`, `--github-token` and `--zizmor-github-token` conflict with each other.
#[test]
fn test_gh_token_github_token_zizmor_github_token_conflict() -> anyhow::Result<()> {
    // As CLI flags.
    insta::assert_snapshot!(
        zizmor()
            .offline(true)
            .expects_failure(2)
            .args(["--gh-token=x", "--github-token=x"])
            .run()?,
        @r"
    error: the argument '--gh-token <GH_TOKEN>' cannot be used with '--github-token <GITHUB_TOKEN>'

    Usage: zizmor --gh-token <GH_TOKEN> --offline --no-progress --show-audit-urls <SHOW_AUDIT_URLS> <INPUTS>...

    For more information, try '--help'.
    "
    );

    insta::assert_snapshot!(
        zizmor()
            .offline(true)
            .expects_failure(2)
            .args(["--gh-token=x", "--zizmor-github-token=x"])
            .run()?,
        @r"
    error: the argument '--gh-token <GH_TOKEN>' cannot be used with '--zizmor-github-token <ZIZMOR_GITHUB_TOKEN>'

    Usage: zizmor --gh-token <GH_TOKEN> --offline --no-progress --show-audit-urls <SHOW_AUDIT_URLS> <INPUTS>...

    For more information, try '--help'.
    "
    );

    insta::assert_snapshot!(
        zizmor()
            .offline(true)
            .expects_failure(2)
            .args(["--github-token=x", "--zizmor-github-token=x"])
            .run()?,
        @r"
    error: the argument '--github-token <GITHUB_TOKEN>' cannot be used with '--zizmor-github-token <ZIZMOR_GITHUB_TOKEN>'

    Usage: zizmor --offline --no-progress --show-audit-urls <SHOW_AUDIT_URLS> <INPUTS>...

    For more information, try '--help'.
    "
    );

    // As environment variables.
    insta::assert_snapshot!(
        zizmor()
            .offline(true)
            .expects_failure(2)
            .setenv("GH_TOKEN", "x")
            .setenv("GITHUB_TOKEN", "x")
            .run()?,
        @r"
    error: the argument '--gh-token <GH_TOKEN>' cannot be used with '--github-token <GITHUB_TOKEN>'

    Usage: zizmor --offline --no-progress --show-audit-urls <SHOW_AUDIT_URLS> --gh-token <GH_TOKEN> <INPUTS>...

    For more information, try '--help'.
    "
    );

    insta::assert_snapshot!(
        zizmor()
            .offline(true)
            .expects_failure(2)
            .setenv("GH_TOKEN", "x")
            .setenv("ZIZMOR_GITHUB_TOKEN", "x")
            .run()?,
        @r"
    error: the argument '--gh-token <GH_TOKEN>' cannot be used with '--zizmor-github-token <ZIZMOR_GITHUB_TOKEN>'

    Usage: zizmor --offline --no-progress --show-audit-urls <SHOW_AUDIT_URLS> --gh-token <GH_TOKEN> <INPUTS>...

    For more information, try '--help'.
    "
    );

    insta::assert_snapshot!(
        zizmor()
            .offline(true)
            .expects_failure(2)
            .setenv("GITHUB_TOKEN", "x")
            .setenv("ZIZMOR_GITHUB_TOKEN", "x")
            .run()?,
        @r"
    error: the argument '--github-token <GITHUB_TOKEN>' cannot be used with '--zizmor-github-token <ZIZMOR_GITHUB_TOKEN>'

    Usage: zizmor --offline --no-progress --show-audit-urls <SHOW_AUDIT_URLS> <INPUTS>...

    For more information, try '--help'.
    "
    );

    Ok(())
}

/// Test that `-qq` suppresses the version banner and "no findings" message.
#[test]
fn test_qq_suppresses_status_output() -> anyhow::Result<()> {
    // With no findings and -qq, stdout should be empty.
    insta::assert_snapshot!(
        zizmor()
            .args(["-qq", "--persona=pedantic"])
            .input(input_under_test("concurrency-limits/cancel-false.yml"))
            .run()?,
        @""
    );

    // With -qq, neither the version banner nor the no findings message should appear.
    insta::assert_snapshot!(
        zizmor()
            .output(OutputMode::Both)
            .args(["-qq", "--persona=pedantic"])
            .input(input_under_test("concurrency-limits/cancel-false.yml"))
            .run()?,
        @""
    );

    Ok(())
}
