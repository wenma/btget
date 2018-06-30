### btget - bt命令行下载工具 （written in rust）
### 使用

```sh
$ btget -h

btget 0.1.0
Wen Ma <wenma1993@gmail.com>

USAGE:
    btget [FLAGS] [OPTIONS] <FILE>

FLAGS:
    -a, --analysis    Only do analysis stuff for the torrent file
    -h, --help        Prints help information
    -V, --version     Prints version information
    -v                Sets the level of verbosity

OPTIONS:
    -s, --speed <M/s>    Sets a custom download speed

ARGS:
    <FILE>    Sets the input torrent file to use
```

### 种子文件解析
```sh
$ btget ~/Desktop/西部世界.第一季S01E01.torrent --analysis
+------------+---------------------------------------------------------------------------------+------------------------------------------+
| 文件大小   | 文件名                                                                          | hash值                                   |
| 239 bytes  | 字幕翻译人员名单.txt                                                            | f0387ce00f65689385fe03e9b46fb3e49628daa4 |
| 27 bytes   | 更多高清请访问www.mp4ba.com.txt                                                 | 0d6d55a75725086f2419a423706f33a918fe68bd |
| 50 bytes   | 本站唯一域名www.mp4ba.com.txt                                                   | f89bfa78ef13ca7aefc5e852b979614ee9b916c4 |
| 122 bytes  | 点击进入高清MP4ba.url                                                           | 965d39ff74c817d41feb67dfa8a2ee50682a7f3f |
| 1.4145 GiB | 西部世界.第一季.Westworld.S01E01.2016.HD720P.X264.AAC.English.CHS-ENG.Mp4Ba.mp4 | 8d450423d183764da01e30118e480df228f19450 |
+------------+---------------------------------------------------------------------------------+------------------------------------------+
```