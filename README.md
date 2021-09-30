# nu_plugin_df
A `df` plugin for Nushell

## install

Clone this repository:
```shell
$ git clone git@github.com:hedonihilist/nu_plugin_df.git
```

In the repository directory, install the plugin:
```shell
$ cargo install --path .
```

The above command will install `nu_plugin_df` to `~/.cargo/bin/`, make sure you have `~/.cargo/bin` in the `nu` plugin dir list.
You can check it out in `nu` by:
```shell
> config get plugin_dirs
/home/z/.cargo/bin
```

You can add plugin dir in `nu` by (change `<HOME_DIR>` to your absolute home dir):
```shell
> config set plugin_dirs ["<HOME_DIR>/.cargo/bin"]
```

## usage in `nu`

### Simple Usage

```shell
> df
/home/z> df
────┬────────────┬──────────┬──────────┬─────────┬──────────┬──────────┬───────────┬───────────┬───────────┬──────────┬───────────────────────────────────────────────────────────────────────────────────────────
 #  │ Filesystem │   Type   │  Inodes  │  IUsed  │  IFree   │  IUse%   │   Total   │   Used    │   Avail   │   Use%   │                                        mount-point                                        
────┼────────────┼──────────┼──────────┼─────────┼──────────┼──────────┼───────────┼───────────┼───────────┼──────────┼───────────────────────────────────────────────────────────────────────────────────────────
  0 │ dev        │ devtmpfs │  2038116 │     593 │  2037523 │   0.0290 │   7.8 GiB │       0 B │   7.8 GiB │   0.0000 │ /dev                                                                                      
  1 │ run        │ tmpfs    │  2040517 │    1150 │  2039367 │   0.0563 │   7.8 GiB │   1.9 MiB │   7.8 GiB │   0.0235 │ /run/snapd/ns                                                                             
  2 │ /dev/sda5  │ ext4     │  6250496 │  804929 │  5445567 │  12.8778 │  93.4 GiB │  60.6 GiB │  32.8 GiB │  64.8701 │ /                                                                                         
  3 │ tmpfs      │ tmpfs    │  2040517 │     909 │  2039608 │   0.0445 │   7.8 GiB │ 536.7 MiB │   7.3 GiB │   6.7329 │ /dev/shm                                                                                  
  4 │ tmpfs      │ tmpfs    │   409600 │     468 │   409132 │   0.1142 │   7.8 GiB │ 137.9 MiB │   7.6 GiB │   1.7305 │ /tmp                                                                                      
  5 │ /dev/loop0 │ squashfs │    10803 │   10803 │        0 │ 100.0000 │  55.5 MiB │  55.5 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/core18/2128                                                           
  6 │ /dev/loop2 │ squashfs │    64986 │   64986 │        0 │ 100.0000 │  65.1 MiB │  65.1 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/gtk-common-themes/1515                                                
  7 │ /dev/loop1 │ squashfs │    27806 │   27806 │        0 │ 100.0000 │ 164.9 MiB │ 164.9 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/gnome-3-28-1804/161                                                   
  8 │ /dev/loop3 │ squashfs │       29 │      29 │        0 │ 100.0000 │ 128.0 KiB │ 128.0 KiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/bare/5                                                                
  9 │ /dev/loop4 │ squashfs │      474 │     474 │        0 │ 100.0000 │  32.4 MiB │  32.4 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/snapd/12883                                                           
 10 │ /dev/loop5 │ squashfs │      132 │     132 │        0 │ 100.0000 │  70.6 MiB │  70.6 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/superproductivity/1360                                                
 11 │ /dev/loop7 │ squashfs │      474 │     474 │        0 │ 100.0000 │  32.4 MiB │  32.4 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/snapd/13170                                                           
 12 │ /dev/loop6 │ squashfs │    65095 │   65095 │        0 │ 100.0000 │  65.2 MiB │  65.2 MiB │       0 B │ 100.0000 │ /var/lib/snapd/snap/gtk-common-themes/1519                                                
 13 │ /dev/sda6  │ ext4     │  6250496 │ 1377433 │  4873063 │  22.0371 │  93.4 GiB │  84.6 GiB │   8.7 GiB │  90.6399 │ /home                                                                                     
 14 │ /dev/sda8  │ fuseblk  │ 53512420 │   25929 │ 53486491 │   0.0484 │ 100.0 GiB │  49.0 GiB │  51.0 GiB │  49.0295 │ /home/z/share                                                                             
 15 │ /dev/sda1  │ vfat     │        0 │       0 │        0 │   0.0000 │ 299.8 MiB │  32.0 MiB │ 267.8 MiB │  10.6881 │ /boot/efi                                                                                 
 16 │ /dev/sda7  │ ext4     │  5005312 │     305 │  5005007 │   0.0060 │  74.6 GiB │  13.7 GiB │  60.9 GiB │  18.3452 │ /home/trusted                                                                             
 17 │ tmpfs      │ tmpfs    │   408103 │     198 │   407905 │   0.0485 │   1.6 GiB │ 212.0 KiB │   1.6 GiB │   0.0129 │ /run/user/1000                                                                            
 18 │ overlay    │ overlay  │  6250496 │  804929 │  5445567 │  12.8778 │  93.4 GiB │  60.6 GiB │  32.8 GiB │  64.8701 │ /var/lib/docker/overlay2/ce8223387557f93a71985cc1d0a6d4cadef3a3220a0c15ab3944671353d7f8e0 
    │            │          │          │         │          │          │           │           │           │          │ /merged                                                                                   
────┴────────────┴──────────┴──────────┴─────────┴──────────┴──────────┴───────────┴───────────┴───────────┴──────────┴───────────────────────────────────────────────────────────────────────────────────────────

```

### Show all file systems

Show all file systems, including pseudo, duplicate, inaccessible file systems
```shell
> df -a
```

### Show local file systems only

Show local file systems only
```shell
> df -l
```