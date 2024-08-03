#!/bin/bash

# Build the Rust project
build_project() {
  echo "Building the Rust project..."
  cargo build --release
}

# Move the service file and reload systemd
setup_service() {
  echo "Moving service file and setting up systemd..."

  # Get the current username
  current_user=$(whoami)

  # Replace 'User=root' with 'User=current_user' and set the correct path in the service file
  sed -e "s/User=root/User=${current_user}/" \
      -e "s|ExecStart=.*|ExecStart=/home/${current_user}/Blaze/target/release/blaze|" \
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
  echo "Checking the status of the 'blaze' service..."
  sudo systemctl status blaze
}

# Main script execution
cd Blaze || { echo "Failed to enter 'blaze' directory."; exit 1; }
build_project
setup_service
check_status

echo "Blaze has been deployed Successfully! Run 'sudo curl -fsSL https://raw.githubusercontent.com/ShadowArcanist/Blaze/master/scripts/test.sh | bash' to simulate high resource for 15 seconds to test Blaze (this is optional)"
