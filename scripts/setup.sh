#!/bin/bash

# Install necessary packages
install_packages() {
  echo "Updating package list and installing necessary packages..."
  sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev build-essential git systemd
}

# Clone the Git repository
clone_repo() {
  echo "Cloning the Git repository..."
  git clone https://github.com/ShadowArcanist/Blaze.git
  cd Blaze || { echo "Failed to enter 'blaze' directory."; exit 1; }
}

# Main script execution
install_packages
create_user
clone_repo

echo "Initial setup is completed. Please edit 'main.rs' to configure the script and then Run 'sudo curl -fsSL https://raw.githubusercontent.com/ShadowArcanist/Blaze/master/scripts/deploy.sh | bash' to deploy Blaze"
