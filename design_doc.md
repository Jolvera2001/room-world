# Design Graphs

```mermaid
---
title: Broad Overview
---
flowchart RL

subgraph Plugins
    direction LR
    Player
    GUI
    subgraph mechanics
        Interaction
        Dialog
    end
end

subgraph App
    direction LR
    EntryPoint[App Setup]
end
Plugins --> |Registered In| EntryPoint
```

```mermaid
---
title: Interaction Events
---
flowchart TD

subgraph Interaction Flow
    P[Player]
    DP[Dialog Plugin]
    GUI[GUI Plugin]
    subgraph Events
        IE[Interaction Events]
        DE[Dialog Event]
        DoE[Door Event]
        ItE[Item Event]
    end
end

P --> |Keypress Emits| IE 

IE --> DE
IE --> DoE
IE --> ItE

DE --> |Consumed by| DP

DP --> |Emits Event & Data Consumed By| GUI
```