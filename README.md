# RusRus

## Requirements

 - Docker


## Depend on

[Yew ver.0.18](https://yew.rs)


## Usase

```
docker build -t rusrus .

# start serve
docker run -p 8080:8080 --rm -it -v $(pwd):/app -w /app/rusrus -e USER=username --name rusrus_container rusrus trunk serve &
(and access http://localhost:8080/)

# view log
docker logs -f rusrus_container
```

## Thanks

 - PuyoPuyo Algorithm Sample: https://puyo.sega.jp/program_2020/

