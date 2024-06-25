# Используем официальный образ Ubuntu для сборки и запуска приложения
FROM ubuntu:20.04 AS base

RUN apt update && apt install -y tcl

# Устанавливаем необходимые зависимости
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libssl-dev \
    pkg-config \
    ca-certificates

# Устанавливаем Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Создаем рабочую директорию
WORKDIR /usr/src/app

# Копируем Cargo.toml и Cargo.lock для установки зависимостей
COPY . .

# Собираем приложение в релизном режиме
RUN cargo build --release

# Используем минимальный образ Ubuntu для запуска приложения
FROM ubuntu:20.04

# Устанавливаем зависимости, необходимые для запуска скомпилированного приложения
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Создаем рабочую директорию
WORKDIR /usr/src/app

# Копируем скомпилированное приложение из предыдущего шага
COPY --from=base /usr/src/app/target/release/matrix .

EXPOSE 3000

# Указываем команду по умолчанию для запуска приложения
CMD ["./matrix"]

