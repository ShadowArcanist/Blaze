#!/bin/bash

# Delete the cloned repository
delete_repo() {
  echo "Deleting the 'blaze' repository..."
  cd ~ || { echo "Failed to return to home directory."; exit 1; }
  sudo rm -rf Blaze
}

# Remove the systemd service
remove_service() {
  echo "Removing the 'blaze' service..."
  sudo systemctl stop blaze
  sudo systemctl disable blaze
  sudo rm -f /etc/systemd/system/blaze.service
  sudo systemctl daemon-reload
}

# Main script execution
remove_packages
delete_repo
remove_service
remove_testing_tools

echo "Blaze has been uninstalled Successfully"
