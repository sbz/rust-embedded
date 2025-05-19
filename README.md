# rust-embedded

This is to build native rust code for the SOC chip of the board **Rockchip RK3399**

# Rust Toolchain

You need to setup rust toolchain as described in the file [rust-toolchain.toml](./rust-toolchain.toml) with corresponding ABI, for in our case it's `armv7-unknown-linux-gnueabi`.

It can be done using the following `rustup` command

```
rustup target add --toolchain stable armv7-unknown-linux-gnueabi
```

# SoC access

## U-boot configuration

You can used [u-boot overlays](https://docs.u-boot.org/en/latest/usage/fdt_overlays.html) to specify
the corresponding chip to boot:

```
overlay_prefix=rockchip-rk3399
overlays=uart4
```

## Flattened Device Tree

You need to use the device tree blob for the `rockpro64`:

```
rk3399-rockpro64.dtb
```

It can be decompiled or analyzed using [fdtdump](https://elixir.bootlin.com/linux/v4.9/source/scripts/dtc/fdtdump.c) or device tree compiler [dtc](https://elixir.bootlin.com/linux/v4.9/source/scripts/dtc/dtc.c)

```
$ fdtdump /boot/dtb/rockchip/rk3399-rockpro64.dtb
$ dtc -I dtb -O dts /boot/dtb/rockchip/rk3399-rockpro64.dtb -o rk3399-rockpro64.dts
$ cat rk3399-rockpro64.dts
```

## Pinouts

This use a direct UART connection on the Pine64 board via the GPIO pins

- `6` GND
- `8` RXD
- `10` TXD

with baud rate speed at `1500000`.

![GPIO pinout cables](https://github.com/sbz/rust-embedded/blob/main/images/pine64.jpg?raw=true)

## USB Bridge model

**Module `CH341T` on 1.33 / 5 V via USB to I2C TTL

The UART bridge model is required to process such high speed rate compare to traditional `115200` or `9600`, it's identified by
dmesg as `ch341` available [on aliexpress](https://fr.aliexpress.com/item/1005007322215978.html):

```
$ sudo dmesg -t --follow
...
usb 4-2: new full-speed USB device number 3 using xhci_hcd
usb 4-2: New USB device found, idVendor=1a86, idProduct=7523, bcdDevice= 2.64
usb 4-2: New USB device strings: Mfr=0, Product=2, SerialNumber=0
usb 4-2: Product: USB Serial
usbcore: registered new interface driver ch341
usbserial: USB Serial support registered for ch341-uart
ch341 4-2:1.0: ch341-uart converter detected
usb 4-2: ch341-uart converter now attached to ttyUSB0
```

## Serial connection

You can use the following command to access the serial UART bridge

```
sudo minicom -D /dev/ttyUSB0 -b 1500000
sudo screen /dev/ttyUSB0 1500000
sudo picocom --baud 1500000 /dev/ttyUSB0
```

# Resources

- [Rockip Wiki](https://opensource.rock-chips.com/wiki_Main_Page)
- [Pine64 Wiki RockPro64 page](https://wiki.pine64.org/wiki/ROCKPro64)
- [Debug Bridge](https://crwulff.blogspot.com/p/rock64.html)
- [U-boot](https://www.u-boot.org)
- [Device Tree Compiler spec](https://devicetree-specification.readthedocs.io/en/stable/introduction.html)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book)
