# Описание

Утилита для подсчёта суммарного количества замкнутых линий в символах текста, то есть так называемых **дырочек**.... 👉🏻👈🏻

Утилита поддерживает подсчёт как через интерфейс коммандной строки, так и через web-интерфейс.

# Демо

* https://dyrochki.onrender.com
* https://dyrochki.herokuapp.com (к сожалению, они скоро [прикроют лавочку](https://www.reddit.com/r/Heroku/comments/wxh4hv/starting_november_28_2022_free_heroku_dynos_free/))


# Компиляция

Для компиляции требуется установленный [Cargo](https://github.com/rust-lang/cargo). Рекомендую устанавливать его через [rustup](https://rustup.rs).

```bash
cargo build --release
```

# Использование

## Чистый CLI

```bash
cargo run -r count "некоторый текст для подсчёта известных отверстий"
# или
./target/release/dyrocki count "некоторый текст для подсчёта известных отверстий"
```

## Запуск веб-сервера

```bash
cargo run -r server
# или
./target/release/dyrocki server
```
