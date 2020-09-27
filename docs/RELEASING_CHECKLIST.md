# RELEASING CHECKLIST

- [ ] DOCS CHANGES
  - [ ] Check over README and make sure the help page is current
  - [ ] add changes to the CHANGELOG
  - [ ] Update the CONFIG_FILE_GUIDE if any new options have been added
  - [ ] Make sure the ROADMAP is updated to reflect any changes
  - [ ] Update **this** document if new items are needed.
- [ ] PUSHING A RELEASE
  - [ ] Push all changes that are intended for release. Do not tag that release.
  - [ ] Increment Cargo appropriately, and then push and tag *that* release.
  - [ ] Cargo publish the release.
  - [ ] Build the release, either with some github action or the old fashioned way.
  - [ ] When the builds have been prepared, add your CHANGELOG text to the release page.
  - [ ] Update scoop
