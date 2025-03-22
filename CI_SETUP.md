# CI/CD Pipeline Setup Guide

This guide will help you set up the necessary secrets and configurations for the Glowing Waffle CI/CD pipeline.

## Fixing Current CI/CD Issues

The following changes have been made to fix CI/CD pipeline issues:

1. **Format Job**: Updated the command to `cargo fmt --all --check` to properly check formatting
2. **Documentation Job**: Switched to using a Personal Access Token (PAT) for GitHub Pages deployment
3. **Benchmark Job**: Fixed benchmark output format to generate the expected JSON file
4. **Code Coverage Job**: Corrected the tarpaulin output directory path
5. **Test Job**: Added `--nocapture` flag for better test output visualization

## Required GitHub Secrets

### 1. Creating a Personal Access Token (PAT)

1. Go to your GitHub account settings → [Developer settings](https://github.com/settings/tokens) → Personal access tokens → Tokens (classic)
2. Click "Generate new token" → "Generate new token (classic)"
3. Give it a descriptive name like "glowing-waffle-ci"
4. Set the expiration as needed (1 year recommended)
5. Select the following scopes:
   - `repo` (Full control of private repositories)
   - `workflow` (Optional, for workflow control)
6. Click "Generate token"
7. **IMPORTANT**: Copy the generated token immediately - you won't be able to see it again!

### 2. Codecov Integration

1. Go to [Codecov](https://codecov.io/) and sign in with your GitHub account
2. Add your repository to Codecov
3. Go to repository settings in Codecov
4. Copy the Upload Token provided by Codecov

### 3. Creating a Crates.io Token

1. Go to [crates.io](https://crates.io/) and log in
2. Go to your Account Settings
3. Under "API Tokens", enter a description like "glowing-waffle-releases"
4. Click "New Token"
5. Copy the generated token

## Setting Up Secrets in GitHub

1. Go to your GitHub repository
2. Navigate to Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Add the following secrets:
   - Name: `PAT_TOKEN`, Value: [Your GitHub Personal Access Token]
   - Name: `CODECOV_TOKEN`, Value: [Your Codecov Upload Token]
   - Name: `CRATES_IO_TOKEN`, Value: [Your Crates.io API Token]

## Updating Local Workflow

When contributing to the project, make sure to:

1. Format code correctly before pushing:
   ```bash
   cargo fmt --all
   ```

2. Run clippy to check for linting issues:
   ```bash
   cargo clippy --all-features -- -D warnings
   ```

3. Run tests to ensure all tests pass:
   ```bash
   cargo test --all-features -- --nocapture
   ```

4. Optionally run benchmarks:
   ```bash
   cargo bench -- --output-format bencher
   ```

## Troubleshooting Common Issues

### GitHub Pages Deployment Failures

If you receive errors like "The process '/usr/bin/git' failed with exit code 128":
1. Ensure the `PAT_TOKEN` has proper permissions (repo scope)
2. Verify the GitHub Pages source is set to the gh-pages branch in repository settings

### Codecov Upload Failures

If you receive errors with Codecov uploads:
1. Ensure the `CODECOV_TOKEN` is set correctly
2. Verify the path to the coverage report is correct (`target/cobertura.xml`)
3. Check if the Codecov GitHub App has proper permissions to access the repository

### Benchmark Data Storage Issues

If benchmarks aren't being saved:
1. Ensure the output format is correct in the command (`--output-format bencher | tee benchmark-results.json`)
2. Verify the GitHub Pages branch is properly configured
3. Check that the `PAT_TOKEN` has sufficient permissions

## Next Steps

Once all secrets are properly set up:

1. Push a small change to trigger CI workflow
2. Verify that all jobs run successfully
3. Check the GitHub Pages deployment to ensure documentation and benchmarks are visible
4. Consider creating a test tag to verify the release process 