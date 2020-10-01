# Spooky Wifi QR Generator

Turns out I'm really good at making frivolous, single-use utilities.

## Usage

Download the latest release binary from the releases tab and run it. You can use the `--help` flag because I'm too lazy to write real documentation.

```
Spooky Wifi QR Code Generator 0.1.0
Kyle Macey <kylemacey@github.com>
Generate spooky QR codes for connecting to your Wifi in the terminal

USAGE:
    spookyqr [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --encryption <encryption>    WEP, WPA
    -f, --false_char <false_char>    the false character displayed
    -p, --password <password>
    -r, --resolution <resolution>    the resolution of the an individual cell n*n
    -s, --ssid <ssid>                The SSID name to connect to
    -t, --true_char <true_char>      the true character displayed. be extra spooky
```

You get a neat little QR Code like this!

![image](https://user-images.githubusercontent.com/519171/94859177-1f822280-0402-11eb-832f-62ec102d416c.png)

## Tips

- Use emoji with fixed width and high contrast
- Make sure it's spooky
- Have fun!

## License

MIT