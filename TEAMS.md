# Forgejo teams

A team is a group of people who work together on a specific area to further Forgejo.

Some of the teams are trusted with access to exclusive resources that require credentials. To participate in such a team someone can open a pull request to add their name and their membership will be decided by the Forgejo community (see the [decision making document](DECISION-MAKING.md) for more information).

## Accessibility

Purpose: Work on improving Forgejo accessibility.

Team members:

* https://codeberg.org/Ryuno-Ki [March 2023 Agreement](https://codeberg.org/forgejo/meta/issues/181)
* https://codeberg.org/fnetX [April 2024 Agreement](https://codeberg.org/forgejo/governance/issues/101)

## User Interface

Purpose: Improve Forgejo's UI. Responsible for decision-making of frontend technology. Includes reviewing proposed changes to the UI and tending to (critical) UI bugs. Works closely together with the Accessibility, Contributors and Localization teams. The team is also responsible for Forgejo's design in collaboration with the Accessibility and User Research teams.

Team members:

* https://codeberg.org/Mai-Lapyst - [June 2024 Agreement](https://codeberg.org/forgejo/governance/issues/131)
* https://codeberg.org/0ko - [June 2024 Agreement](https://codeberg.org/forgejo/governance/issues/132)
* https://codeberg.org/caesar - [June 2024 Agreement](https://codeberg.org/forgejo/governance/issues/134)

## Helm

Purpose: Improve Forgejo Helm chart. Members have full write access to the Helm chart repo. They are responsible for improving the chart and reviewing / merging community PRs.

The team has access to the following Forgejo repositories:

* On code.forgejo.org: all repositories in the <https://code.forgejo.org/forgejo-helm> organization.

Team members:

* https://codeberg.org/viceice [June 2024 Agreement](https://codeberg.org/forgejo/governance/issues/145)
* https://codeberg.org/earl-warren [June 2024 Agreement](https://codeberg.org/forgejo/governance/issues/146)

## Devops

Purpose: The team cares of all the technical resources that Forgejo depends on (hardware, CI, static web site hosting, social media etc.). It helps all other teams to use those resources by installing, upgrading or migrating them when needed. If a resource becomes unavailable, it will help restore it in a functional state.

Accountability:

* Fix problems that prevent the resources that Forgejo depends on from running.
* Keep the [credentials to access the resources](https://forgejo.org/docs/next/developer/secrets/) in a safe place and share them with the teams that need them.

Team members:

* https://codeberg.org/crystal [April 2023 Agreement](https://codeberg.org/forgejo/governance/issues/18)
* https://codeberg.org/earl-warren [July 2024 Agreement](https://codeberg.org/forgejo/governance/issues/140)

## Contributors

Purpose: Improve Forgejo. Anyone can become a member of the team, as long as they need the associated permissions to contribute to Forgejo. Anyone can ask that an existing member confirms their membership in accordance to the [decision making process](DECISION-MAKING.md).

The team has access to most Forgejo repositories:

* On codeberg.org: ([discussions](https://codeberg.org/forgejo/discussions), [docs](https://codeberg.org/forgejo/docs), [forgejo](https://codeberg.org/forgejo/forgejo), [governance](https://codeberg.org/forgejo/governance), [pages](https://codeberg.org/forgejo/pages), [sustainability](https://codeberg.org/forgejo/sustainability), [test](https://codeberg.org/forgejo/test-env), [website](https://codeberg.org/forgejo/website)).
* On code.forgejo.org: all repositories in the https://code.forgejo.org/forgejo and https://code.forgejo.org/actions organizations.

The permissions of the team are:

| Unit | 	Permission |
| -- | -- |
| Code | 	Read |
| Issues | 	Write |
| Pull | Requests 	Write |
| Releases | 	Read |
| Wiki | 	Write |
| Projects | 	Write |
| Packages | 	Read |
| Actions | 	Write |

Team members:

* https://codeberg.org/0ko
* https://codeberg.org/algernon
* https://codeberg.org/dachary
* https://codeberg.org/DanielGibson
* https://codeberg.org/fluzz
* https://codeberg.org/gmem
* https://codeberg.org/JakobDev
* https://codeberg.org/jerger
* https://codeberg.org/KaKi87
* https://codeberg.org/maltejur
* https://codeberg.org/n0toose
* https://codeberg.org/patdyn
* https://codeberg.org/realaravinth
* https://codeberg.org/rome-user
* https://codeberg.org/snematoda
* https://codeberg.org/viceice
* https://codeberg.org/wetneb
* https://codeberg.org/Xinayder
* https://codeberg.org/zareck

## Mergers

Purpose: Review and merge pull requests in Forgejo repositories in accordance to the [pull request agreement](PullRequestsAgreement.md). The team is responsible for the same repositories as the contributors team.

Team members:

* https://codeberg.org/wetneb [January 2024 Agreement](https://codeberg.org/forgejo/governance/issues/54)
* https://codeberg.org/0ko [April 2024 Agreement](https://codeberg.org/forgejo/governance/issues/106)
* https://codeberg.org/viceice [August 2024 Agreement](https://codeberg.org/forgejo/governance/issues/156)
* Members of the Security, Devops & Releases teams

## Localization

Purpose: Manage the [Weblate localization](https://translate.codeberg.org/projects/forgejo/) project.

Accountability:

* Develop the software and workflows required for the translations to be available and updated in the Forgejo codebase.
* Document the localization process.
* Actively look for new translators to improve the quality and completeness of the project.

Admins accountability:

* Avoid destructive actions (such as resetting the weblate repository)
* Ensure the the weblate repository is in sync with the Forgejo repository
* Manage team assignments of members
* Block users performing destructive actions (such as vandalism or harassment in comments) and report these actions

Team members:

* Arabic https://codeberg.org/oatbiscuits - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/72) (admin)
* Brazilian Portuguese https://codeberg.org/rmorettibr - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/73)
* Brazilian Portuguese https://codeberg.org/Xinayder - [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/90)
* Chinese (Traditional) https://codeberg.org/leana8959 - [May 2024 Agreement](https://codeberg.org/forgejo/governance/issues/117)
* Czech https://codeberg.org/Fjuro - [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/97)
* Dutch https://codeberg.org/gusted - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/69) (admin)
* Esperanto https://codeberg.org/jadedctrl - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/77)
* Filipino https://codeberg.org/kita - [April 2024 Agreement](https://codeberg.org/forgejo/governance/issues/105)
* French https://codeberg.org/earl-warren - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/71) (admin)
* French https://codeberg.org/KaKi87 - [May 2024 Agreement](https://codeberg.org/forgejo/governance/issues/123)
* German https://codeberg.org/fnetX - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/85) (admin)
* German https://codeberg.org/nmmr - [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/86)
* German https://codeberg.org/Wuzzy - [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/93)
* Greek https://codeberg.org/n0toose - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/84)
* Italian https://codeberg.org/Zughy - [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/103)
* Japanese https://codeberg.org/ledyba - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/74)
* Korean https://codeberg.org/kdh8219 - [May 2024 Agreement](https://codeberg.org/forgejo/governance/issues/121)
* Russian https://codeberg.org/0ko - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/70) (admin)
* Russian https://codeberg.org/Werenter - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/76)
* Spanish https://codeberg.org/maletil - [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/88)
* Ukrainian https://codeberg.org/nykula - [February 2024 Agreement](https://codeberg.org/forgejo/governance/issues/75)

## User Research

Purpose: Conduct User Research in the context of Forgejo. Anyone can become a member of the team, as long as they need the associated permissions to contribute to work on the [User Research repository](https://codeberg.org/forgejo/user-research). Anyone can ask that an existing member confirms their membership in accordance to the [decision making process](DECISION-MAKING.md).

Team members:

* https://codeberg.org/ei8fdb
* https://codeberg.org/caesar
* https://codeberg.org/fnetX

## Releases

Purpose: [See the documentation](https://forgejo.org/docs/next/developer/release/). The team is trusted with the primary GPG key used to sign Forgejo releases.

Accountability:

* Publish Forgejo releases.

Team members:

* https://codeberg.org/crystal [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/80)
* https://codeberg.org/earl-warren [July 2024 Agreement](https://codeberg.org/forgejo/governance/issues/141)

## Security

Purpose: [See the documentation](https://forgejo.org/docs/next/developer/discussions/#security).

Accountability:

* Handle security vulnerabilities.

Team members:

* https://codeberg.org/algernon [April 2024 Agreement](https://codeberg.org/forgejo/governance/issues/116) (limited to case by case assistance)
* https://codeberg.org/Gusted [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/96)
* https://codeberg.org/fnetX [March 2024 Agreement](https://codeberg.org/forgejo/governance/issues/95)
* https://codeberg.org/earl-warren [November 2023 Agreement](https://codeberg.org/forgejo/governance/issues/41)

## Social account

Purpose: Reply to questions and publish news on https://floss.social/@forgejo

Accountability:

* Sign the toots that are not discussed before being published. If a toot is published by a Forgejo contributor without consultation with anyone, it must be signed with `~ @name` that links to the social account of the contributor.
* If a toot is agreed upon, in a public space, by other Forgejo contributors it does not need to be signed.
* Attach an alt text to images for accessibility.

Team members:

* All members of the release team.
* All members of the moderation team.

## Moderation

Purpose: [See the documentation](https://forgejo.org/docs/next/developer/COC/).

Accountability:

* Take action when a behavior in a Forgejo space goes against the Code of Conduct or the law.
* [Follow the moderation process](MODERATION-PROCESS.md) and publish auditable reports based on facts and logic.

Team members:

* https://codeberg.org/caesar ([Oct 2023 Agreement](https://codeberg.org/forgejo/governance/issues/35))

Enforcer (only for the purpose of enforcing moderation decisions):

* https://codeberg.org/crystal ([Feb 2024 Agreement](https://codeberg.org/forgejo/governance/issues/78))

Observer (access to team discussions and history of actions, without any moderation rights):

* https://codeberg.org/oliverpool ([Jan 2024 Agreement](https://codeberg.org/forgejo/governance/issues/55))


## Liberapay team members

Purpose: Receive a share of donations distributed via the [liberapay team](https://liberapay.com/forgejo).

Accountability:

* Manage the Forgejo liberapay team account, which was created by the Codeberg account (@fnetx has access)
* Decide on a fair distribution of the incoming funds among the team members

Team members:

* Fallback: Codeberg e.V. receives part of the share to make use of leftover budget (@fnetx can manage the account)
* @algernon ([July 2024 Agreement](https://codeberg.org/forgejo/governance/issues/151))
* @viceice ([August 2024 Agreement](https://codeberg.org/forgejo/governance/issues/157))
* [@meissa](https://codeberg.org/meissa) ([August 2024 Agreement](https://codeberg.org/forgejo/governance/issues/158))
    * @jerger
    * @patdyn


## Sustainability Team 

Purpose: The team focuses on concerted efforts to make Forgejo a durable endeavour. This includes taking a lead on fundraising, work out propositions on strategic planning, and coordinating related activities within the community. The team also monitors the distribution of funding, and acts as a mediator in the event of a disagreement.

Accountability: The administrative aspects of grant applications from start to finish.

Team members:

- [avobs](https://codeberg.org/avobs) ([September 2024 Agreement](https://codeberg.org/forgejo/governance/issues/179))


## [GitHub organisation](https://github.com/forgejo) owners as of 2024-01-09

This organisation is only use to prevent squatting. Its information must be kept up-to-date (website, forge and social links).

- [caesar](https://github.com/caesar)
- [crystal](https://github.com/crystalcommunication)
- [gapodo](https://github.com/gapodo)
- [gusted](https://github.com/Gusted)
- [oliverpool](https://github.com/oliverpool)

## [GitLab.com organisation](https://gitlab.com/forgejo) owners as of 2024-02-04

This organisation is only used to prevent squatting.

- [crystal](https://gitlab.com/crystalcommunication)
- [oliverpool](https://gitlab.com/oliverpool)
