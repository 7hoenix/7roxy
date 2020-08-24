# Proxy

## An open source Artificial Intelligence (AI) Agent

- Incoming / Outgoing data communication channels.
  - Type level (AST) commands
  - Audio
  - Textual
- Reacts to data that is received.

Outer loop:
- Learns to improve over time.

Improvement in this context:
- User can load preferences.

## Domains (modules / interfaces):

### Capabilities

- What actions is Proxy capable of taking.

Hook for when we get to interface with other Agents later.

### Policies

- What responses is Proxy authorized to take for a given scenario.
- Delegation of authority and credentials.

### Preferences

Users have preferences (stuff like):

- Would prefer to meet soonest. (weight 3)
- Would prefer to optimize longest time frame possible. (weight 2)
- Would prefer to find best fit of time frame so that my time is chunked. (weight 2)

These preferences are taken into consideration based on this precendence order:

1. Per request.
2. Per person.
3. User Default.
4. System Default.

Preferences help drive Policies as customization points.

### Directives

Users issue directives to their 7roxy.

### Requests

7roxy instances issue requests to other agents to fullfill directives.

### Negotiation between agents

Trusted Negotion Policy

Each Constraint can have a trust level associated with it. When negotiating the trust
level will filter which Constraints are passed to the other agent.

(More passed. . . the closer the set of constraints is to matching.)

ask :: [Constraint] -> Query -> Response
ask otherAgentConstraints =
    ourConstraints <- findOurConstraints
    findBest (ourConstraints ++ otherAgentConstraints)

Timeslots returned:
Z: 10 3 5
J: 11 5 7 10

Negotiation rounds:

First round:
Z: [10]
J: [11]
No match

Second round:
Z: [10, 3]
J: [11, 5]
No match

Third round:
Z: [10, 3, 5]
J: [11, 5, 7]
5 Matches. :handshake: we have an accord.

--

## Usecase examples

### Schedule pairing Agent - Agent

Prerequisites:
-> Permission to see and update calendars.
-> Permission to manage segments of time (granted to human).

--> Default setting is to respond to requests (queries) with suggestions that then require signoff.

Level 2 or 3:
---> User sets priorities (for instance, schedule pairing with Zephyr once per month).

IDEA:
---> Potential idea is to allow auto approval of suggestions (for certain policies).

### Scheduling (balance) assistant -- time budget.

Do X amount of work on this goal xs
Do Y amount of work on this goal ys

How you want your schedule to be arranged:
- When for each domain:
  - Physical
  - Mental
  - etc.
- When are you at your best for each of those?
- How much ramp up do you need to chunk into your short term memory to be effective at the work.

Metrics / Improvment measured by:
- A / B .. / N testing of strategies (strategies are permutations of blocks that *could* be effective).
  - Quality of work (as reported by human).
  - Feeling around sustainable pace, enjoyment of work, etc (as reported by human).

### Personal history repository

- Job history --> Prompt for blurb about completed projects as scheduling to populate portions of this.
- Residence history
- Doctors

### Scheduled reports

- Important scheduled events.
  - Reminders of upcoming deadlines.
- Anything important in the world.
  - How many news outlets are carrying the same story?
    - Extract sets of words across different social media bubbles.
    - Display similar framing
    - Display disparate framing
- Progress report on how Agent (and associates) are doing on longer term assignments
  - Graph of actions taken on your behalf in relation to an established policy.
  - Any blockers communicating with other Agents
  - Anything that needs human intervention <- Can't get something scheduled
- Aggregate metrics

### Outstanding requests that it attempts to complete periodically

If AgentZ makes request but AgentJ is down. . . what happens?
  - Store requests in a per-agent queue and keep trying periodically
      - As part of persistant peer to peer connection protocol

AgentZ as part of fulfilling requests to pair, first needs to connect to AgentJ.
This happens with an exponential backoff per-agent. Any further requests for AgentJ
are queued until a connection to AgentJ is established. No connection is needed for
other agents UNLESS there are requests to be fulfilled by that agent. i.e. the first
request to AgentJ intiatates AgentZ to try and connect to AgentJ over a TCP socket.

### Cross reference StackOverflow and Github for tasks

- User inputs phrase to search for.
- 7roxy searches StackOverflow.
  - Sort by most recent & highest rated.
  - Examine questions & answers for common terminology that is not in
    query.
    - ? 7roxy expands searches to find variants.
  - 7roxy checks back in with User.
  - User selects candidate(s) for 7roxy to pursue.
  - 7roxy takes code snippets from candidate(s) and searches github
    api.
  - 7roxy reports results (with links).
  - Perform semantic anlysis on code to remove details specific to
    that example.
    - What if 7roxy just had a grammar that it understood? Like Bash.

--

API's are super specific.

What process does the program need to do in order to resolve?

It does its own analyis ()
Pick candidates out that maps "user input phrases -> things we are searching for"
f.g("what is the shebang in bash") -> [so result with example, github code snippet]
f("what is the shebang in bash"): value estimate, output search terms for github (eg. code snippets), relevant code/text snippets
g(output search terms for github, value estimate): value estimate, relevant code snippets

--

## Persistent Peer to Peer Address Book Protocol

- Agents ping each other regularly to verify connection (once per hour - once per day).
  - Agents are run as daemons that reset on restart.
  - Agents Address Book of known Agents is stored locally.
  - If AgentZ receives 3 failing checks from AgentJ.
    - Stop sending pings.
  - If AgentZ gets new ping from unknown ip (with shared secret) then it will assume that this is AgentJ.
    - Update IP address for AgentJ in Address Book.
    - Resume sending pings to new address.

--

## Security

### Authorized Agent key exchange

- Verified identities of other Agents
- Keybase (ish) implementation of socially signed linked accounts.
  - Github to start.

### Registry

- Tiered registrys?

### Operational level security concerns

- This needs to be addressed.

--

## SPIKE -- Recommend next pairing session Agent - Agent

- Shared secret password exchanged through communication side channel (hardcoded)
- Verification done by sending hashed password over tls.
- Persistent Peer to Peer communication protocol implemented.
- Agent has Permission to read calendar.
- Agent is able to distinguish managed segments of time (granted by human).
  - Human puts down time slots during week that are managed by agent.
- Implement Policy for minimum time duration of pairing session (pairing for 30 minutes is not enough).

Full Use case

- AgentZ and AgentJ are both running on work stations.
- Zephyr instructs AgentZ to schedule a pairing session with Justin within the next month.
  - 7roxy schedule --type pairing --with "Justin Holzmann" --within "one month"
- AgentZ checks Zephyrs calendar for managed slots within specified time period.
  - If no slots over policy minimum within time period are found. . . then ReportResult.
    - "Moar time slots please."
- AgentZ requests negotiation with AgentJ.
  - If AgentJ disagrees, then ReportResult.
    - "Justin is not available during this time. Or perhaps forgot to add timeslots."
For each agent in {AgentZ, AgentJ}:
- @agent selects a Negotiation Policy based on relationship with otherAgent.
  - Based on the trust level with otherAgent, @agent selects a set of Constraints to share.
  - @agent runs function to find ordered list of candidates (by preference) across both sets of Constraints.
- Perform Negotation rounds to find best.
- ReportResult
ReportResult:
  For each (agent, user) in {(AgentZ,Zephyr), (AgentJ,Justin)}:
    - @agent tells @user via desktop notification to check Agent information.
    - @user runs CLI command for agent information.
    - Agent information includes entire interaction information between agents:
      [ Request to negotiate to find a time to pair.
      , Negotation agreed to in time duration requested.
      , Exchange constraints.
      , Perform negotiations (rounds).
      , Report result.
      ]
