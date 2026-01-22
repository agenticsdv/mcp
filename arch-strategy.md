# Architectural Considerations & Placing Strategies
- Hosting tools, prupose wise
  - Same server hosting tools of different purpose, e.g. SOME-IP, CAN
  - Different servers hosting tools of single purpose
- Where to host server
  - Inside Telematics ECU, for internal & external access as a gateway
  - Inside Infotainment ECU, for internal access
  - Inside Domain Gateway (In case of Domain Oriented EE Architecture)
  - Inside Central Compute/HPC Node (In case of Zonal EE Architecture)
  - One MCP server per Zonal Gateway
