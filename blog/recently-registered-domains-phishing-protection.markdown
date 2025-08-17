---
title: "Recently Registered Domains: Phishing Risks & Protection"
date: "2025-08-08"
image: "https://wj6zzer4ts.ufs.sh/f/xAYSmYcg8VaGTRPBKLryMmENosYedrtPaQbx4cU03VJRSG7f"
about: "Recently registered domains are commonly used for phishing and malware. Blocking them is a simple, effective way to improve security with minimal false positives."
tags:
  - "cybersecurity"
  - "domain security"
  - "browser security"
  - "domain reputation"
---

<alert type="success" message="The RRD (Recently Registered Domains) extension is live now!" > </alert>

Get the [RRD (Recently Registered Domains)](https://chromewebstore.google.com/detail/rrd-recently-registered-d/bbohfomhenmmbmjocbjeicahihfgmjla) extension now

Most newly registered domains tend to be used for fraudulent purposes, including phishing scams and malware distribution campaigns. 
Because attackers often register fresh domains to evade detection and blacklist lists, these domains serve as a key indicator of potential threats. 
Blocking access to recently registered domains can significantly strengthen your security posture by reducing exposure to these attacks. 

From my personal experience implementing this defense strategy, false positives are rare—occurring only every few months—making it a reliable and low-maintenance protective measure.

In well-secured companies and networks, blocking newly registered domains (NRDs) is a common and effective strategy. 
Threat actors frequently register new domains just before launching malware campaigns to ensure their command-and-control servers are not blacklisted. 
By blocking domains that are less than `30 days old`, organizations can effectively disrupt many malware operation

Research by Tripwire reveals that over 70% of NRDs are classified as "malicious", "suspicious", or "not safe for work", 
a rate almost ten times higher than domains in Alexa’s top 10,000 list. Moreover, many malicious NRDs are short-lived,
existing only for a few hours or days—often disappearing before security vendors can detect them. This fleeting nature underscores why blocking NRDs is a necessary, preventive security measure.

![NDR Pie Chart](https://cdn.ebadfd.tech/recently-registered-domains-phishing-protection/NDRs%20Classification.png "NDR Pie Chart")

Palo Alto Networks defines NRDs as domains registered or changed within the previous 32 days.
While blocking these domains improves security, organizations should weigh the potential inconvenience, 
as users might be unable to access newly launched product sites or recently rebranded pages. If blocking is not feasible, 
it is recommended to monitor and set alerts for access attempts to these domains.

Additionally, certain top-level domains (TLDs), such as .to, .kl, and .nf, are more commonly associated with malicious content. 
Organizations aiming for heightened security may choose to block these TLDs entirely, though this approach may increase false positives.

Ultimately, despite occasional inconvenience, blocking or closely monitoring NRDs and high-risk TLDs significantly reduces exposure to cyber threats.

## Protecting Yourself as an Individual

It’s one thing for organizations to block recently registered domains (NRDs) — most already do this at the firewall or DNS level — but what about individuals? 
Even without enterprise-grade tools, you still have effective options to protect yourself from these threats.

If you have a home router or firewall that supports advanced filtering, you can set up a network-wide rule to block NRDs. For an easier, no-hardware-required option, 
consider using a DNS service like [NextDNS](https://help.nextdns.io/t/35yz3m6/is-there-a-way-to-block-newly-active-domains). 
NextDNS can automatically block newly registered domains at the DNS level, providing protection for all devices connected through it.

However, there are situations where you might still need to access recently registered domains — perhaps for legitimate new services, products, or events.
This is where our own solution comes in: **RRD (Recently Registered Domains)**, a browser extension that acts as a smart warning system.

![RRD Logo](https://cdn.ebadfd.tech/rrd-bg.png)

RRD works by querying a domain’s registration date and comparing it to your configured threshold (default: 30 days). 
If the domain is newer than the threshold, it displays a clear popup alert — fully customizable to suit your needs.
This ensures you’re aware of potential risks before interacting with the site.

**Project Repository:** [github.com/ebadfd/RRD](https://github.com/ebadfd/RRD)

Even if you’re already using a DNS-based blocker like NextDNS, RRD remains valuable. For example, if you have DNS-over-HTTPS (DoH) enabled in your browser,
it can bypass your DNS filter. RRD still provides protection by alerting you directly within the browser.

**Using RRD is simple:**

1. Install the extension.
2. (Optional) Adjust the day threshold by right-clicking the extension and opening the settings page.
3. Browse normally — RRD will quietly watch for risky domains and alert you when necessary.

With this combination — DNS filtering plus RRD — you can bring enterprise-grade NRD protection into your personal browsing environment.


---

*Source and images adapted from:*
- [Tripwire – Block Newly Registered Domains to Reduce Security Threats](https://www.tripwire.com/state-of-security/block-newly-registered-domains-to-reduce-security-threats-in-your-organisation)
- [Tripwire NRD Analysis](https://www.tripwire.com/sites/default/files/nrd-pie.jpeg)
- [Tripwire TLD Risk Chart](https://www.tripwire.com/sites/default/files/tld.jpeg)

