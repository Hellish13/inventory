mod models;
mod database;
mod repositories;

use anyhow::Result;
use std::io::{self, Write};
use crate::models::*;
use crate::database::Database;
use crate::repositories::*;

fn main() -> Result<()> {
    let db = Database::new()?;
    db.init()?;

    main_menu(&db)?;

    Ok(())
}

fn main_menu(db: &Database) -> Result<()> {
    loop {
        println!("\n=== СИСТЕМА УПРАВЛЕНИЯ СПИСАНИЕМ ТОВАРОВ ===");
        println!("1.  Просмотр данных");
        println!("2.  Добавление данных");
        println!("3.   Изменение данных"); 
        println!("4.   Удаление данных");
        println!("5.  Экспорт данных");
        println!("6.  Выход");
        println!("===========================================");

        print!("Выберите действие: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => view_data_menu(db)?,
            "2" => add_data_menu(db)?,
            "3" => update_data_menu(db)?,
            "4" => delete_data_menu(db)?,
            "5" => export_data(db)?,  
            "6" => {
                println!("До свидания!");
                break;
            }
            _ => println!("Неверный выбор!"),
        }
    }

    Ok(())
}

// МЕНЮ ПРОСМОТРА ДАННЫХ
fn view_data_menu(db: &Database) -> Result<()> {
    loop {
        println!("\n---  ПРОСМОТР ДАННЫХ ---");
        println!("1. Показать всех менеджеров");
        println!("2. Показать все товары");
        println!("3. Показать все заявки");
        println!("4. Показать заявку с деталями");
        println!("5. Назад в главное меню");
        println!("---------------------------");

        print!("Выберите действие: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => show_all_managers(db)?,
            "2" => show_all_products(db)?,
            "3" => show_all_requests(db)?,
            "4" => show_request_details(db)?,
            "5" => break,
            _ => println!("Неверный выбор!"),
        }
    }
    Ok(())
}

// МЕНЮ ДОБАВЛЕНИЯ ДАННЫХ
fn add_data_menu(db: &Database) -> Result<()> {
    loop {
        println!("\n--- ➕ ДОБАВЛЕНИЕ ДАННЫХ ---");
        println!("1. Добавить менеджера");
        println!("2. Добавить товар");
        println!("3. Создать заявку на списание");
        println!("4. Добавить товар в заявку");
        println!("5. Назад в главное меню");
        println!("----------------------------");

        print!("Выберите действие: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => add_manager_interactive(db)?,
            "2" => add_product_interactive(db)?,
            "3" => create_write_off_request_interactive(db)?,
            "4" => add_item_to_request_interactive(db)?,
            "5" => break,
            _ => println!("Неверный выбор!"),
        }
    }
    Ok(())
}

// МЕНЮ ИЗМЕНЕНИЯ ДАННЫХ
fn update_data_menu(db: &Database) -> Result<()> {
    loop {
        println!("\n---   ИЗМЕНЕНИЕ ДАННЫХ ---");
        println!("1. Изменить данные менеджера");
        println!("2. Изменить данные товара");
        println!("3. Изменить статус заявки");
        println!("4. Обновить количество товара");
        println!("5. Назад в главное меню");
        println!("---------------------------");

        print!("Выберите действие: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => update_manager_interactive(db)?,
            "2" => update_product_interactive(db)?,
            "3" => update_request_status_interactive(db)?,
            "4" => update_product_quantity_interactive(db)?,
            "5" => break,
            _ => println!("Неверный выбор!"),
        }
    }
    Ok(())
}

// МЕНЮ УДАЛЕНИЯ ДАННЫХ
fn delete_data_menu(db: &Database) -> Result<()> {
    loop {
        println!("\n---   УДАЛЕНИЕ ДАННЫХ ---");
        println!("1. Удалить менеджера");
        println!("2. Удалить товар");
        println!("3. Удалить заявку");
        println!("4. Удалить товар из заявки");
        println!("5. Назад в главное меню");
        println!("--------------------------");

        print!("Выберите действие: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => delete_manager_interactive(db)?,
            "2" => delete_product_interactive(db)?,
            "3" => delete_request_interactive(db)?,
            "4" => delete_item_interactive(db)?,
            "5" => break,
            _ => println!("Неверный выбор!"),
        }
    }
    Ok(())
}

// ПРОСМОТР ДАННЫХ
fn show_all_managers(db: &Database) -> Result<()> {
    let manager_repo = ManagerRepository::new(db);
    let managers = manager_repo.get_all()?;

    println!("\n---  СПИСОК МЕНЕДЖЕРОВ ---");
    for manager in managers {
        println!("ID: {} | {} | {} | {} | {}", 
            manager.id.unwrap(),
            manager.name,
            manager.email,
            manager.phone.unwrap_or("нет телефона".to_string()),
            if manager.is_active { "активен" } else { "неактивен" }
        );
    }
    Ok(())
}

fn show_all_products(db: &Database) -> Result<()> {
    let product_repo = ProductRepository::new(db);
    let products = product_repo.get_all()?;

    println!("\n---  СПИСОК ТОВАРОВ ---");
    for product in products {
        println!("ID: {} | {} | {} руб. | {} шт. | {}",
            product.id.unwrap(),
            product.name,
            product.price,
            product.quantity,
            product.category.unwrap_or("без категории".to_string())
        );
    }
    Ok(())
}

fn show_all_requests(db: &Database) -> Result<()> {
    let request_repo = WriteOffRequestRepository::new(db);
    let requests = request_repo.get_all()?;

    println!("\n---  СПИСОК ЗАЯВОК ---");
    for request in requests {
        let status = match request.status {
            RequestStatus::Pending => "ожидание",
            RequestStatus::Approved => "утверждено",
            RequestStatus::Rejected => "отклонено",
        };
        println!("ID: {} | Менеджер: {} | {} | {}",
            request.id.unwrap(),
            request.manager_id,
            request.reason,
            status
        );
    }
    Ok(())
}

fn show_request_details(db: &Database) -> Result<()> {
    print!("Введите ID заявки: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let request_id: i64 = input.trim().parse()?;

    let request_repo = WriteOffRequestRepository::new(db);
    if let Some(details) = request_repo.get_request_with_details(request_id)? {
        println!("\n---  ДЕТАЛИ ЗАЯВКИ #{} ---", request_id);
        println!("Менеджер: {}", details.manager.name);
        println!("Причина: {}", details.request.reason);
        println!("Дата создания: {}", details.request.request_date);
        println!("Статус: {}", match details.request.status {
            RequestStatus::Pending => "ожидание",
            RequestStatus::Approved => "утверждено",
            RequestStatus::Rejected => "отклонено",
        });

        if let Some(admin) = details.admin {
            println!("Утвердил: {}", admin.name);
        }

        println!("Товары для списания:");
        for item_with_product in details.items {
            let total = item_with_product.item.quantity as f64 * item_with_product.item.unit_price;
            println!("  - {}: {} шт. × {} руб. = {} руб.",
                item_with_product.product.name,
                item_with_product.item.quantity,
                item_with_product.item.unit_price,
                total
            );
        }
    } else {
        println!("Заявка с ID {} не найдена", request_id);
    }
    Ok(())
}

fn add_manager_interactive(db: &Database) -> Result<()> {
    println!("\n---  ДОБАВЛЕНИЕ МЕНЕДЖЕРА ---");

    print!("ФИО менеджера: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    print!("Email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;

    print!("Телефон (необязательно): ");
    io::stdout().flush()?;
    let mut phone = String::new();
    io::stdin().read_line(&mut phone)?;

    let manager = Manager {
        id: None,
        name: name.trim().to_string(),
        email: email.trim().to_string(),
        phone: if phone.trim().is_empty() { None } else { Some(phone.trim().to_string()) },
        is_active: true,
    };

    let manager_repo = ManagerRepository::new(db);
    let manager_id = manager_repo.create(&manager)?;
    println!("Менеджер добавлен с ID: {}", manager_id);

    Ok(())
}

fn add_product_interactive(db: &Database) -> Result<()> {
    println!("\n---  ДОБАВЛЕНИЕ ТОВАРА ---");

    print!("Название товара: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    print!("Описание: ");
    io::stdout().flush()?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;

    print!("Категория: ");
    io::stdout().flush()?;
    let mut category = String::new();
    io::stdin().read_line(&mut category)?;

    print!("Цена: ");
    io::stdout().flush()?;
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str)?;
    let price: f64 = price_str.trim().parse()?;

    print!("Количество: ");
    io::stdout().flush()?;
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str)?;
    let quantity: i32 = quantity_str.trim().parse()?;

    print!("SKU: ");
    io::stdout().flush()?;
    let mut sku = String::new();
    io::stdin().read_line(&mut sku)?;

    let product = Product {
        id: None,
        name: name.trim().to_string(),
        description: if description.trim().is_empty() { None } else { Some(description.trim().to_string()) },
        category: if category.trim().is_empty() { None } else { Some(category.trim().to_string()) },
        price,
        quantity,
        sku: sku.trim().to_string(),
    };

    let product_repo = ProductRepository::new(db);
    let product_id = product_repo.create(&product)?;
    println!("Товар добавлен с ID: {}", product_id);

    Ok(())
}

fn create_write_off_request_interactive(db: &Database) -> Result<()> {
    println!("\n---  СОЗДАНИЕ ЗАЯВКИ ---");

    show_all_managers(db)?;
    print!("ID менеджера: ");
    io::stdout().flush()?;
    let mut manager_id_str = String::new();
    io::stdin().read_line(&mut manager_id_str)?;
    let manager_id: i64 = manager_id_str.trim().parse()?;

    print!("Причина списания: ");
    io::stdout().flush()?;
    let mut reason = String::new();
    io::stdin().read_line(&mut reason)?;

    print!("Заметки (необязательно): ");
    io::stdout().flush()?;
    let mut notes = String::new();
    io::stdin().read_line(&mut notes)?;

    let request = WriteOffRequest {
        id: None,
        manager_id,
        admin_id: None,
        request_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        approval_date: None,
        status: RequestStatus::Pending,
        reason: reason.trim().to_string(),
        notes: if notes.trim().is_empty() { None } else { Some(notes.trim().to_string()) },
    };

    let request_repo = WriteOffRequestRepository::new(db);
    let request_id = request_repo.create(&request)?;
    println!("Заявка создана с ID: {}", request_id);

    Ok(())
}

fn add_item_to_request_interactive(db: &Database) -> Result<()> {
    println!("\n---  ДОБАВЛЕНИЕ ТОВАРА В ЗАЯВКУ ---");

    show_all_requests(db)?;
    print!("ID заявки: ");
    io::stdout().flush()?;
    let mut request_id_str = String::new();
    io::stdin().read_line(&mut request_id_str)?;
    let request_id: i64 = request_id_str.trim().parse()?;

    show_all_products(db)?;
    print!("ID товара: ");
    io::stdout().flush()?;
    let mut product_id_str = String::new();
    io::stdin().read_line(&mut product_id_str)?;
    let product_id: i64 = product_id_str.trim().parse()?;

    print!("Количество: ");
    io::stdout().flush()?;
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str)?;
    let quantity: i32 = quantity_str.trim().parse()?;

    let product_repo = ProductRepository::new(db);
    let product = product_repo.get_by_id(product_id)?
        .ok_or_else(|| anyhow::anyhow!("Товар не найден"))?;

    let item = WriteOffItem {
        id: None,
        request_id,
        product_id,
        quantity,
        unit_price: product.price,
    };

    let item_repo = WriteOffItemRepository::new(db);
    let item_id = item_repo.create(&item)?;
    println!("Товар добавлен в заявку с ID позиции: {}", item_id);

    Ok(())
}

// ИЗМЕНЕНИЕ ДАННЫХ
fn update_manager_interactive(db: &Database) -> Result<()> {
    println!("\n---   ИЗМЕНЕНИЕ МЕНЕДЖЕРА ---");

    show_all_managers(db)?;
    print!("ID менеджера для изменения: ");
    io::stdout().flush()?;
    let mut manager_id_str = String::new();
    io::stdin().read_line(&mut manager_id_str)?;
    let manager_id: i64 = manager_id_str.trim().parse()?;

    let manager_repo = ManagerRepository::new(db);
    let mut manager = manager_repo.get_by_id(manager_id)?
        .ok_or_else(|| anyhow::anyhow!("Менеджер не найден"))?;

    let phone_display = manager.phone.as_ref().map(|p| p.as_str()).unwrap_or("нет телефона");
    println!("Текущие данные: {} | {} | {}", 
        manager.name, manager.email, phone_display);

    print!("Новое ФИО (оставьте пустым чтобы не менять): ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    if !name.trim().is_empty() {
        manager.name = name.trim().to_string();
    }

    print!("Новый email (оставьте пустым чтобы не менять): ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    if !email.trim().is_empty() {
        manager.email = email.trim().to_string();
    }

    print!("Новый телефон (оставьте пустым чтобы не менять): ");
    io::stdout().flush()?;
    let mut phone = String::new();
    io::stdin().read_line(&mut phone)?;
    if !phone.trim().is_empty() {
        manager.phone = Some(phone.trim().to_string());
    }

    manager_repo.update(&manager)?;

    println!("Данные менеджера обновлены");

    Ok(())
}

fn update_product_interactive(db: &Database) -> Result<()> {
    println!("\n---   ИЗМЕНЕНИЕ ТОВАРА ---");

    show_all_products(db)?;
    print!("ID товара для изменения: ");
    io::stdout().flush()?;
    let mut product_id_str = String::new();
    io::stdin().read_line(&mut product_id_str)?;
    let product_id: i64 = product_id_str.trim().parse()?;

    let product_repo = ProductRepository::new(db);
    let mut product = product_repo.get_by_id(product_id)?
        .ok_or_else(|| anyhow::anyhow!("Товар не найден"))?;

    println!("Текущие данные: {} | {} руб. | {} шт.", 
        product.name, product.price, product.quantity);

    print!("Новое название (оставьте пустым чтобы не менять): ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    if !name.trim().is_empty() {
        product.name = name.trim().to_string();
    }

    print!("Новая цена (оставьте пустым чтобы не менять): ");
    io::stdout().flush()?;
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str)?;
    if !price_str.trim().is_empty() {
        product.price = price_str.trim().parse()?;
    }

    print!("Новое количество (оставьте пустым чтобы не менять): ");
    io::stdout().flush()?;
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str)?;
    if !quantity_str.trim().is_empty() {
        product.quantity = quantity_str.trim().parse()?;
    }

    product_repo.update(&product)?;

    println!("Данные товара обновлены");

    Ok(())
}

fn update_request_status_interactive(db: &Database) -> Result<()> {
    println!("\n--- ✏️  ИЗМЕНЕНИЕ СТАТУСА ЗАЯВКИ ---");

    show_all_requests(db)?;
    print!("ID заявки: ");
    io::stdout().flush()?;
    let mut request_id_str = String::new();
    io::stdin().read_line(&mut request_id_str)?;
    let request_id: i64 = request_id_str.trim().parse()?;

    println!("Выберите новый статус:");
    println!("1. Ожидание");
    println!("2. Утверждено");
    println!("3. Отклонено");
    print!("Ваш выбор: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let status = match choice.trim() {
        "1" => RequestStatus::Pending,
        "2" => RequestStatus::Approved,
        "3" => RequestStatus::Rejected,
        _ => {
            println!(" Неверный выбор статуса");
            return Ok(());
        }
    };

    let request_repo = WriteOffRequestRepository::new(db);
    request_repo.update_status(
        request_id,
        status,
        1, 
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    )?;

    println!("Статус заявки обновлен");

    Ok(())
}

fn update_product_quantity_interactive(db: &Database) -> Result<()> {
    println!("\n---   ОБНОВЛЕНИЕ КОЛИЧЕСТВА ТОВАРА ---");

    show_all_products(db)?;
    print!("ID товара: ");
    io::stdout().flush()?;
    let mut product_id_str = String::new();
    io::stdin().read_line(&mut product_id_str)?;
    let product_id: i64 = product_id_str.trim().parse()?;

    print!("Новое количество: ");
    io::stdout().flush()?;
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str)?;
    let quantity: i32 = quantity_str.trim().parse()?;

    let product_repo = ProductRepository::new(db);
    product_repo.update_quantity(product_id, quantity)?;

    println!("Количество товара обновлено");

    Ok(())
}

// УДАЛЕНИ ДАННЫХ
fn delete_manager_interactive(db: &Database) -> Result<()> {
    println!("\n---   УДАЛЕНИЕ МЕНЕДЖЕРА ---");

    show_all_managers(db)?;
    print!("ID менеджера для удаления: ");
    io::stdout().flush()?;
    let mut manager_id_str = String::new();
    io::stdin().read_line(&mut manager_id_str)?;
    let manager_id: i64 = manager_id_str.trim().parse()?;

    let request_repo = WriteOffRequestRepository::new(db);
    let manager_requests = request_repo.get_requests_by_manager(manager_id)?;
    
    if !manager_requests.is_empty() {
        println!("Нельзя удалить менеджера, у которого есть заявки!");
        println!("Сначала удалите или переназначьте {} заявок", manager_requests.len());
        return Ok(());
    }

    let manager_repo = ManagerRepository::new(db);
    manager_repo.delete(manager_id)?;

    println!("Менеджер удален");

    Ok(())
}

// Функция удаления товара
fn delete_product_interactive(db: &Database) -> Result<()> {
    println!("\n---   УДАЛЕНИЕ ТОВАРА ---");

    show_all_products(db)?;
    print!("ID товара для удаления: ");
    io::stdout().flush()?;
    let mut product_id_str = String::new();
    io::stdin().read_line(&mut product_id_str)?;
    let product_id: i64 = product_id_str.trim().parse()?;

    let product_repo = ProductRepository::new(db);
    
    let item_repo = WriteOffItemRepository::new(db);
    let conn = db.get_connection();
    
    let item_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM write_off_item WHERE product_id = ?",
        rusqlite::params![product_id],
        |row| row.get(0),
    )?;

    if item_count > 0 {
        println!("Нельзя удалить товар, который используется в заявках!");
        println!("Сначала удалите товар из {} заявок", item_count);
        return Ok(());
    }

    product_repo.delete(product_id)?;
    println!("Товар удален");

    Ok(())
}

fn delete_request_interactive(db: &Database) -> Result<()> {
    println!("\n---   УДАЛЕНИЕ ЗАЯВКИ ---");

    show_all_requests(db)?;
    print!("ID заявки для удаления: ");
    io::stdout().flush()?;
    let mut request_id_str = String::new();
    io::stdin().read_line(&mut request_id_str)?;
    let request_id: i64 = request_id_str.trim().parse()?;

    let request_repo = WriteOffRequestRepository::new(db);
    request_repo.delete(request_id)?;

    println!("Заявка и связанные позиции удалены");

    Ok(())
}

fn delete_item_interactive(db: &Database) -> Result<()> {
    println!("\n---   УДАЛЕНИЕ ТОВАРА ИЗ ЗАЯВКИ ---");

    print!("ID заявки: ");
    io::stdout().flush()?;
    let mut request_id_str = String::new();
    io::stdin().read_line(&mut request_id_str)?;
    let request_id: i64 = request_id_str.trim().parse()?;

    // товары в заявке
    let item_repo = WriteOffItemRepository::new(db);
    let items = item_repo.get_items_with_products(request_id)?;
    
    if items.is_empty() {
        println!("В заявке нет товаров");
        return Ok(());
    }

    println!("Товары в заявке:");
    for item in &items {
        println!("ID позиции: {} | {} | {} шт.",
            item.item.id.unwrap(),
            item.product.name,
            item.item.quantity
        );
    }

    print!("ID позиции для удаления: ");
    io::stdout().flush()?;
    let mut item_id_str = String::new();
    io::stdin().read_line(&mut item_id_str)?;
    let item_id: i64 = item_id_str.trim().parse()?;

    item_repo.delete(item_id)?;

    println!("Товар удален из заявки");

    Ok(())
}
fn export_data(db: &Database) -> Result<()> {
    println!("\n=== 📤 ЭКСПОРТ ДАННЫХ ===");
    
    let exporter = DataExporter::new(db);
    exporter.export_write_off_requests()?;
    
    Ok(())
}

// экспорт данных
struct DataExporter<'a> {
    db: &'a Database,
}

impl<'a> DataExporter<'a> {
    fn new(db: &'a Database) -> Self {
        DataExporter { db }
    }
    
    fn export_write_off_requests(&self) -> Result<()> {
        use std::fs;
        use std::path::Path;

        let out_dir = Path::new("out");
        if !out_dir.exists() {
            fs::create_dir_all(out_dir)?;
            println!(" Создана папка 'out'");
        }
        
        let requests_with_details = self.get_requests_with_details()?;
        println!("Получено {} заявок для экспорта", requests_with_details.len());
        
        self.export_to_json(&requests_with_details)?;
        
        self.export_to_csv(&requests_with_details)?;
        
        self.export_to_xml(&requests_with_details)?;
        
        self.export_to_yaml(&requests_with_details)?;
        
        println!("Все файлы успешно созданы в папке 'out/'");
        println!("Файлы: data.json, data.csv, data.xml, data.yaml");
        
        Ok(())
    }
    
    fn get_requests_with_details(&self) -> Result<Vec<WriteOffRequestWithDetails>> {
        let request_repo = WriteOffRequestRepository::new(self.db);
        let mut result = Vec::new();
        
        // Получаем все заявки
        let requests = request_repo.get_all()?;
        
        for request in requests {
            if let Some(details) = request_repo.get_request_with_details(request.id.unwrap())? {
                result.push(details);
            }
        }
        
        Ok(result)
    }
    
    fn export_to_json(&self, data: &[WriteOffRequestWithDetails]) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let json_data = serde_json::to_string_pretty(data)?;
        let mut file = File::create("out/data.json")?;
        file.write_all(json_data.as_bytes())?;
        
        println!("Данные экспортированы в data.json");
        Ok(())
    }
    
    fn export_to_csv(&self, data: &[WriteOffRequestWithDetails]) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let mut csv_data = String::new();
        
        csv_data.push_str("id,manager,admin,request_date,approval_date,status,reason,notes,items_count,total_amount\n");
        
        for details in data {
            let items_count = details.items.len();
            let total_amount: f64 = details.items.iter()
                .map(|item| item.item.quantity as f64 * item.item.unit_price)
                .sum();
            
            let admin_name = if let Some(admin) = &details.admin {
                &admin.name
            } else {
                "Не назначен"
            };
            
            let approval_date = if let Some(date) = &details.request.approval_date {
                date.as_str()
            } else {
                "Не утверждена"
            };
            
            let notes = if let Some(note) = &details.request.notes {
                note.replace(',', ";")
            } else {
                String::new()
            };
            
            csv_data.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{:.2}\n",
                details.request.id.unwrap(),
                details.manager.name.replace(',', ";"),
                admin_name.replace(',', ";"),
                details.request.request_date,
                approval_date,
                match details.request.status {
                    RequestStatus::Pending => "ожидание",
                    RequestStatus::Approved => "утверждено", 
                    RequestStatus::Rejected => "отклонено",
                },
                details.request.reason.replace(',', ";"),
                notes,
                items_count,
                total_amount
            ));
        }
        
        let mut file = File::create("out/data.csv")?;
        file.write_all(csv_data.as_bytes())?;
        
        println!("Данные экспортированы в data.csv");
        Ok(())
    }
    
    fn export_to_xml(&self, data: &[WriteOffRequestWithDetails]) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        use quick_xml::se::to_string;
        
        #[derive(serde::Serialize)]
        struct ExportData<'a> {
            #[serde(rename = "request")]
            requests: &'a [WriteOffRequestWithDetails],
        }
        
        let export_data = ExportData { requests: data };
        let xml_data = to_string(&export_data)?;
        
        let mut file = File::create("out/data.xml")?;
        file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
        file.write_all(xml_data.as_bytes())?;
        
        println!("Данные экспортированы в data.xml");
        Ok(())
    }
    
    fn export_to_yaml(&self, data: &[WriteOffRequestWithDetails]) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let yaml_data = serde_yaml::to_string(data)?;
        let mut file = File::create("out/data.yaml")?;
        file.write_all(yaml_data.as_bytes())?;
        
        println!("Данные экспортированы в data.yaml");
        Ok(())
    }
}