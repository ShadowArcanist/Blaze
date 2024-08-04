![Blaze](/public/banner.png)

# Blaze
Blaze is a Rust script designed to run 24/7 on a server, providing a summary of resource usage every 30 minutes (configurable) to a Discord channel via Webhook.

It also sends alerts to Discord when specific conditions are met, such as high resource usage (RAM and CPU) by any process, potential DDoS attack indications, or unusually high server resource usage (configurable thresholds). The script monitors the server every 3 seconds (configurable).

<br />

## Features
![Blaze Features](/public/features.png)

<br />

## Screenshots
![Blaze Discord Screenshots](/public/screenshots.png)

<br />

## Links
- [Documentation](https://envix.shadowarcanist.com/docs/monitoring-tools/blaze) (Written guide to setup & use blaze)
- [Youtube](https://www.youtube.com/watch?v=iQoID4Msx3w) (Video tutorial to setup & use blaze)

<br />

## Disclaimer 
The script has been tested and used on a production server running Ubuntu 24.04 LTS. Testing for DDoS attacks directly on a production server is not feasible. Instead, we tested the script on a Virtual Machine running Ubuntu 24.04 LTS with a simulated DDoS attack using hping3, and it worked well.

<br />

## Can I contribute to this project?
Yes, you can contribute. However, we may or may not accept Pull Requests for new changes. Before investing time in writing or updating code, please create an issue to discuss the topic you plan to contribute on.

<br />

## Licence
This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/ShadowArcanist/blaze/blob/master/LICENCE) file for details
