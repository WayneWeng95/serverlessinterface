# vHive compile process

## Cleanups
```
sudo nft delete table ip nat //Iptable

sudo kubeadm reset //resetkubeadm

./scripts/github_runner/clean_cri_runner.sh //vHive cleanup

```
## node setup
```
./setup_tool setup_node firecracker
```

## backgrounds
```
sudo containerd

sudo /usr/local/bin/firecracker-containerd --config /etc/firecracker-containerd/config.toml

sudo chmod -t /tmp

sudo ./vhive

sudo kubeadm config images pull
```

## Start the system
```
./setup_tool create_one_node_cluster firecracker
```