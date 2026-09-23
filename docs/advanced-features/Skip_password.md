<!-- markdownlint-disable MD033 -->
# 跳过开机输入密码与锁屏界面

设置前记得开启AUTO-MAS的开机自启功能~
也记得给队列设置定时启动~

## 跳过密码输入

### Windows

1.下载 autologon

- [点击进入autologon下载网页](<https://learn.microsoft.com/zh-cn/sysinternals/downloads/autologon>)

![下载Autologon.png](https://image.989464244.xyz/file/AgACAgUAAyEGAATrAAHMTwADE2qzl4QJrV4HphI2nMdO4ecwPrhrAALREmsbMdWgVU7XDaRjyRhQAQADAgADdwADPQQ.png)

2.下载完成后,双击运行autologon.exe
![双击运行.png](https://image.989464244.xyz/file/AgACAgUAAyEGAATrAAHMTwADFWqzl8xlDdCxOG4zpp_AYaPgiZX3AALSEmsbMdWgVRXQp_Mp4DS1AQADAgADbQADPQQ.png)

3.在第三行输入你的密码(第一第二行一般会自动读取你的账户名和设备名)

是本地账户就输本地账户的密码，是微软账户就输微软账户的密码
![输入密码.png](https://image.989464244.xyz/file/AgACAgUAAyEGAATrAAHMTwADFmqzmM584KRSWx1qtgROj4robB3xAAK6GGsbkMmhVVHjllIw7aIbAQADAgADeAADPQQ.png)

4.点击Enable
![点击确认.png](https://image.989464244.xyz/file/AgACAgUAAyEGAATrAAHMTwADF2qzmUqtq-Sa8yKbCnc2gAOHmt1AAAK-GGsbkMmhVVZZm4qNkKiHAQADAgADeAADPQQ.png)

5.实际测试
![成功.png](https://image.989464244.xyz/file/AgACAgUAAyEGAATrAAHMTwADGGqzna4vSTF20Gxc0iiqvffj-vhsAALsGGsbkMmhVeoid8_w-n1mAQADAgADeAADPQQ.png)
若出现的是上方所示的弹窗，那便表示设置成功，快去重启试试看吧

<details>
<summary>坏结局</summary>

![失败.png](https://image.989464244.xyz/file/AgACAgUAAyEGAATrAAHMTwADGWqzoUd6ntoY60P2MZqu9S1rISHDAAImGWsbkMmhVTENEG6D4lwoAQADAgADeAADPQQ.png)
密码输入错误，如果你尝试的是微软账户的密码那就换成输入本地账户密码，反之亦然

### MacOS

待补充...

### Linux

待补充...

</details>

## 定时开机

<!-- markdownlint-disable-next-line MD024 -->
### Windows

<details>

<summary>以下方法来源网络，所以没有截图，若有误请及时反馈</summary>

[来源](<https://www.bilibili.com/video/BV1pKwxzwEaP?vd_source=43fdeaf65ea5c12a419d74d955a2ebfd>)
以及通义千问

</details>

1.进入BIOS界面(每个品牌/型号的按键可能不一样，请自行查阅硬件厂商说明)

启动电脑时，一直按下 F12 或 Delete 进入BIOS界面

2.进入BIOS界面后，进入 SETTINGS 界面

3.找到 Wake Up Event Setup

- RTC(Real-Time Clock)是主板上的独立时钟芯片，即使电脑断电也能保持运行。你可以设置一个未来的时间点，RTC会在到达该时间时发送信号唤醒主板电源，启动系统
- 将 Resume by RTC Alarm 设置为 Enable

4.将 Date (of Month) Alarm 设置为 0

- "Date of Month Alarm"(日期闹钟)是 RTC 闹钟的一个设置选项，表示每月的哪一天触发闹钟,设置为0那就每天触发
- 下方的 time(hh)(mm) 与 (ss) 就表示电脑自动开机的时间
- 例如：我希望可以每天 3:45 自动开机，那我就如下设置

  - Time(hh) Alarm：3
  - Time(mm) Alarm：45
  - Time(ss) Alarm：0

改完按 F10 就可以保存退出、重启进入系统了

<!-- markdownlint-disable-next-line MD024 -->
### MacOS

待补充...

<!-- markdownlint-disable-next-line MD024 -->
### Linux

待补充...
