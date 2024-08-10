#!/bin/bash

# Delete all the data related to Blaze
delete_repo() {
  echo "Deleting the 'Blaze' data..."
  sudo rm -rf /opt/Blaze
}

# Remove the systemd service
remove_service() {
  echo "Removing the 'blaze' 24/7 background service..."
  sudo systemctl stop blaze
  sudo systemctl disable blaze
  sudo rm -f /etc/systemd/system/blaze.service
  sudo systemctl daemon-reload
}

# Main script execution
delete_repo
remove_service

echo "Blaze has been uninstalled successfully"
