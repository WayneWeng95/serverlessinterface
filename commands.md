

# The IP address of a guest is derived from its MAC address with
# `fcnet-setup.sh`, this has been pre-configured in the guest rootfs. It is
# important that `TAP_IP` and `FC_MAC` match this.
FC_MAC="06:00:AC:10:00:02"

# Set network interface
curl -X PUT --unix-socket "${API_SOCKET}" \
    --data "{
        \"iface_id\": \"net1\",
        \"guest_mac\": \"$FC_MAC\",
        \"host_dev_name\": \"$TAP_DEV\"
    }" \
    "http://localhost/network-interfaces/net1"

# API requests are handled asynchronously, it is important the configuration is
# set, before `InstanceStart`.
sleep 0.015s

# API requests are handled asynchronously, it is important the microVM has been
# started before we attempt to SSH into it.
sleep 0.015s

# SSH into the microVM
ssh -i ./ubuntu-22.04.id_rsa root@172.16.0.2

# Use `root` for both the login and password.
# Run `reboot` to exit.


# Running on the local machine

## Start the firecracker with the API socket and the Config JSON:

```
sudo ./../firecracker/build/cargo_target/x86_64-unknown-linux-musl/debug/firecracker --api-sock /tmp/firecracker.socket  --config-file vm_config.json
```

## Terminate this socket:

```
sudo curl --unix-socket /tmp/firecracker.socket -i -X PUT 'http://localhost/actions' -H  'Accept: application/json' \
-H  'Content-Type: application/json' -d'
{
   "action_type": "SendCtrlAltDel"
}'

```

## Track the firecracker pid:

```
top -b | less | grep firecr
```

## Note: Need to remove the socket before start it:

```
sudo rm firecracker.socket
```

## Start the instance:

```
sudo curl --unix-socket /tmp/firecracker.socket -i -X PUT 'http://localhost/actions' -H  'Accept: application/json' \
-H  'Content-Type: application/json' -d'
{
   "action_type": "InstanceStart"
}'
```

## Flush the Metrics:

```
sudo curl --unix-socket /tmp/firecracker.socket -i -X PUT 'http://localhost/actions' -H  'Accept: application/json' \
-H  'Content-Type: application/json' -d'
{
   "action_type": "FlushMetrics"
}'
```

## Running with sudo

```
sudo ./target/debug/dirty_page_tracker 1
```

# Command on the host configurations

## Setup network interface on host
```
TAP_DEV="tap0"
TAP_IP="172.16.0.1"
MASK_SHORT="/30"

sudo ip link del "$TAP_DEV" 2> /dev/null || true
sudo ip tuntap add dev "$TAP_DEV" mode tap
sudo ip addr add "${TAP_IP}${MASK_SHORT}" dev "$TAP_DEV"
sudo ip link set dev "$TAP_DEV" up
```

## Enable ip forwarding
```
sudo sh -c "echo 1 > /proc/sys/net/ipv4/ip_forward"
```


## Set up microVM internet access
```
sudo iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE || true
sudo iptables -D FORWARD -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT \
    || true
sudo iptables -D FORWARD -i tap0 -o eth0 -j ACCEPT || true
sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo iptables -I FORWARD 1 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT
sudo iptables -I FORWARD 1 -i tap0 -o eth0 -j ACCEPT

API_SOCKET="/tmp/firecracker.socket"
LOGFILE="./firecracker.log"
```


# Adding the log file

## Create log file
touch $LOGFILE

## Set log file
curl -X PUT --unix-socket "${API_SOCKET}" \
    --data "{
        \"log_path\": \"${LOGFILE}\",
        \"level\": \"Debug\",
        \"show_level\": true,
        \"show_log_origin\": true
    }" \
    "http://localhost/logger"

KERNEL="./vmlinux-5.10.204"
KERNEL_BOOT_ARGS="console=ttyS0 reboot=k panic=1 pci=off"

ARCH=$(uname -m)

if [ ${ARCH} = "aarch64" ]; then
    KERNEL_BOOT_ARGS="keep_bootcon ${KERNEL_BOOT_ARGS}"
fi


# Command for the Start the VM

## Start Firecracker

```
weikang@ghost:/home/shared/Firecracker$
./firecracker/build/cargo_target/x86_64-unknown-linux-musl/debug/firecracker --api-sock /tmp/firecracker.socket
```

## Set boot source

```
curl --unix-socket /tmp/firecracker.socket -i       -X PUT 'http://localhost/boot-source'         -H 'Accept: application/json'                 -H 'Content-Type: application/json'           -d "{
            \"kernel_image_path\": \"/home/shared/images/vmlinux-5.10.198\",
            \"boot_args\": \"console=ttyS0 reboot=k panic=1 pci=off\"
       }"
```

## Set rootfs

```
curl --unix-socket /tmp/firecracker.socket -i   -X PUT 'http://localhost/drives/rootfs'   -H 'Accept: application/json'             -H 'Content-Type: application/json'       -d "{
        \"drive_id\": \"rootfs\",
        \"path_on_host\": \"/home/shared/images/ubuntu-22.04.ext4\",
        \"is_root_device\": true,
        \"is_read_only\": false
   }"
```

## Start the server
```

curl --unix-socket /tmp/firecracker.socket -i -X PUT 'http://localhost/actions' -H 'Accept: application/json' -H 'Content-Type: application/json' -d'
{
"action_type": "InstanceStart"
}'
```

## End Execuation

```
curl --unix-socket /tmp/firecracker.socket -i -X PUT 'http://localhost/actions' -H  'Accept: application/json' -H  'Content-Type: application/json' -d'
{
   "action_type": "SendCtrlAltDel"
}'
```


# What left, network interface