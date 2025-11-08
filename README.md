# Система управления списанием товаров

1. Предметная область
Система учитывает и отслеживает:

Товары на складе (наименование, описание, категория, цена, количество, артикул)

Сотрудников (менеджеров и администраторов с контактными данными)

Процесс списания (заявки, причины списания, статусы, даты)

Историю операций (кто, когда, что и почему списал)

2. Цели системы
Решаемые задачи:

Автоматизация процесса списания товаров

Контроль списаний

Учет остатков товаров на складе

Польза для пользователей:

Сокращение времени обработки заявок

Уменьшение ошибок при списании

3. Пользователи и роли
 Менеджер:

Создает заявки на списание

Указывает причины и товары

Просматривает историю своих заявок

 Администратор:

Утверждает/отклоняет заявки

Управляет товарами и сотрудниками

Видит все заявки в системе

## Архитектура системы

### Модель данных

```mermaid
erDiagram
    MANAGER {
        integer id PK
        varchar name
        varchar email
        varchar phone
        boolean is_active
    }

    ADMIN {
        integer id PK
        varchar name
        varchar email
        varchar phone
    }

    PRODUCT {
        integer id PK
        varchar name
        text description
        varchar category
        decimal price
        integer quantity
        varchar sku
    }

    WRITE_OFF_REQUEST {
        integer id PK
        integer manager_id FK
        integer admin_id FK
        date request_date
        date approval_date
        varchar status
        text reason
        text notes
    }

    WRITE_OFF_ITEM {
        integer id PK
        integer request_id FK
        integer product_id FK
        integer quantity
        decimal unit_price
    }

    MANAGER ||--o{ WRITE_OFF_REQUEST : creates
    ADMIN ||--o{ WRITE_OFF_REQUEST : approves
    WRITE_OFF_REQUEST ||--o{ WRITE_OFF_ITEM : contains
    PRODUCT ||--o{ WRITE_OFF_ITEM : included_in