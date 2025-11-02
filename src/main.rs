// src/main.rs
use rusqlite::{Connection, Result, params};

fn main() -> Result<()> {
    // 2.1 – открываем (или создаём) файл БД
    let conn = Connection::open("inventory.db")?;

    // 2.2 – создаём таблицы
    create_tables(&conn)?;

    // 2.3 – вставляем тестовые данные
    insert_test_data(&conn)?;

    println!("База данных успешно создана и заполнена!");
    Ok(())
}

// -------------------------------------------------
// 2.2 CREATE TABLE
// -------------------------------------------------
fn create_tables(conn: &Connection) -> Result<()> {
    // Таблица Сотрудник
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Сотрудник (
            employee_full_name TEXT PRIMARY KEY
        )",
        [],
    )?;

    // Таблица Инвентарь
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Инвентарь (
            inventory_num   INTEGER PRIMARY KEY,
            name            TEXT    NOT NULL,
            status          TEXT    NOT NULL,
            current_holder  TEXT,
            FOREIGN KEY (current_holder) REFERENCES Сотрудник(employee_full_name)
        )",
        [],
    )?;

    // Таблица Возврат
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Возврат (
            inventory_num   INTEGER,
            client_full_name TEXT,
            return_date     TEXT    NOT NULL,
            condition       TEXT    NOT NULL,
            PRIMARY KEY (inventory_num, return_date),
            FOREIGN KEY (inventory_num)   REFERENCES Инвентарь(inventory_num),
            FOREIGN KEY (client_full_name) REFERENCES Сотрудник(employee_full_name)
        )",
        [],
    )?;

    println!("Таблицы созданы.");
    Ok(())
}

// -------------------------------------------------
// 2.3 INSERT INTO (тестовые данные)
// -------------------------------------------------
fn insert_test_data(conn: &Connection) -> Result<()> {
    // Сотрудники
    let employees = [
        "Иванов Иван Иванович",
        "Петров Пётр Петрович",
        "Сидорова Анна Михайловна",
    ];
    for name in employees {
        conn.execute(
            "INSERT OR IGNORE INTO Сотрудник (employee_full_name) VALUES (?1)",
            params![name],
        )?;
    }

    // Инвентарь
    let inventory = [
        (1, "Монитор Dell 24\"", "на_складе", None::<String>),
        (2, "Клавиатура Logitech", "у_сотрудника", Some("Иванов Иван Иванович".to_string())),
        (3, "Мышь A4Tech", "повреждено", None::<String>),
        (4, "Ноутбук Lenovo", "у_сотрудника", Some("Петров Пётр Петрович".to_string())),
    ];
    for (num, name, status, holder) in inventory {
        conn.execute(
            "INSERT OR REPLACE INTO Инвентарь
             (inventory_num, name, status, current_holder)
             VALUES (?1, ?2, ?3, ?4)",
            params![num, name, status, holder],
        )?;
    }

    // Возвраты
    let returns = [
        (2, "Иванов Иван Иванович", "2025-03-15 14:30:00", "исправно"),
        (4, "Петров Пётр Петрович", "2025-04-01 09:15:00", "царапина на корпусе"),
    ];
    for (num, client, date, cond) in returns {
        conn.execute(
            "INSERT INTO Возврат
             (inventory_num, client_full_name, return_date, condition)
             VALUES (?1, ?2, ?3, ?4)",
            params![num, client, date, cond],
        )?;
    }

    println!("Тестовые данные добавлены.");
    Ok(())
}