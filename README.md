# Система управления списанием товаров

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![SQLite](https://img.shields.io/badge/SQLite-3.x-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

Профессиональная система для автоматизации процесса списания товаров на предприятиях розничной торговли и складах. Приложение предоставляет полный цикл управления списаниями - от создания заявки до утверждения и исполнения.

## 🏗️ Архитектура системы

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