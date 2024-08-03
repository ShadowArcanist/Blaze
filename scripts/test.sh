#!/bin/bash

# Install Testing Tools
echo "Installing testing tools..."
sudo apt update && sudo apt install -y hping3 stress-ng

# Main script execution
echo "Starting system load simulations..."

# Simulate CPU Load for 15s
echo "Simulating CPU load for 15s. Check Discord for notification."
sudo stress-ng --cpu 10 --timeout 15s

# Simulate Memory Load for 15s
echo "Simulating Memory load for 15s. Check Discord for notification."
sudo stress-ng --vm 6 --vm-bytes 5G --timeout 15s

# Simulate Network Load for 15s (DDoS Attack Test)
echo "Simulating Network load for 15s (DDoS Attack Test). Check Discord for notification."
sudo timeout 15s hping3 --flood -d 100000 localhost

echo "Test completed!"
