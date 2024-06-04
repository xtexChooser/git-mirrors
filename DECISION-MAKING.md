# Decision-Making Guide

Welcome to Forgejo's decision-making guide! It will guide you through **how to
make a decision together**.

It's not meant to be a rule you "must" follow! It's meant to be a **helpful
resource** to help us make collaborative decisions effectively.

How to use:

- If you know your way through decision-making and you're doing fine without
  reading this guide and everyone is happy, that's fine, it's not a requirement
  (perhaps you can [help improve this guide](#9-feedback) using your
  experience!)
- If you're curious or unsure about a certain decision, or about
  decision-making in Forgejo in general, try the
  [simplified process](#1-simplified-process)
- For decisions where there's challenge and it's not working, or big decisions
  where you'd like more detailed instructions, try the
  [detailed process](#3-detailed-process)

| Table | of                                                                        | Contents                                                                  |
|-------|:-------------------------------------------------------------------------:|:-------------------------------------------------------------------------:|
| 1     | [Simplified Process](#1-simplified-process)                               |                                                                           |
| 2     | [Principles](#2-principles)                                               |                                                                           |
| 3     | [Detailed Process](#3-detailed-process)                                   |                                                                           |
| 3.1   |                                                                           | [Who decides](#3-1-who-decides)                                           |
| 3.2   |                                                                           | [Preparation](#3-2-preparation)                                           |
| 3.3   |                                                                           | [Describing the decision question](#3-3-describing-the-decision-question) |
| 3.4   |                                                                           | [Choosing duration for input](#3-4-choosing-duration-for-input)           |
| 3.5   |                                                                           | [Gathering stakeholders](#3-5-gathering-stakeholders)                     |
| 3.6   |                                                                           | [Choosing which phase to start at](#3-6-choosing-which-phase-to-start-at) |
| 3.7   |                                                                           | [Picking a communication medium](#3-7-picking-a-communication-medium)     |
| 3.8   |                                                                           | [Criteria-gathering phase](#3-8-criteria-gathering-phase)                 |
| 3.9   |                                                                           | [Proposal-creation phase](#3-9-proposal-creation-phase)                   |
| 3.10  |                                                                           | [Integration phase](#3-10-integration-phase)                              |
| 4     | [Summary of decision description](#4-summary-of-decision-description)     |                                                                           |
| 5     | [Disagreement on a Closed Decision](#5-disagreement-on-a-closed-decision) |                                                                           |
| 6     | [When It Takes too Long](#6-when-it-takes-too-long)                       |                                                                           |
| 7     | [Onboarding and Mentorship](#7-onboarding-and-mentorship)                 |                                                                           |
| 8     | [Further Support](#8-further-support)                                     |                                                                           |
| 9     | [Feedback](#9-feedback)                                                   |                                                                           |

## (1) Simplified Process

- Who decides: Check in the
  [table](AGREEMENTS.md#Responsibility-distribution-table)
- Where to document decisions about how we do things:
  [`AGREEMENTS.md`](AGREEMENTS.md)
- Want support, mentorship, co-holding? Ask on Matrix <3
  - More info in the [Onboarding and Mentorship](#7-onboarding-and-mentorship)
    section
- Open an issue/PR on `forgejo` repo if it's about the codebase itself, on
  `meta` repo if it's about governance/processes

General process to use in most cases:

1. Phrase an open inclusive decision question, and your proposal
2. Ask for people's *approval* and *concerns*, making sure everyone who may
   care or be impacted by this decision is invited and included
3. If someone disapproves, try to hear **what's important to them** about the
   decision
4. Together, modify the proposal to address their concerns
5. Ask for approval on modified proposal
6. Document and announce the decision
7. Open new issue about implementing the decision, if relevant

Process for more complex or controversial decisions:

1. Phrase an open inclusive decision question
2. If the expected proposals on this decision are expected to be big, sensitive
   or otherwise non-trivial, e.g. a plan for a whole system, consider to start
   by gathering criteria, i.e. which qualities the chosen proposal will need to
   have
3. Invite people to make suggestions, ping people who have related
   knowledge/expertise to make sure they participate
4. Wait a few days for people make proposals
5. Ask people to make a thumb-up on each decision they're **willing to live
   with, comfortable with**
6. Wait (recommended: **2 weeks**) to give people time to do that, ping people
   you believe may be impacted by the decision to make sure they have a chance
   to participate
7. Pick the option that has the least opposition, ask the people who aren't
   willing to live with it:
    - What's your concern about this proposal, what's important to you that
      prevents you from feeling comfortable with this proposal?
    - What can support you (to be done or changed) to feel comfortable with
      this proposal?
8. Change the proposal if needed, based on the feedback
9. Ask everyone to react with thumb-up if they're willing to live with the
   modified proposal
10. Once there's consensus,
    - Document the decision in AGREEMENTS.md and announce in public channels if
      relevant
    - Document any followup tasks and who's going to do them and when
    - Eventually close the issue

Some tips and hints:

- Strong disagreement = very important concerns being present! Try to find them
- Conflict = opportunity to connect, grow and learn! Try to breathe and go more
  slowly
- Stuck in a challenging situation, conflict, disagreement? *Ask for help*

## (2) Principles

- **Willingness**: Instead of voting on the option you *prefer* (which leads to
  polarization and confusion, and makes it impossible to find consensus), you
  raise your hand/thumb on the options you're *comfortable with and willing to
  accept* (which supports us in identifying concerns, handling them, and
  picking proposals that really take us towards our goals and that we all agree
  on)
- **Concerns**:
    - When people are against a proposal, try to guess and look for *what's
      important to them*, which need/concern/value they're defending
    - So the key question is **What's important to you?**, it often helps to
      finish arguments and move forward together
- **Proposals**:
    - We seek to make a decision that is:
        - Good enough for now, safe enough to try (i.e. it doesn't need to be
          perfect)
        - Moving us towards our shared goal
    - So we think of concerns and oppositions as gifts expressing a potential
      reason that the current proposal:
        - Isn't good enough for the current circumstances
        - Isn't safe enough to try
        - Hinders or endangers our progress towards the shared goal
- **Togetherness**:
  - Working with willingness and concerns allows us to work together as a team,
    to make good decisions about Forgejo, rather than competing for our voice
    to be heard and for our personal proposals to be picked
  - Communication and decision-making can be challenging! We aim to hold each
    other with loving supporting care, even when we struggle or fail to execute
    the processes described in this guide (or any other process we've chosen)
- **Capacity and clarity**:
    - Only what we have energy to do and what we're willing to do gets done,
      both as individuals and as a community
    - We aim to make clear actionable practical agreements we're really capable
      of following. If we repeatedly struggle/fail to follow them, it's often a
      sign they're beyond our current capacity, and then we identify the
      difficulty and modify the agreements to be fully within what we're
      currently capable of.

## (3) Detailed Process

What can you do when you see a decision that needs to be made, or modified, or
something isn't working?

- If you just want to report about it and let other people handle it, open an
  issue (or a pull request if relevant) and you're done
- If you'd like to **facilitate** the decision-making process, keep reading
- If you're new here, and/or facilitating sounds scary, don't hesitate to ask
  for support/mentorship/co-holding! On Matrix or in the issue/PR

### (3.1) Who decides

Look at the *Responsibility Distribution Table* in the *Decision Making*
section in [`AGREEMENTS.md`](AGREEMENTS.md). Based on the topic/domain of your
decision, **is there a team/person who's been granted the power to make
decisions in this topic?**

- If yes, open an issue/PR, asking that team to take over and lead the decision
  (you can assign the issue/PR to the team's members, they're listed in
  [`TEAMS.md`](TEAMS.md)). Done.
- If the decision's topic isn't entrusted to a specific team/person, you can
  facilitate the decision-making process! By following the steps below
- If you don't want the responsibility/work of facilitating the process, just
  open the issue and someone else will pick it up

### (3.2) Preparation

1. **Issue or PR?**
  - If you have a specific proposal, you can open a PR that implements
    it. If it's about changing the organization's processes, you'll
    probably want to add/edit an agreement in
    [`AGREEMENTS.md`](AGREEMENTS.md). Look at its *Decision Making*
    section for info about other relevant files.
  - Otherwise, the default is to open an issue
    - If it's a technical topic the Forgejo software itself, open the
      issue in the Forgejo repo
    - Otherwise, open the issue in the meta repo
2. **Assign yourself** to the issue/PR
3. Would you like **mentorship/support/co-holding** in facilitating the
   process?  If yes, mention that in the issue/PR description. You can also ask
   on Matrix, and if someone offers to mentor/support you, you can assign them
   to the issue/PR as well.
    - More info in the
      [Onboarding and Mentorship](#7-onboarding-and-mentorship) section

### (3.3) Describing the decision question

1. In the issue/PR description, phrase the decision question (also known as the
   *shared motive*).
   Guidelines and tips:
    - Use an inclusive and integrative open question, rather than either/or
      language
    - Refer to what we *do* want, rather than what we don't want
    - Refer to the core need or shared goal that the decision aims to address
    - Use practical language rather than ideological language
   Suggested format:
    1. Summary of the **current situation**, in a few sentences or bullet items
       (example: *Spam bots are filling people's inboxes with random useless
       messages*)
    2. If needed, paragraphs/links/attachments explaining the situation in more
       detail
    3. One sentence asking the **decision question** (example: *How do we make
       sure people receive only useful and relevant information?*)
2. Impact and timeline: If the decision is about a critical or urgent
   situation:
    - Describe the potential danger/effect/impact of not handling the
      situation
    - Suggest a timeline for reaching a decision
    - Specify what we do if we can't reach a decision in time (e.g. delegate
      the decision to a few specific people to make a fast but careful decision
      on behalf of the community)
3. After the decision question (and impact), if you intend to suggest a
   specific **proposal**(s), describe your proposal(s)
4. Phrase the issue/PR **title**:
  - If you have a single clear candidate non-controversial proposal, a one-line
    summary of the proposal can serve as the title
  - Otherwise, use a short concise essence of the current problematic situation
    and/or the decision question

### (3.4) Choosing duration for input

As a globally distributed community project, lot of our communication is
asynchronous. This means that in each step of the process you'll be making a
request, and allowing time for people to react and send their input. Pick
sufficient time given:

- People's availability
- The decision's urgency
- The decision's complexity
- The impact of making the decision
- The impact of taking too much time to decide

Especially when time is constrained, make sure at least the following people
react:

- The people most *impacted*
- The people with relevant *expertise*
- The people with *strong opinions*
- The *outliers* who hold controversial or unusual perspectives
- The people who hold required *resources*

Suggestion for time to wait for people to send input, especially for decisions
involving the whole organization and/or wider community:

- Safe default: 2 weeks
- To move faster with non-risky decisions: 1 week
- For urgent decisions, perhaps less than a week

### (3.5) Gathering stakeholders

Gather, notify, @mention, invite and call for the relevant people (a.k.a.
stakeholders) to participate:

- If a specific team is entrusted by the organization to make this decision,
  gather the team
- Otherwise, invite whole organization

Also, in particular make sure the following people (whether in or out of the
Forgejo community) are included, or at least gather their advice and consent:

1. The people affected/impacted by the decision
2. Who are the people with expertise on the matter
3. Who are the people who have/hold/provide/maintain the resources needed for
   implementing the decision
4. Do you need advice from outside the Forgejo community? If yes, ask for this
   advice (consider using Matrix and the Fediverse to spread your request for
   advice)
5. Any people who can oppose the results, or stop the process of implementation
6. Relevant Forgejo teams

For high-impact or controversial decisions, listing these people in the issue
description can support to create a sense of trust, inclusion and togetherness
around the shared problem.

### (3.6) Choosing which phase to start at

- If you already have a clear proposal, especially if the decision's
  scope/impact are small, start from the **integration** phase
- If you don't have a clear proposal, or you believe it would be beneficial to
  creativity/trust to invite more proposals, start from the
  **proposal-creation** phase
- For a major or complex or controversial decision, or a decision with
  many/major impacts, or a decision for which creating a sufficient proposal
  would be complex or non-trivial, start from the **criteria-gathering** phase

### (3.7) Picking a communication medium

- Each phase (or all phases) can be done via Issue/PR comments, or Matrix, or
  Jitsi, or possibly something else
- Multi-step mutually-influential decisions may be easier to do in a real-time
  meeting (e.g. a chat meeting on Matrix or audio/video on Jitsi or combined
  text and video)
- However, for a global community of volunteers who speak different languages
  and have different timezones and privacy concerns, and who don't have regular
  organization meetings, gathering everyone to make a decision might be
  difficult or impossible
- **Suggested default**:
  - For very controversial or sensitive or very big decisions where you're
    confident that discusion via issue comments would result with chaos and
    conflict, schedule a real-time meeting (Matrix/Jitsi)
  - Otherwise, **start the discussion via issue comments**, and only switch to
    a real-time meeting (while also inviting input, feedback and advice from
    people who can't attend) if the discussion via issue comments isn't working
- Not sure? Discuss via Matrix and ask for advice/support

### (3.8) Criteria-gathering phase

1. Invite people to comment about what's important to them about this decision
    - People can also comment on someone else's criteria if they have a
      concern/disagreement about it
    - Alternative phrasing for invitation: Which questions do we need to ask,
      in order to make a decision that serves our shared goal
2. Allow time for discussion to happen
3. Gather a single unified concise list of criteria (or questions) from
   people's comments, where each item:
    - Is in terms of what we *do* want rather than what we *don't* want
    - Goes deep enough in the direction of a value/need to be non-controversial
      (with respect to the other criteria people have raised), but not more,
      i.e. as specific as possible while as abstract as needed to reach across
      controversy
    - Is inclusive, includes everyone, is beyond specific people/situation
4. Invite people to look at the list and comment if something is missing or if
   there's something they disagree with, or react with **thumb-up** if they're
   okay with moving forward
5. Allow time for people to comment
6. Edit your list, adding/modifying criteria based on the new comments
7. Invite the people who commented to react on the list with **thumb-up** if
   the modification is satisfactory for them, and to comment if the list still
   isn't capturing their criteria. And invite everyone to comment if they
   agreed to the original list but now have a concern about the modifications
8. Wait for people to react and comment
9. Are there still concerns or disagreements about the criteria list? If yes,
   switch to talking with the concerned people via Matrix/Jitsi, or ask on
   Matrix for help from another community member with integrating the criteria
10. Once there's agreement on the list, move to proposal-creation phase, and
    edit the issue description to have a link to the comment that contains the
    criteria list, for easy reference

### (3.9) Proposal-creation phase

- Define one or more small teams, where each team works on a proposal
- Team size can be 2-3 people for simpler cases, 5-6 for big or controversial
  decisions
- Aim to pick the team(s) such that it holds the disagreement/controversy
  within it, i.e. people with different perspectives, specifically the people
  who are most struggling to agree with each other
- Aim to include in the team(s) the people who have relevant expertise and
  resources, or at least ask/invite the team to consult with those people
- A good proposal:
  * Is detailed and very specific
  * Handles all open loops, or specifies when and how they'll be handled
  * Specifies who's responsible for what
  * Specifies the time at which we'll review whether the decision is working,
    and adapt it as needed
  * Specifies criteria for assessing whether and in which ways the decision is
    working
- Each team can discuss their proposal using a separate issue, or on Matrix, or
  via Jitsi, or something else that works for them
- Once a team has a proposal ready, they make a *single comment* on the issue,
  that presents the proposal and begins with the text *PROPOSAL*
- Once proposal(s) are ready, move to integration phase
- If proposals struggle to be relevant, consider moving back to
  criteria-gathering phase

### (3.10) Integration phase

1. Pick a threshold for concerns
  - Safe default: "If you're (not) okay with the proposal"
  - If you think people may struggle/fear to bring up their concerns, or
    there's a lot of space and time, use a lower threshold, e.g. "If you have
    *any* concern about the proposal"
  - If it's an urgent decision or people are exhausted, use a higher threshold,
    e.g. "If you really can't live with this proposal"
2. Invite people to express their stance about all the proposals, using the
   threshold you picked:
  - If you're okay with the proposal, make a **thumb-up** reaction on it
  - If you're okay with the proposal, but you have some note/info/point/worry
    to express to the group, make a **thumb-up** reaction
  - If you have a concern about this proposal, i.e. you see a reason that:
    - It isn't serving/beneficial to our shared goal, or
    - It's not good enough for now / for the timeframe / context / situation,
      or
    - It's not safe enough to try
    Then make an **eyes** reaction
  - **Note**: If there's only one proposal, combine steps 2-5 into a single
    step, asking people to make a reaction *and* make a note/concern comment
3. Allow time for people to make reactions
4. Pick the proposal that has the most willingness / least concern
5. Invite people to comment about their stance on this proposal:
  - If you made a thumb-up reaction but had a note to share, write your note in
    a comment, starting it with **NOTE:**
  - If you made an eyes reaction, describe your concern in a comment, starting
    it with **CONCERN:**, and optionally suggest a modification to the proposal
    and/or relevant support, that would allow you to accept the proposal
6. Allow time for people to comment (make sure everyone who reacted with *eyes*
   expresses their concern)
7. If there are concerns, ask the people who raised them, or the people who
   created the proposal, or the whole group, to offer a modification to the
   proposal and/or relevant support, and check with the person who expressed a
   concern whether the suggestion works for them, while inviting concerns from
   the group about the suggestion
8. If reaching agreement on the proposal is difficult and taking a lot of time,
   consider switching to the next best proposal, jumping to step 5. If there
   are no proposals left to try, or all proposals seem far from agreement, send
   people back to another round of working on the proposals
9. If reaching agreement is taking too long and/or the group is exhausted, pick
   a small team (that includes those who created the proposal and those holding
   concerns) that will create a modified proposal based on the concerns and
   will either bring it to the group to make a final approval (if there's
   enough time) or will make the decision on behalf of the group (if the
   decision is too urgent/exhausting to wait for the whole group to approve)
10. Once all concerns have been integrated:
  - Announce the final decision, by adding a **final comment** on the issue/PR
  - Make it clear if you amend a previous decision taken by you or someone else
  - Attend to any NOTEs that need action
  - Document the decision (e.g. as an agreement) if needed
  - Close the issue
  - If the decision now needs to be implemented, open a *new* issue/PR and
    assign it to the people who will lead the implementation.
  - If it's a major or critical decision, consider to announce it using
    Matrix/Fediverse/blog

Make sure the final decision includes:

1. Who's affected by the decision?
2. Who can take an action on the decision?
3. What steps are needed to take action and who will take them?
4. Who or which team owns/oversees the implementation process?
5. All of this is communicated via the issue/PR description and relevant
    team/organization/community members are @mentioned

After that, please **continue to follow** through with any additional steps
that are needed to implement the decision.

- Watch for adoption. Adoption levels are a form of feedback, and perhaps this
  will inform updates to the decision.
- This doesn't mean that you need to do everything on your own. If you want
  support, please ask via Matrix.

## (4) Summary of decision description

Summary of information to include in your issue:

- Assign yourself and anyone else who has agreed to co-hold the decision with
  you
- Description
  - Request for mentorship/support, if you want support but didn't find it on
    Matrix
  - Current situation that requires a decision
  - Decision question
  - For critical or urgent decisions:
    - Impact of not handling the situation
    - Timeline for the process and for reaching a decision
    - Emergency strategy if we can't decide within the timeline, usually pick a
      few people who will decide together in behalf of the community
  - For high-impact or controversial decisions, make lists of stakeholders,
    i.e. people who are:
      - Affected
      - With expertise
      - Who hold the resources
      - Outside the Forgejo community
      - Who can stop/oppose the process/results
      - Relevant teams
  - Specific request from people to react
  - As the process progresses, paste links to special comments for easy
    reference and tracking:
      - Criteria list
      - Proposal(s)
- Title: Concise essence of the situation/question
- Label: Keep updating it to one of the *[Decision]* labels, based on the
  current phase of the process

## (5) Disagreement On A Closed Decision

What if someone has made a decision that I don't agree with (regardless if I
was invited for advice/input/approval)?

- Once a decision is made, it's active. If you disagree with a decision, you're
  welcome to:
    - Give feedback to the person/team who made the decision, via
      Matrix/Fediverse/Email
    - If you believe the decision that's been made is critically harmful to the
      project, and the person/team who made the decision is unavailable, open a
      new decision
- If you've been excluded from a decision and that's painful to you, or there's
  a conflict about a decision, and it affects your sense of trust and safety in
  the community, please reach out to the
  [well-being team](TEAMS.md#well-being), or to the community on Matrix
- As a community, we commit to the final decision (even if we didn't take part
  in the decision and/or the decision isn't in alignment with our preferences)
- If there's a high level of disagreement, and you've used the simplified
  process, consider trying the [detailed process](#3-detailed-process)
  instructions to revisit the decision (ask for help with that if needed),
  and/or use a Matrix/Jitsi real-time meeting

## (6) When It Takes Too Long

As a community project, we want to integrate both:

- Care, inclusion, togetherness, holding the project's community and values
- Efficienct work, progress, flow, holding the project's purpose

Making decisions collaboratively in a group, seeking a solution that works for
everyone, can sometimes take more time than we have or that we wish, or require
skill or practice we don't yet have, both from participants and facilitators,
especially when there's controversy or high level of disagreement. Some reasons
that decisions take a long time are:

- Many people discussing an urgent decision, creating a very long discussion
  and integration process
- It takes time to practice and build skill to efficiently navigate complex
  decisions and to communicate effectively, both for participants and
  facilitators
- Care for purpose in a complex situation can be challenging! For example, a
  low-trust situation can lead us to defend our chance for self-expression,
  even when it's slowing down a very urgent/exhausting decision
- We all carry emotional pains, wounds and triggers about inclusion,
  collaboration and communication, and we live in a world and culture that
  creates these pains and doesn't support us in healing them
- We're a global community of volunteers from different cultures, timezones,
  skills and languages
- Unexpected problems naturally happen, requiring urgent action and there's no
  time for a long discussion

While accepting our individual and collective limitations with love and care,
and while accepting the uncerainty and unpredictability of life, **what can we
do when a decision is taking too long?** How can we be both inclusive and
efficient?

When starting a very low-impact or urgent decision:

- Pick a time frame for reaching a decision, and pick an alternative strategy
  for the case we don't reach decision in time (suggested strategy: Delegate
  the decision to a small group of trusted people, to make a decision on behalf
  of the community)
- Raise the threshold for concerns, for example switch from "does anyone have a
  concern about this" to "is there anyone who really can't live with this"
- Consider to focus on gathering concerns and willingness from a small (but
  varied) group of stakeholders representing the various considerations of the
  community, rather than a broad focus on everyone
- For very low-impact decisions, is there anyone who would particularly feel
  excluded if you moved forward without their input? How's the level of trust
  in the team/organization/community? Consider to just make a decision yourself
  and move forward, while announcing your decision and inviting feedback
- For very urgent decisions, consider going directly to Matrix/Jitsi to decide
  with who'se available, and when the urgent problem is attended, announce what
  happened and why

During a decision-making process:

- As a participant:
  - If your messages expressing concerns are long, try to shorten them by
    identifying/distilling the essence, or at least *mark* the essence inside
    the long message
  - Is there someone you trust, who's going to participate in the decision and
    can represent your concerns? If so, consider to ask them to represent you,
    and then not participate yourself, to help reduce the length of discussion
- As a facilitator:
  - Raise the threshold for concerns, for example switch from "is anyone
    uncomfortable with this" to "does anyone see something really important
    that we'd miss if we proceeded with this"
  - Delegate the decision to a small group of trusted people, to make a fast
    and caring decision on behalf of the community

Evolving our structures and processes:

- In the
  [responsibility distribution table](AGREEMENTS.md#responsibility-distribution-table),
  entrust more decisions and topics to specific teams and roles
- Within a team, if there's enough trust (and consent), consider making use of
  the "Advice Process", i.e. allow any team member to gather advice on a
  decision and make the final decision themselves without needing
  approval/consent
- Consider defining a central circle/team of project stewards that makes
  wide-scope decisions (perhaps only if the regular community process is too
  long/difficult/urgent)
- Look into creative ways to increase trust, e.g. social activities where we
  get to know each other more, or pair programming sessions, etc.

## (7) Onboarding And Mentorship

How do we nourish and evolve our decision-making skills, both as individuals
and collectively within the project and the community? How do we pass on these
skills, creating an inclusive space and project where people can gradually step
into leadership and get involved?

If you're new here, and/or unsure about facilitating a decision-making process:

- Practice on low-risk, non-urgent and small decisions
- Ask, on Matrix or in issues, for mentorship or co-holding from another
  community member
  - Facilitate a decision, with a mentor being there to support
  - One of you or both of you facilitate, while discussing on Matrix (publicly
    or privately) and deciding together on the next steps

Asking for help, support and mentoring is very welcome and encouraged!

If you're more experienced/confident and have capacity and willingness to
support others:

- Watch for support requests on Matrix and in issues
- Watch for challenging decision processes that might need/enjoy support
- If you see an issue/discussion struggling with a decision, step in to
  facilitate (if nobody is) or gently offer support (if there's already an
  assigned facilitator), possibly via Matrix to avoid disrupting the on-topic
  flow of discussion in a busy issue
- Watch for potential committed contributors who might want to make a further
  step into the project (and learn precious skills and increase the bus
  factor), and gently offer support/mentoring for decision-making

Also see list of learning resources in the next section.

## (8) Further Support

Having a challenge in making a decision, in reaching agreement, or in choosing
the process for some big decision? Contact the
[decision-making advocates](TEAMS.md#decision-making) and ask for support.

Resources for learning, practice and external support for decision-making
processes and systems:

- [Convergent Facilitation](https://convergentfacilitation.org) a.k.a CF (includes book, course recordings, live courses, practice groups and more)
- [Decision-making systems learning packet](https://thefearlessheart.org/item/decision-making-systems-from-either-or-to-integration-packet)
- [Vision Mobilization](https://visionmobilization.org) (in particular the part about agreements)
- [Sociocracy](https://sociocracyforall.org)

## (9) Feedback

Have feedback/improvements for this guide? Open an issue/PR with the
*Governance* label.

Have feedback about decisions in Forgejo? What's working, what's not working?
Open an issue with the *Governance* label and/or come to the biweekly
governance meetings where we evolve and improve our processes.
