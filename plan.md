## План улучшений для `telegram-dice-bot`

Краткий, практичный план по внедрению тестов, CI/CD, контейнеризации, наблюдаемости и улучшений функционала. Выполняем по фазам, каждая фаза даёт самодостаточную ценность.

### Фаза 1 — Базовое качество (тесты, стиль, лицензия)
- [x] Тесты ядра игры (`src/game.rs`):
  - [x] Unit-тесты для `check_even_odd`, `check_high_low`, `check_exact_number`, `check_guess_one`, `dice_emoji`
  - [x] Property-тесты (crate `proptest`) для диапазона 1..=6 и свойств чётности/диапазонов
  - [ ] Интеграционные тесты (`tests/`): сценарии `/start`, `/help`, `/play` и callback’и c моком Telegram API
- [x] Лицензия: добавить файл `LICENSE` (MIT), чтобы совпадал с README
- [x] Единый стиль и инструменты разработчика:
  - [x] `rustfmt.toml` (форматирование) и `.editorconfig`
  - [x] Предкоммит-хуки (например, `pre-commit`) с `cargo fmt`, `cargo clippy`, `cargo test`
- [x] Конфигурация окружения:
  - [x] Добавить `dotenvy` и автозагрузка `.env` в dev-режиме
  - [x] Шаблон `.env.example` (переменные: `BOT_TOKEN`, `PORT=5000`, `MODE=polling|webhook`)

Примеры команд:

```bash
cargo add proptest --dev
cargo add dotenvy
cargo fmt && cargo clippy -D warnings && cargo test
```

### Фаза 2 — CI/CD
- [x] GitHub Actions: `.github/workflows/ci.yml`
  - [x] Установка toolchain, кеш Cargo
  - [x] Шаги: `fmt --check`, `clippy -D warnings`, `test --all`
  - [ ] Безопасность: `cargo-audit`, `cargo-deny`
- [ ] Автообновления: `dependabot.yml` (Cargo + GitHub Actions)
- [ ] Релизы (опционально): `cargo-release` и публикация GitHub Releases по тегу

### Фаза 3 — Контейнеризация и поставка
- [x] `Dockerfile` (multi-stage):
  - [x] Сборка релизного бинаря
  - [x] Runtime-образ минимальный, запуск под non-root
  - [x] По возможности `rustls` вместо OpenSSL для упрощения доставки
- [ ] `docker-compose.yml` для локального запуска
- [ ] Публикация образа в GHCR из CI (по тегу/мэйнам)

Примеры команд:

```bash
docker build -t ghcr.io/<org>/telegram-dice-bot:local .
docker run --rm -e BOT_TOKEN=... -e PORT=5000 -p 5000:5000 ghcr.io/<org>/telegram-dice-bot:local
```

### Фаза 4 — Наблюдаемость и эксплуатация
- [ ] Логи: перейти на `tracing` + `tracing-subscriber`
  - [ ] Структурированные JSON-логи для продакшена, уровни через `RUST_LOG`
- [ ] Метрики Prometheus: эндпоинт `/metrics` (через `axum` + `prometheus`/`metrics`)
- [ ] Ошибки: интеграция Sentry (опционально)

### Фаза 5 — Инфраструктура (Kubernetes/Helm) — опционально
- [ ] Манифесты: `Deployment`, `Service`, `Ingress/Gateway`
- [ ] Пробы: `liveness/readiness` на `GET /health`
- [ ] Ресурсы и HPA, аннотации логирования
- [ ] Helm Chart для стандартного деплоя

### Фаза 6 — Функциональные улучшения бота
- [ ] Включить webhook-режим (фича уже подключена):
  - [ ] Роут в `axum` для приёма апдейтов, переключение режимов через `MODE`
  - [ ] Безопасная валидация токенов/секрета вебхука
- [ ] Использовать `teloxide::dispatching::dialogue` и существующий `DialogueState`
- [ ] Троттлинг/анти-спам адаптеры (опционально)

### Фаза 7 — Производительность и релизы — опционально
- [ ] Бенчмарки на `criterion`
- [ ] Сборка статического бинаря под `x86_64-unknown-linux-musl` для меньшего образа
- [ ] Автоматизация релизов (`cargo-release`), changelog

### Безопасность
- [ ] `cargo-audit` и `cargo-deny` в CI
- [ ] Хранение секретов только в секретах CI/деплой-платформы
- [ ] Проверка лицензий зависимостей (политика в `deny.toml`)

### Definition of Done (минимум)
- [ ] Unit/интеграционные тесты проходят локально и в CI
- [ ] CI: fmt, clippy, tests, audit/deny — зелёные
- [ ] Образ собирается и запускается локально, `/health` доступен
- [ ] Документация обновлена: README разделы по запуску (локально/Docker), переменные окружения

### Следующие шаги (я могу сделать сразу)
- [x] Добавить unit-тесты для `src/game.rs`
- [x] Создать `.github/workflows/ci.yml` (fmt, clippy, test)
- [x] Добавить `Dockerfile` и `.env.example`
