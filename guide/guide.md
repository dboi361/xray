# About Xray

# About this guide

# Foundational priorities

Our goal is to build an approachable, attractive, cross-platform text editor that is designed from the beginning around the following foundational priorities:

## High performance

*Xray feels lightweight and responsive.*

We design our features to be responsive from the beginning. We reliably provide visual feedback within the latency windows suggested by the [RAIL performance model](https://developers.google.com/web/fundamentals/performance/rail). For all interactions, we shoot for the following targets on the hardware of our median user:

| Duration | Action |    
| - | - |
| 8ms | Scrolling, animations, and fine-grained interactions such as typing or cursor movement. |
| 50ms | Coarse-grained interactions such as opening a file or initiating a search. If we can't complete the action within this window, we should show a progress bar. |
| 150ms | Opening an application window. |

We are careful to maximize throughput of batch operations such as project-wide search. Memory consumption is kept within a low constant factor of the size of the project and open buffer set, but we trade memory for speed and extensibility so long as memory requirements are reasonable.

## Collaboration

*Xray makes it as easy to code together as it is to code alone.*

We design features for collaborative use from the beginning. Editors and other relevant UI elements are designed to be occupied by multiple users. Interactions with the file system and other resources such as subprocesses are abstracted to work over network connections.

## Extensibility

*Xray gives developers control over their own tools.*

We expose convenient and powerful APIs to enable users to add non-trivial functionality to the application. We balance the power of our APIs with the ability to ensure the responsiveness, stability, and security of the application as a whole. We avoid leaking implementation details and use versioning where possible to enable a sustained rapid development without destabilizing the package ecosystem.

# Process
