# Frontend Workflows

This directory owns app CI and the inactive Pages placeholder. CI runs from the
repository root and uses `project/ci/versions`. Do not share this
configuration with the server through a submodule; use reusable GitHub workflows
later only if common workflow logic becomes substantial.
