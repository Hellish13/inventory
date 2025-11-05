mod models;
mod database;
mod repositories;

use anyhow::Result;
use crate::models::*;
use crate::database::Database;
use crate::repositories::*;

fn interactive_data_entry(db: &Database) -> Result<()> {
    use std::io::{self, Write};
    
    println!("\n=== ИНТЕРАКТИВНОЕ ДОБАВЛЕНИЕ ДАННЫХ ===");
    
    let manager_repo = ManagerRepository::new(db);
    let product_repo = ProductRepository::new(db);
    
    // Добавление менеджера
    println!("\n--- Добавление менеджера ---");
    
    print!("Введите ФИО менеджера: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();
    
    print!("Введите email менеджера: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim().to_string();
    
    print!("Введите телефон менеджера (необязательно): ");
    io::stdout().flush()?;
    let mut phone = String::new();
    io::stdin().read_line(&mut phone)?;
    let phone = if phone.trim().is_empty() {
        None
    } else {
        Some(phone.trim().to_string())
    };
    
    let manager = Manager {
        id: None,
        name,
        email,
        phone,
        is_active: true,
    };
    
    let manager_id = manager_repo.create(&manager)?;
    println!("✅ Менеджер добавлен с ID: {}", manager_id);
    
    // Добавление товара
    println!("\n--- Добавление товара ---");
    
    print!("Введите название товара: ");
    io::stdout().flush()?;
    let mut product_name = String::new();
    io::stdin().read_line(&mut product_name)?;
    let product_name = product_name.trim().to_string();
    
    print!("Введите описание товара: ");
    io::stdout().flush()?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description.trim().to_string())
    };
    
    print!("Введите категорию товара: ");
    io::stdout().flush()?;
    let mut category = String::new();
    io::stdin().read_line(&mut category)?;
    let category = if category.trim().is_empty() {
        None
    } else {
        Some(category.trim().to_string())
    };
    
    print!("Введите цену товара: ");
    io::stdout().flush()?;
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str)?;
    let price: f64 = price_str.trim().parse()?;
    
    print!("Введите количество товара: ");
    io::stdout().flush()?;
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str)?;
    let quantity: i32 = quantity_str.trim().parse()?;
    
    print!("Введите SKU товара: ");
    io::stdout().flush()?;
    let mut sku = String::new();
    io::stdin().read_line(&mut sku)?;
    let sku = sku.trim().to_string();
    
    let product = Product {
        id: None,
        name: product_name,
        description,
        category,
        price,
        quantity,
        sku,
    };
    
    let product_id = product_repo.create(&product)?;
    println!("✅ Товар добавлен с ID: {}", product_id);
    
    println!("\n=== ДАННЫЕ УСПЕШНО ДОБАВЛЕНЫ! ===");
    
    Ok(())
}

fn main() -> Result<()> {
    // Инициализация базы данных
    let db = Database::new()?;
    db.init()?;
    interactive_data_entry(&db)?;
    // Репозитории
    let manager_repo = ManagerRepository::new(&db);
    let product_repo = ProductRepository::new(&db);
    let request_repo = WriteOffRequestRepository::new(&db);
    let item_repo = WriteOffItemRepository::new(&db);

    // Демонстрация работы с данными
    
    // Получение всех менеджеров
    println!("=== Все менеджеры ===");
    let managers = manager_repo.get_all()?;
    for manager in managers {
        println!("{} ({})", manager.name, manager.email);
    }

    // Получение всех товаров
    println!("\n=== Все товары ===");
    let products = product_repo.get_all()?;
    for product in products {
        println!("{} - {} руб. (остаток: {})", product.name, product.price, product.quantity);
    }

    // Получение всех заявок
    println!("\n=== Все заявки на списание ===");
    let requests = request_repo.get_all()?;
    for request in requests {
        let status = match request.status {
            RequestStatus::Pending => "ожидание",
            RequestStatus::Approved => "утверждено",
            RequestStatus::Rejected => "отклонено",
        };
        println!("Заявка #{}: {} - {}", 
                 request.id.unwrap(), request.reason, status);
    }

    // Получение информации о заявке
    println!("\n=== Детали заявки #1 ===");
    if let Some(details) = request_repo.get_request_with_details(1)? {
        println!("Заявка от {}: {}", details.manager.name, details.request.reason);
        println!("Статус: {}", match details.request.status {
            RequestStatus::Pending => "ожидание",
            RequestStatus::Approved => "утверждено",
            RequestStatus::Rejected => "отклонено",
        });
        
        println!("Товары для списания:");
        for item_with_product in details.items {
            println!("  - {}: {} шт. x {} руб.", 
                     item_with_product.product.name, 
                     item_with_product.item.quantity,
                     item_with_product.item.unit_price);
        }
    }

    // Создание заявки
    println!("\n=== Создание новой заявки ===");
    let new_request = WriteOffRequest {
        id: None,
        manager_id: 2,
        admin_id: None,
        request_date: "2024-01-18".to_string(),
        approval_date: None,
        status: RequestStatus::Pending,
        reason: "Повреждение при транспортировке".to_string(),
        notes: Some("Треснул экран".to_string()),
    };

    let request_id = request_repo.create(&new_request)?;
    println!("Создана новая заявка #{}", request_id);

    // Добавление товара в заявку
    let new_item = WriteOffItem {
        id: None,
        request_id,
        product_id: 2, 
        quantity: 3,
        unit_price: 7999.99,
    };

    item_repo.create(&new_item)?;
    println!("Добавлен товар в заявку");

    // Получение заявок менеджера
    println!("\n=== Заявки менеджера #2 ===");
    let manager_requests = request_repo.get_requests_by_manager(2)?;
    for req in manager_requests {
        println!("Заявка #{}: {}", req.id.unwrap(), req.reason);
    }

    Ok(())
}