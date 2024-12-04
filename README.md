# ai_tcr_rust
an ai tcr rust template

## Setup
### Requirements
- commit_message in path from https://github.com/vextorspace/aiCommit/releases
- ai_coder in path from https://github.com/vextorspace/aiCoder/releases
- watchexec in path from https://github.com/watchexec/watchexec
- windows will need to change from .sh to batch files and of course syntax will need to change
### Operation
- run ./start.sh
- add a failing test
- save the file
- it should detect this, run the unit tests, and make them pass (hopefully).
  - if it fails, it will reset you to the last working copy.
  - I would then try to write a smaller test. Or give it a better name next time. Give your variables good names. See what happens!


