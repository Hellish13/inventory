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

# ER-диаграмма системы управления списанием товаров

```mermaid
erDiagram
    MANAGER {
        integer id PK "Идентификатор менеджера"
        varchar name "ФИО менеджера"
        varchar email "Email (уникальный)"
        varchar phone "Телефон"
        boolean is_active "Статус активности"
    }

    ADMIN {
        integer id PK "Идентификатор администратора"
        varchar name "ФИО администратора"
        varchar email "Email (уникальный)"
        varchar phone "Телефон"
    }

    PRODUCT {
        integer id PK "Идентификатор товара"
        varchar name "Наименование товара"
        text description "Описание товара"
        varchar category "Категория"
        decimal price "Цена"
        integer quantity "Количество на складе"
        varchar sku "Артикул (уникальный)"
    }

    WRITE_OFF_REQUEST {
        integer id PK "Идентификатор заявки"
        integer manager_id FK "ID менеджера"
        integer admin_id FK "ID администратора"
        date request_date "Дата создания"
        date approval_date "Дата утверждения"
        varchar status "Статус заявки"
        text reason "Причина списания"
        text notes "Примечания"
    }

    WRITE_OFF_ITEM {
        integer id PK "Идентификатор позиции"
        integer request_id FK "ID заявки"
        integer product_id FK "ID товара"
        integer quantity "Количество для списания"
        decimal unit_price "Цена за единицу"
    }

    MANAGER ||--o{ WRITE_OFF_REQUEST : creates
    ADMIN ||--o{ WRITE_OFF_REQUEST : approves
    WRITE_OFF_REQUEST ||--o{ WRITE_OFF_ITEM : contains
    PRODUCT ||--o{ WRITE_OFF_ITEM : included_in