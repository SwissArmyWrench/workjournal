# 1.1.0

### API changes (Non-breaking)

- Added `workjournal configpath` command to print path to expected location of `config.yaml`
- Added `workjournal active` to print the current active job
- Added `workjournal mknote --job 12345` (or `-j`) to make a note to a job other than the current job

### Bug fixes and patches

- Fixed issue of printing headers for files containing no relevant notes for the requested job
- Fixed issue of being unable to change away from jobs with fewer than 5 digits after changing to them
- File headers for `printout` commands now only print the filename and not the full path
