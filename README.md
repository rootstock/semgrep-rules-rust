Example of Github Actions Workflow for these Semgrep rules:

```yaml
name: "Semgrep Security Scan"

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
  schedule:
    - cron: '18 9 * * 4'

jobs:
  semgrep:
    name: Semgrep Scan
    runs-on: 'ubuntu-latest'
    permissions:
      security-events: write
      packages: read
      actions: read
      contents: read

    steps:
    - name: Checkout repository
      uses: actions/checkout@v4

    - name: Checkout rules repository
      uses: actions/checkout@v4
      with:
        repository: rootstock/semgrep-rules-rust
        path: ./semgrep-rules

    - name: Setup Python
      uses: actions/setup-python@v4
      with:
        python-version: '3.x'

    - name: Install Semgrep
      run: |
        python -m pip install --upgrade pip
        pip install semgrep

    - name: Run Semgrep
      run: |
        semgrep \
          --config=p/rust \
          --config=./semgrep-rules/security/ \
          --sarif \
          --output=semgrep.sarif \
          --verbose \
          .
      continue-on-error: true

    - name: Upload SARIF file to GitHub
      uses: github/codeql-action/upload-sarif@v3
      if: always()
      with:
        sarif_file: semgrep.sarif
        category: semgrep
```