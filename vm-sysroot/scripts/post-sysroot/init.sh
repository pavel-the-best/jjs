#!/usr/bin/env bash
sudo mkdir "$SYSROOT"/{dev,proc,sys}

sudo tee "$SYSROOT/init" >/dev/null << EOF
#!/bin/sh

mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devpts /dev/pts
mkdir -p /dev/shm
chmod 1777 /dev/shm

mount -t tmpfs tmpfs /tmp
chmod 1777 /tmp

mount -t tmpfs tmpfs /sys/fs/cgroup
for i in cpuacct pids memory
do
    mkdir /sys/fs/cgroup/\$i
    mount -t cgroup -o nosuid,nodev,noexec,\$i cgroup /sys/fs/cgroup/\$i
done
echo 1 > /sys/fs/cgroup/memory/memory.use_hierarchy

mount -o remount,rw /
ifdown lo
ifup lo

haveged -F &

su postgres -c 'postgres -D /var/lib/postgresql/*/main & while ! psql -c ""; do true; done'
sleep 5

echo "We are: \$(id)"

su jjs -c '
$(cat env.txt)

if [ "x\$JJS_ENV" == xdev ]
then echo "WARNING: jjs-frontend is running in development mode. To switch to production mode, run \\\`jjs-prod\\\`." >&2
fi

jjs-frontend &
'

$(cat env.txt)

if [ "x\$JJS_ENV" == xdev ]
then echo "WARNING: jjs-frontend is running in development mode. To switch to production mode, run \\\`jjs-prod\\\`." >&2
fi

jjs-invoker &

if [ "x\$(readlink /proc/1/exe)" == x/ ]
then
    ifdown eth0
    ifup eth0
fi

if [ "\$\$" == 1 ]
then
sh
killall jjs-frontend
killall jjs-invoker
killall -INT postgres
while killall -0 postgres
do true
done
mount -o remount,sync /
mount -o remount,ro /
sync
poweroff -f
fi
EOF
sudo chmod +x "$SYSROOT/init"
sudo mkdir -p "$SYSROOT/etc/init.d"
sudo ln -s /init "$SYSROOT/etc/init.d/rcS"
