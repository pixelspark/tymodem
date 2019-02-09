# Tymodem

Read connection information from Huawei E3372s LTE dongle and print to console.

## Usage

`cargo run`

Will output:

````
State: operator=XXX (MCC, MNC) mode=automatic conn=LTE (rssi=-85 dBm, rsrq=32, rsrp=-105 dBm, sinr=161) err=no error
````

Every second.

By default, tymodem will read data from `/dev/cu.HUAWEIMobile-Pcui`, which is the port available for 
querying the stick on Mac.

## Compatibility

Note: the E3372 comes in two different versions, differing only by the firmware each uses:

* The `E3372s` has 'stick mode' firmware, which makes the modem appear to your computer as an AT modem. This has better compatibility (e.g. works much better on Mac) but has lower maximum throughput. You can configure the stick using AT commands over the 'Pcui' port that is made available when the stick is inserted (presumably there is software from Huawei to assist the user).
* The `E3372h` has 'HiLink mode' firmware, which makes the modem appear to your computer as a network adapter. Configuring the modem can be done through a web interface. In some cases an AT interface also appears to be availble.

Tymodem only supports 'stick mode'. It is possible to flash the 'stick mode' firmware onto an E3372h by following [these](https://www.0xf8.org/2017/01/flashing-a-huawei-e3372h-4g-lte-stick-from-hilink-to-stick-mode/) instructions.

## AT commands used

The following is an overview of the most useful AT commands supported by the E3372. Although they are not documented for
the E3372 specifically, many of the commands that work for other Huawei devices (e.g. [as listed here](http://download-c.huawei.com/download/downloadCenter?downloadId=51047&version=120450&siteCode)) work for the E3372.

(You can try these yourself by connecting to the `/dev/cu.HUAWEIMobile-Pcui` port using `screen` or a serial terminal app).

### Stick info

#### ATI

Queries device information

````
> ATI
< Manufacturer: huawei
< Model: E3372
< Revision: 21.180.01.00.00
< IMEI: XXX
< +GCAP: +CGSM,+DS,+ES
````

#### AT^VERSION?

Query firmware versions.

````
> AT^VERSION?
< ^VERSION:BDT:Sep 30 2014, 15:17:21
< ^VERSION:EXTS:21.180.01.00.00
< ^VERSION:INTS:
< ^VERSION:EXTD:WEBUI_17.100.18.05.1217_HILINK
< ^VERSION:INTD:
< ^VERSION:EXTH:CL2E3372HM Ver.A
< ^VERSION:INTH:
< ^VERSION:EXTU:E3372
< ^VERSION:INTU:
< ^VERSION:CFG:1004
< ^VERSION:PRL:
< ^VERSION:OEM:
< ^VERSION:INI:
````

#### AT^TBATVOLT?

Query battery voltage (not applicable for the E3372 as it is a USB-powered stick, but probably returns the USB voltage).

````
> AT^TBATVOLT?
< ^TBATVOLT:4458
````

#### AT^CHIPTEMP?

Query the chip temperature.

````
> AT^CHIPTEMP?
< ^CHIPTEMP: 304,304,65535,27,65535
````

### SIM info

#### AT+COPN

Query known/supported network names.

````
> AT+COPN
< +COPN: "732123","Movistar"
< +COPN: "73402","DIGITEL GSM"
...
````

#### AT+CPOL?

Query preferred roaming networks.

````

> AT+CPOL?
+CPOL: 1,2,"26202",1,0,1,1
....
+CPOL: 62,2,"40443",1,0,1,1
````

#### AT^ICCID?

Query the ICCID (SIM card number).

````
> AT^ICCID?
< ^ICCID: 8931XXXXXXXXXXXXX
````

#### AT^CPBR?

Query the SIM phone book.

````
> AT^CPBR=?
< ^CPBR: (1-250),40,20
> AT^CPBR=1
< ^CPBR: 1,"+XXXXXXXXX",145,"XXXXXXX",0
````

### Connection info

#### AT^PLMN?

Query the connected network code (MNC/MCC).

````
> AT^PLMN? 
< ^PLMN: mnc,mcc
````

#### AT^LOCINFO?

Query network location info.

````
> AT^LOCINFO?
< ^LOCINFO:XXXXX,0xXXXX,0xXX,0xXXXXXX
````

#### AT^CREG?

Query registration info, which contains (among other things) the cell ID.

````
< AT+CREG?
> +CREG: 2,1,"XXXX","XXXX"
````

#### AT+COPS?

Query network operator info:

````
> AT+COPS?
< +COPS: 0,0,"NAME OF MNO",7
````

#### AT^CSNR?

Query signal-to-noise level.

````
> AT^CSNR?
< ^CSNR: -145,-32
````

#### AT^CERRSI?

Query received signal strength indications (RSSI).

````
> AT^CERSSI?
< ^CERSSI:0,0,0,0,255,-90,-7,15,32639,32639,32639
````

#### AT^DSFLOWQRY

Query flow (traffic) statistics.

````
> AT^DSFLOWQRY
< ^DSFLOWQRY:00000010,0000000000060A85,000000000003CC18,00000010,0000000000060A85,000000000003CC18
````

