# Rust Workshop (June 2023)

To get started:

1. Install Docker and make sure it's running
2. Clone the repository
   ```sh
   git clone git@github.com:jfrere/rs-demo.git
   # or
   git clone https://github.com/jfrere/rs-demo.git
   ```
3. Open the code in VSCode.  (Other editors are available, but VSCode can understand the `.devcontainer` stuff.)
4. A dialog should appear `Folder contains a Dev Container configuration file.`  Click on "Reopen in Container".
5. The project should be reopened in a container with all the required Rust tools/plugins/etc installed.  To check this, run the following in a terminal inside VSCode:
  ```sh
  cargo --version  # should be e.g. 1.68.2
  cargo build      # should download all the necessary crates and build the whole project
