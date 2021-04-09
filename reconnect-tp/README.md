ThinkPad X1 Carbon Gen6 のトラックポイントがスリープから復帰したあとに効かなくなることがあるので、強制的につなぎ直すプログラム。

## systemd のサービスとして実行させる
`/etc/systemd/system/reconnect-tp.service` に以下のようなファイルを配置して

```
[Unit]
After=network.target

[Service]
User=root
ExecStart=/path/to/reconnect-tp -l
Restart=always

[Install]
WantedBy=multi-user.target
```

サービスを有効化する。

```
$ sudo systemctl daemon-reload
$ sudo systemctl enable reconnect-tp.service
$ sudo systemctl start reconnect-tp.service
```
