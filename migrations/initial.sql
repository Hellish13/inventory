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
);

-- Тестовые данные
INSERT INTO manager (name, email, phone) VALUES 
('Иванов Петр Сергеевич', 'ivanov@company.com', '+7-999-123-45-67'),
('Сидорова Анна Владимировна', 'sidorova@company.com', '+7-999-123-45-68');

INSERT INTO admin (name, email, phone) VALUES 
('Кузнецов Алексей Иванович', 'kuznetsov@company.com', '+7-999-123-45-69'),
('Петрова Мария Дмитриевна', 'petrova@company.com', '+7-999-123-45-70');

INSERT INTO product (name, description, category, price, quantity, sku) VALUES 
('Ноутбук Dell XPS 13', '13-дюймовый бизнес-ноутбук', 'Электроника', 89999.99, 15, 'DLXPS13-001'),
('Мышь Logitech MX Master 3', 'Беспроводная мышь', 'Аксессуары', 7999.99, 30, 'LGMX3-002'),
('Монитор Samsung 27"', '4K монитор 27 дюймов', 'Электроника', 34999.99, 8, 'SAM27-003'),
('Клавиатура Keychron K2', 'Механическая клавиатура', 'Аксессуары', 5999.99, 25, 'KCK2-004');

INSERT INTO write_off_request (manager_id, admin_id, request_date, approval_date, status, reason, notes) VALUES 
(1, 1, '2024-01-15', '2024-01-16', 'approved', 'Бракованный товар', 'Не включается'),
(2, NULL, '2024-01-17', NULL, 'pending', 'Устаревшая модель', 'Требуется утилизация');

INSERT INTO write_off_item (request_id, product_id, quantity, unit_price) VALUES 
(1, 1, 1, 89999.99),
(1, 3, 2, 34999.99),
(2, 4, 5, 5999.99);