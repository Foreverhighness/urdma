# User Space RDMA Driver Framework


```bash
cd $LINUX_SRC
echo '' > patch.config
./scripts/config --file patch.config --enable CONFIG_DEBUG_INFO
./scripts/config --file patch.config --enable CONFIG_GDB_SCRIPTS
./scripts/config --file patch.config --disable CONFIG_RANDOMIZE_BASE
./scripts/config --file patch.config --disable CONFIG_WERROR
./scripts/config --file patch.config --module CONFIG_INFINIBAND
./scripts/config --file patch.config --module CONFIG_INFINIBAND_USER_MAD
./scripts/config --file patch.config --module CONFIG_INFINIBAND_USER_ACCESS
./scripts/config --file patch.config --module CONFIG_RDMA_RXE

make defconfig
./scripts/kconfig/merge_config.sh .config ./patch.config

make
make scripts_gdb
```

```bash
clangd --check=main.c |& awk -F"'" '/unknown argument:/ {print $2}' | xargs -I{} sed -i 's/{}//g' compile_commands.json
```