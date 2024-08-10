#!/bin/bash

# Install necessary packages
install_packages() {
  echo "Updating package list and installing necessary packages..."
  sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev build-essential git systemd
}

# Clone the Git repository
clone_repo() {
  echo "Cloning the ShadowArcanist's Blaze git repository..."
  sudo git clone https://github.com/ShadowArcanist/Blaze.git /opt/Blaze
  cd /opt/Blaze || { echo "Failed to clone ShadowArcanist's Blaze git repository"; exit 1; }
}

# Main script execution
install_packages
clone_repo

echo "Initial setup is completed. Please edit '/opt/Blaze/src/main.rs' to configure the script (add webhook url, alert roles and adjust alert thresholds) and then Run 'sudo curl -fsSL https://raw.githubusercontent.com/ShadowArcanist/Blaze/master/scripts/deploy.sh | bash' to deploy Blaze"
