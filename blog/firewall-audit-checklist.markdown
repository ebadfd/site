---
title: "Firewall Audit Checklist"
date: "2024-10-18"
image: "https://utfs.io/f/xAYSmYcg8VaG405tOAIeAULufYZFOl8H4mSq2so7Rc0JPD1X"
tags:
  - "audit"
  - "firewalls"
---

Auditing your firewall is a critical step in maintaining a secure network environment. This generic checklist can be used regardless of the vendor,
focusing solely on the technical aspects of firewall management. As you conduct your audit, it's also essential to consider manual elements, such as the physical security of the firewalls

## Key Information to Gather Before the Audit

Before completing the audit, there are a few key pieces of information that need to be collected:

- **Firewall Vendor Information**: Gather all relevant details, including OS version, latest patches, and default configuration.
- **Security Policies**: Obtain copies of relevant security policies.
- **Firewall Logs**: Access logs that can be analyzed against the firewall rule base to understand which rules are actively being used.
- **Network Diagrams**: Ensure you have an accurate diagram of the current network and firewall topologies.
- **Previous Audit Reports**: Review reports and documents from previous audits, including firewall rules, objects, and policy revisions.
- **ISP and VPN Identification**: Identify all Internet Service Providers (ISPs) and Virtual Private Networks (VPNs) in use.
- **Critical Asset Understanding**: Familiarize yourself with key servers and information repositories in the network and assess their value.
- **Port Restrictions**: Before recommending which ports to block, ensure that the services associated with those ports are not critical to business operations.
- **Internal Modems**: Check for any modems within the internal network, as they can pose a threat by bypassing firewall protections.
- **Application-Level Firewalls**: Ensure that the operating system for any application-level firewalls is as secure as possible, since their effectiveness relies on both components.
- **Defense in Depth**: Recognize that firewall implementation is part of a broader security strategy. Assess the security of additional components, such as Intrusion Detection Systems (IDS) and Intrusion Prevention Systems (IPS).
- **Remote User Security**: For users connecting to the corporate network via VPN, verify the security of their end devices to maintain network integrity.

## Firewall Audit Checklist

| No. | Checklist Item                         | Description                                                                                       | Action                               |
|-----|----------------------------------------|---------------------------------------------------------------------------------------------------|--------------------------------------|
| 1   | Ruleset Review                      | Ensure the ruleset follows the proper order: <br> 1. Anti-spoofing (block private addresses and internal addresses appearing from the outside). <br> 2. User Permit (e.g. allow HTTP to public webserver). <br> 3. Management Rules (e.g. SNMP traps to network management server). <br> 4. Noise Drops (e.g. discard OSPF and HSRP chatter). <br> 5. Deny and Alert (alert systems administrator about suspicious traffic). <br> 6. Deny and Log (log remaining traffic for analysis). |
| 2   | Application-Based Firewalls           | Monitor attempts to violate security policy; block specific SMTP and FTP commands.                | Implement monitoring and blocking     |
| 3   | Stateful Inspection                    | Review rules for source/destination IPs, ports, and timeouts.                                   | Adjust rules and timeouts            |
| 4   | Logging                                | Enable logging and regularly review logs for potential attack patterns.                          | Check logging configuration           |
| 5   | Patches and Updates                    | Apply latest patches and updates; ensure update sources are trusted.                             | Verify and apply updates              |
| 6   | Location – DMZ                        | Ensure two firewalls are in place: one to the internet, one to the internal network.            | Verify firewall placement             |
| 7   | Vulnerability Assessments/Testing      | Establish a process for testing open ports; ensure unnecessary ports are closed.                 | Conduct regular vulnerability scans   |
| 8   | Compliance with Security Policy        | Confirm that the ruleset complies with the organization's security policies.                     | Review compliance                     |
| 9   | Block Spoofed, Private, and Illegal Addresses | Ensure specific addresses are blocked (e.g., unroutables, private). check the  Spoofed Illegal Address List section                             | Configure address filtering           |
| 10  | Source Routing                        | Block and log loose and strict source routing.                                                  | Update firewall settings              |
| 11  | Block Specific Ports                   | Check and block specified ports based on security needs. check the Ports That Should Be Blocked section                                         | Implement port blocking               |
| 12  | Remote Management                      | Use SSH instead of Telnet for remote management.                                                 | Configure remote access               |
| 13  | FTP Server Segmentation                | Ensure the FTP server is on a separate subnet.                                                  | Verify server placement               |
| 14  | ICMP Filtering                         | Implement rules to block ICMP echo requests and replies.                                         | Adjust ICMP settings                  |
| 15  | Zone Transfers                         | Ensure proper filtering for DNS to prevent unauthorized zone transfers.                          | Configure DNS filtering               |
| 16  | Egress Filtering                       | Allow only traffic originating from internal IPs; log external traffic.                          | Set up egress filtering               |
| 17  | Critical Servers                       | Deny traffic directed at critical internal addresses from external sources.                      | Implement deny rules                  |
| 18  | Personal Firewalls                    | Provide training on personal firewalls and review settings.                                      | Conduct user training                 |
| 19  | Distributed Firewalls                 | Ensure consistent security policy distribution and integrity controls during transfer.           | Review policy distribution            |
| 20  | Stealth Firewalls                     | Reset default usernames and passwords; review access control lists.                               | Secure firewall configuration         |
| 21  | ACK Bit Monitoring                     | Establish monitoring to prevent remote systems from initiating TCP connections.                  | Implement ACK monitoring              |
| 22  | Continued Availability of Firewalls    | Ensure there is a hot standby for the primary firewall.                                          | Set up redundancy                     |


### Ports That Should Be Blocked

To enhance network security, it is essential to block certain ports that are not in use. The following ports are known for vulnerabilities and should be restricted to prevent unauthorized access and potential exploits.


| Service Description                           | Port Type | Port Number             |
|-----------------------------------------------|-----------|-------------------------|
| DNS Zone Transfers (except from external)     | TCP       | 53                      |
| TFTP Daemon                                   | UDP       | 69                      |
| Link                                          | TCP       | 87                      |
| SUN RPC                                       | TCP & UDP | 111                     |
| BSD UNIX                                      | TCP       | 512 – 514               |
| LPD                                           | TCP       | 515                     |
| UUCPD                                         | TCP       | 540                     |
| Open Windows                                   | TCP & UDP | 2000                    |
| NFS                                           | TCP & UDP | 2049                    |
| X Windows                                     | TCP & UDP | 6000 – 6255             |
| Small services                                 | TCP & UDP | 20 and below            |
| FTP                                           | TCP       | 21                      |
| SSH                                           | TCP       | 22                      |
| Telnet                                        | TCP       | 23                      |
| SMTP (except external mail relays)            | TCP       | 25                      |
| NTP                                           | TCP & UDP | 37                      |
| Finger                                        | TCP       | 79                      |
| HTTP (except to external web servers)         | TCP       | 80                      |
| POP                                           | TCP       | 109 & 110               |
| NNTP                                          | TCP       | 119                     |
| NTP                                           | TCP       | 123                     |
| NetBIOS in Windows NT                         | TCP & UDP | 135                     |
| NetBIOS in Windows NT                         | UDP       | 137 & 138               |
| NetBIOS                                       | TCP       | 139                     |
| IMAP                                          | TCP       | 143                     |
| SNMP                                          | TCP       | 161 & 162               |
| SNMP                                          | UDP       | 161 & 162               |
| BGP                                           | TCP       | 179                     |
| LDAP                                          | TCP & UDP | 389                     |
| SSL (except to external web servers)          | TCP       | 443                     |
| NetBIOS in Win2k                             | TCP & UDP | 445                     |
| Syslog                                        | UDP       | 514                     |
| SOCKS                                         | TCP       | 1080                    |
| Cisco AUX port                                | TCP       | 2001                    |
| Cisco AUX port (stream)                       | TCP       | 4001                    |
| Lockd (Linux DoS Vulnerability)               | TCP & UDP | 4045                    |
| Cisco AUX port (binary)                       | TCP       | 6001                    |
| Common high order HTTP ports                  | TCP       | 8000, 8080, 88         |

### Spoofed Illegal Address List

Ensure that the following spoofed, private (RFC 1918), and illegal addresses are blocked:

#### Standard Unroutables
- **255.255.255.255**
- **127.0.0.0**

#### Private (RFC 1918) Addresses
- **10.0.0.0 – 10.255.255.255**
- **172.16.0.0 – 172.31.255.255**
- **192.168.0.0 – 192.168.255.255**

#### Reserved Addresses
- **240.0.0.0**

#### Illegal Addresses
- **0.0.0.0**

#### Additional Considerations
- **UDP Echo**
- **ICMP Broadcast (RFC 2644)**

Ensure that traffic from the above addresses is not transmitted by the interface.


### References

- [SANS Firewall Checklist](https://www.sans.org/media/score/checklists/FirewallChecklist.pdf)
- [AlgoSec Firewall Audit Checklist](https://www.algosec.com/resources/firewall-audit-checklist)
- [eSecurity Planet: How to Do a Firewall Audit](https://www.esecurityplanet.com/networks/how-to-do-a-firewall-audit/)

