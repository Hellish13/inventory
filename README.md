# Система учёта инвентаря

База данных на **Rust + SQLite**.  
Учёт сотрудников, инвентаря и истории возвратов.

---

## ER‑диаграмма базы данных

### Логическая модель

```plantuml
@startuml
entity "Сотрудник" as employee {
  * employee_full_name : TEXT <<PK>>
  --
  держит
}

entity "Инвентарь" as inventory {
  * inventory_num : INTEGER <<PK>>
  --
  name : TEXT
  status : TEXT
  current_holder : TEXT <<FK>>
  --
  возвращает
}

entity "Возврат" as return {
  * inventory_num : INTEGER <<FK, PK>>
  * return_date : TEXT <<PK>>
  --
  client_full_name : TEXT <<FK>>
  condition : TEXT
}

employee ||--o{ inventory : "держит"
employee ||--o{ return : "возвращает"
inventory ||--o{ return : "возвращается"
@enduml