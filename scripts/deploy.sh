#!/bin/bash

# Change the ownership of /opt/Blaze to the current user
change_ownership() {
  echo "Updating the ownership of Blaze directory to the Current user..."

  # Get the current username
  current_user=$(whoami)
  sudo chown -R $current_user:$current_user /opt/Blaze
}

# Build the Rust project
build_project() {
  echo "Building Blaze..."
  cd /opt/Blaze || { echo "Failed to enter '/opt/Blaze' directory. Check if the 'Blaze' directory exist on the '/opt' directory"; exit 1; }
  cargo build --release
}

# Update and move the service file
setup_service() {
  echo "Setting up Blaze to run 24/7 on background..."

  # Replace 'User=root' with the current user's name
  sed -e "s|User=root|User=${current_user}|g" \
      blaze.service > /tmp/blaze.service

  # Move the modified service file to the systemd directory and setup systemctl
  sudo mv /tmp/blaze.service /etc/systemd/system/
  sudo chown root:root /etc/systemd/system/blaze.service
  sudo chmod 644 /etc/systemd/system/blaze.service
  sudo systemctl daemon-reload
  sudo systemctl enable blaze
  sudo systemctl start blaze
}

# Check the service status
check_status() {
  echo "Checking the current status of the Blaze..."
  sudo systemctl status blaze
}

# Main script execution
change_ownership
build_project
setup_service
check_status

echo "Blaze has been deployed successfully! Run 'sudo curl -fsSL https://raw.githubusercontent.com/ShadowArcanist/Blaze/master/scripts/test.sh | bash' to simulate high resource for 15 seconds to test Blaze (this is optional)."
