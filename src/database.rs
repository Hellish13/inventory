use rusqlite::Connection;
use anyhow::Context;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> anyhow::Result<Self> {
        let conn = Connection::open("write_off_system.db")?;
        Ok(Database { conn })
    }

    pub fn init(&self) -> anyhow::Result<()> {
  
        let tables_sql = "
        -- Создание таблицы менеджеров
        CREATE TABLE IF NOT EXISTS manager (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            phone TEXT,
            is_active BOOLEAN DEFAULT TRUE
        );

        -- Создание таблицы администраторов
        CREATE TABLE IF NOT EXISTS admin (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            phone TEXT
        );

        -- Создание таблицы товаров
        CREATE TABLE IF NOT EXISTS product (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            category TEXT,
            price REAL NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 0,
            sku TEXT UNIQUE NOT NULL
        );

        -- Создание таблицы заявок на списание
        CREATE TABLE IF NOT EXISTS write_off_request (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            manager_id INTEGER NOT NULL,
            admin_id INTEGER,
            request_date TEXT NOT NULL,
            approval_date TEXT,
            status TEXT NOT NULL DEFAULT 'pending',
            reason TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (manager_id) REFERENCES manager(id),
            FOREIGN KEY (admin_id) REFERENCES admin(id)
        );

        -- Создание таблицы позиций списания
        CREATE TABLE IF NOT EXISTS write_off_item (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            unit_price REAL NOT NULL,
            FOREIGN KEY (request_id) REFERENCES write_off_request(id),
            FOREIGN KEY (product_id) REFERENCES product(id)
        );";
        
        self.conn.execute_batch(tables_sql)
            .context("Failed to create tables")?;
        
        self.insert_test_data()?;
        
        println!("Database initialized successfully");
        Ok(())
    }

    fn insert_test_data(&self) -> anyhow::Result<()> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM manager",
            [],
            |row| row.get(0)
    )?;
    
    if count > 0 {
        return Ok(());
    }

    // Тестовые данные - каждый INSERT как отдельная операция
    let test_data_sql = "
        INSERT INTO manager (name, email, phone) VALUES 
        ('Тимур Леонов Тимурович', 'sky@mail.ru', '+7-999-322-55-55');
        
        INSERT INTO admin (name, email, phone) VALUES 
        ('Иван Шурпатов Иванович', 'lowtab@mail.ru', '+7-999-123-55-69');
        
        INSERT INTO product (name, description, category, price, quantity, sku) VALUES 
        ('Гриф', '20кг', 'Тяжелая атлетика', 4000, 5, '001'),
        ('Гиря', '24кг', 'Тяжелая атлетика', 2000, 6, '002'),
        ('Резинка', '16-39кг 208см', 'Легкая атлетика', 500, 8, '003');
        
        INSERT INTO write_off_request (manager_id, admin_id, request_date, approval_date, status, reason, notes) VALUES 
        (1, 1, '2024-01-15', '2024-01-16', 'approved', 'Бракованный товар', 'Порванный'),
        (1, NULL, '2024-01-17', NULL, 'pending', 'Устаревший', 'Утилизация');
        
        INSERT INTO write_off_item (request_id, product_id, quantity, unit_price) VALUES 
        (1, 1, 1, 4000),
        (1, 3, 2, 500),
        (2, 2, 5, 2000);";
    
    self.conn.execute_batch(test_data_sql)
        .context("Failed to insert test data")?;
    
    println!("Test data inserted successfully");
    Ok(())
}

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}