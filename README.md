# Mini Beacon Node API

Русский | [English](#english)

Небольшой учебный REST API на Rust, моделирующий последовательность блоков. Сервис хранит блоки в памяти, проверяет базовые правила добавления нового блока и создаёт SHA-256 идентификатор следующего блока.

> Проект учебный: данные не сохраняются после перезапуска сервера.

## Возможности

- создание стартового (genesis) блока при запуске;
- добавление блока с проверкой родителя, порядка и размера данных;
- вычисление SHA-256 root для нового блока;
- получение текущей головы цепочки, отдельного блока и списка блоков;
- пагинация списка через `from_slot` и `limit`;
- endpoint состояния сервиса.

## Стек

| Технология | Назначение |
| --- | --- |
| Rust 2024 | язык и edition проекта |
| Tokio | асинхронный runtime и `RwLock` |
| Axum | HTTP-маршрутизация и извлечение параметров |
| Serde | сериализация и десериализация JSON |
| SHA-2 + hex | SHA-256 и hex-представление root |

## Запуск

```bash
cargo run
```

Сервер слушает `http://localhost:3000` (привязка: `0.0.0.0:3000`).

## API

| Метод | Путь | Описание |
| --- | --- | --- |
| `GET` | `/health` | Статус сервиса и количество блоков |
| `GET` | `/head` | Текущий последний блок |
| `GET` | `/blocks/{root}` | Поиск блока по его `parent_root` |
| `GET` | `/blocks?from_slot={n}&limit={n}` | Получение диапазона блоков |
| `POST` | `/blocks` | Валидация и добавление следующего блока |

### Формат блока

```json
{
  "slot": 1,
  "root": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "parent_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "proposer_index": 1,
  "body": "first block"
}
```

Для `POST /blocks` поле `slot` должно быть на единицу больше slot текущей головы, а `parent_root` — совпадать с `parent_root` текущей головы и иметь длину 66 символов (ожидаемый формат: `0x` + 64 hex-символа). Длина `body` ограничена 1024 символами. После проверки сервер рассчитывает новый root и возвращает его в ответе.

## Примеры запросов

Проверить состояние:

```bash
curl http://localhost:3000/health
```

Получить голову цепочки:

```bash
curl http://localhost:3000/head
```

Получить список блоков, начиная с нулевой позиции:

```bash
curl "http://localhost:3000/blocks?from_slot=0&limit=10"
```

Добавить первый блок после стартового:

```bash
curl -X POST http://localhost:3000/blocks \
  -H "Content-Type: application/json" \
  -d '{
    "slot": 1,
    "parent_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "proposer_index": 1,
    "body": "first block"
  }'
```

После создания блока можно передать его `root`, полученный в ответе `POST`, в запрос поиска:

```bash
curl http://localhost:3000/blocks/<root>
```

## Ответы об ошибках

API использует `406 Not Acceptable`, если формат данных неверен, родительский root не совпадает, нарушен порядок slot или тело блока слишком длинное. Если блок не найден, возвращается `404 Not Found`.

---

<a id="english"></a>

# English

A small educational REST API written in Rust that models a sequence of blocks. The service keeps blocks in memory, validates basic rules for adding a block, and creates a SHA-256 identifier for the next block.

> This is a learning project: all data is lost when the server restarts.

## Features

- creates a genesis block on startup;
- validates the parent, ordering, and payload size of a new block;
- calculates a SHA-256 root for the new block;
- exposes the chain head, a single block, and a list of blocks;
- supports list pagination through `from_slot` and `limit`;
- provides a health endpoint.

## Stack

| Technology | Purpose |
| --- | --- |
| Rust 2024 | project language and edition |
| Tokio | asynchronous runtime and `RwLock` |
| Axum | HTTP routing and request extractors |
| Serde | JSON serialization and deserialization |
| SHA-2 + hex | SHA-256 and hexadecimal root representation |

## Run

```bash
cargo run
```

The server listens on `http://localhost:3000` (binding: `0.0.0.0:3000`).

## API

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/health` | Service status and block count |
| `GET` | `/head` | Current chain head |
| `GET` | `/blocks/{root}` | Finds a block by its `parent_root` |
| `GET` | `/blocks?from_slot={n}&limit={n}` | Returns a block range |
| `POST` | `/blocks` | Validates and appends the next block |

### Block format

```json
{
  "slot": 1,
  "parent_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "proposer_index": 1,
  "body": "first block"
}
```

For `POST /blocks`, `slot` must be one greater than the current head slot; `parent_root` must equal the current head's `parent_root` and have a length of 66 characters (expected format: `0x` plus 64 hexadecimal characters). `body` is limited to 1024 characters. Once validation succeeds, the server calculates and returns the new root.

## Request examples

```bash
# Health check
curl http://localhost:3000/health

# Current head
curl http://localhost:3000/head

# Paginated block list
curl "http://localhost:3000/blocks?from_slot=0&limit=10"

# Append the first block after genesis
curl -X POST http://localhost:3000/blocks \
  -H "Content-Type: application/json" \
  -d '{
    "slot": 1,
    "parent_root": "0x0000000000000000000000000000000000000000000000000000000000000000",
    "proposer_index": 1,
    "body": "first block"
  }'

# Use the root returned by POST to find the block
curl http://localhost:3000/blocks/<root>
```

## Error responses

The API returns `406 Not Acceptable` for invalid data format, an unknown parent root, an invalid slot order, or an oversized block body. A missing block returns `404 Not Found`.
