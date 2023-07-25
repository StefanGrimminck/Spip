# spip-agent and Elasticsearch Integration Guide

This guide describes how to integrate spip-agent with Elasticsearch, allowing the agent to send its stdout logs directly to an Elasticsearch instance. This approach bypasses the need to save logs to a local file.
## Prerequisites

1. Ensure you have spip-agent set up as per the main README.
2. An active Elasticsearch instance. If you don't have one, you can set up a local instance for testing or use a managed service like Elastic Cloud.

## Steps for Integration
#### 1. Install Filebeat

Filebeat is a lightweight shipper for logs. You'll use it to capture the stdout from spip-agent and forward it to Elasticsearch.

```bash

# For Debian/Ubuntu:
sudo apt-get install filebeat

# For RedHat/CentOS:
sudo yum install filebeat
```

#### 2. Configure Filebeat

Navigate to the Filebeat configuration directory (typically /etc/filebeat/).

Edit the filebeat.yml configuration:

```bash
sudo vim /etc/filebeat/filebeat.yml
```

Now, configure Filebeat to capture stdout from spip-agent. Add the following lines:

```yaml
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /proc/$(pgrep spip-agent)/fd/1
```

Set up the Elasticsearch output:

```yaml
output.elasticsearch:
  hosts: ["your_elasticsearch_host:port"]
  index: "spip-agent-logs-%{+yyyy.MM.dd}"
```

Replace your_elasticsearch_host:port with the address of your Elasticsearch instance.
#### 3. Start Filebeat

Start the Filebeat service:

```bash
sudo service filebeat start
```
#### 4. Verify Integration

Run spip-agent. As it outputs logs to stdout, Filebeat should capture and send them to the specified Elasticsearch instance. Check your Elasticsearch for the new index (spip-agent-logs-<date>) and ensure that logs are populating correctly.