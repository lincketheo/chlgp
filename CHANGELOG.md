# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] 2024-06-16 <lincketheo@gmail.com>


### Added

- v1.1 Brazilian Portuguese translation.
- v1.1 German Translation
- v1.1 Spanish translation.
- v1.1 Italian translation.
- v1.1 Polish translation.
- v1.1 Ukrainian translation.

### Changed

- Use frontmatter title & description in each language version template
- Replace broken OpenGraph image with an appropriately-sized Keep a Changelog 
  image that will render properly (although in English for all languages)
- Fix OpenGraph title & description for all languages so the title and 
description when links are shared are language-appropriate

### Removed

- Trademark sign previously shown after the project description in version 
0.3.0

## [1.1.1] 2023-03-05 <lincketheo@gmail.com>

### Added

- Arabic translation (#444).
- v1.1 French translation.
- v1.1 Dutch translation (#371).
- v1.1 Russian translation (#410).
- v1.1 Japanese translation (#363).
- v1.1 Norwegian Bokmål translation (#383).
- v1.1 "Inconsistent Changes" Turkish translation (#347).
- Default to most recent versions available for each languages.
- Display count of available translations (26 to date!).
- Centralize all links into `/data/links.json` so they can be updated easily.

### Fixed

- Improve French translation (#377).
- Improve id-ID translation (#416).
- Improve Persian translation (#457).
- Improve Russian translation (#408).
- Improve Swedish title (#419).
- Improve zh-CN translation (#359).
- Improve French translation (#357).
- Improve zh-TW translation (#360, #355).
- Improve Spanish (es-ES) transltion (#362).
- Foldout menu in Dutch translation (#371).
- Missing periods at the end of each change (#451).
- Fix missing logo in 1.1 pages.
- Display notice when translation isn't for most recent version.
- Various broken links, page versions, and indentations.

### Changed

- Upgrade dependencies: Ruby 3.2.1, Middleman, etc.

### Removed

- Unused normalize.css file.
- Identical links assigned in each translation file.
- Duplicate index file for the english version.

## [1.1.0] 2019-02-15 <lincketheo@gmail.com>

### Added

- Danish translation (#297).
- Georgian translation from (#337).
- Changelog inconsistency section in Bad Practices.

### Fixed

- Italian translation (#332).
- Indonesian translation (#336).

## [1.0.0] 2017-06-20 <lincketheo@gmail.com>


### Added

- Version navigation.
- Links to latest released version in previous versions.
- "Why keep a changelog?" section.
- "Who needs a changelog?" section.
- "How do I make a changelog?" section.
- "Frequently Asked Questions" section.
- New "Guiding Principles" sub-section to "How do I make a changelog?".

### Changed

- Start using "changelog" over "change log" since it's the common usage.
- Start versioning based on the current English version at 0.3.0 to help
  translation authors keep things up-to-date.
- Rewrite "What makes unicorns cry?" section.
- Rewrite "Ignoring Deprecations" sub-section to clarify the ideal
  scenario.
- Improve "Commit log diffs" sub-section to further argument against
  them.
- Merge "Why can’t people just use a git log diff?" with "Commit log
  diffs".
- Fix typos in Simplified Chinese and Traditional Chinese translations.
- Fix typos in Brazilian Portuguese translation.
- Fix typos in Turkish translation.
- Fix typos in Czech translation.
- Fix typos in Swedish translation.
- Improve phrasing in French translation.
- Fix phrasing and spelling in German translation.

### Removed

- Section about "changelog" vs "CHANGELOG".

## [0.3.0] 2015-12-03 <lincketheo@gmail.com>


### Added

- RU translation 
- pt-BR translation 
- es-ES translation 

## [0.2.0] 2015-10-06 <lincketheo@gmail.com>


### Changed

- Remove exclusionary mentions of "open source" since this project can
  benefit both "open" and "closed" source projects equally.

## [0.1.0] 2015-10-06 <lincketheo@gmail.com>


### Added

- Answer "Should you ever rewrite a change log?".

### Changed

- Improve argument against commit logs.

## [0.0.8] 2015-02-17 <lincketheo@gmail.com>


### Changed

- Update year to match in every README example.
- Reluctantly stop making fun of Brits only, since most of the world
  writes dates in a strange way.

### Fixed

- Fix typos in recent README changes.
- Update outdated unreleased diff link.

## [0.0.7] 2015-02-16 <lincketheo@gmail.com>


### Added

- Link, and make it obvious that date format is ISO 8601.

### Changed

- Clarified the section on "Is there a standard change log format?".

### Fixed

- Fix Markdown links to tag comparison URL with footnote-style links.

## [0.0.6] 2014-12-12 <lincketheo@gmail.com>


### Added

- README section on "yanked" releases.

## [0.0.5] 2014-08-09 <lincketheo@gmail.com>


### Added

- Markdown links to version tags on release headings.
- Unreleased section to gather unreleased changes and encourage note
  keeping prior to releases.

## [0.0.4] 2014-08-09 <lincketheo@gmail.com>


### Added

- Better explanation of the difference between the file ("CHANGELOG")
  and its function "the change log".

### Changed

- Refer to a "change log" instead of a "CHANGELOG" throughout the site
  to differentiate between the file and the purpose of the file — the
  logging of changes.

### Removed

- Remove empty sections from CHANGELOG, they occupy too much space and
  create too much noise in the file. People will have to assume that the
  missing sections were intentionally left out because they contained no
  notable changes.

## [0.0.3] 2014-08-09 <lincketheo@gmail.com>


### Added

- "Why should I care?" section mentioning The Changelog podcast.

## [0.0.2] 2014-07-10 <lincketheo@gmail.com>


### Added

- Explanation of the recommended reverse chronological release ordering.

## [0.0.1] 2014-05-31 <lincketheo@gmail.com>


### Added

- This CHANGELOG file to hopefully serve as an evolving example of a
  standardized open source project CHANGELOG.
- CNAME file to enable GitHub Pages custom domain.
- README now contains answers to common questions about CHANGELOGs.
- Good examples and basic guidelines, including proper date formatting.
- Counter-examples: "What makes unicorns cry?".
