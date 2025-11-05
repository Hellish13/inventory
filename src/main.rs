mod models;
mod database;
mod repositories;

use anyhow::Result;
use crate::models::*;
use crate::database::Database;
use crate::repositories::*;

fn main() -> Result<()> {
    // Инициализация базы данных
    let db = Database::new()?;
    db.init()?;

    // Репозитории
    let manager_repo = ManagerRepository::new(&db);
    let product_repo = ProductRepository::new(&db);
    let request_repo = WriteOffRequestRepository::new(&db);
    let item_repo = WriteOffItemRepository::new(&db);

    // Демонстрация работы с данными
    
    // 1. Получение всех менеджеров
    println!("=== Все менеджеры ===");
    let managers = manager_repo.get_all()?;
    for manager in managers {
        println!("{} ({})", manager.name, manager.email);
    }

    // 2. Получение всех товаров
    println!("\n=== Все товары ===");
    let products = product_repo.get_all()?;
    for product in products {
        println!("{} - {} руб. (остаток: {})", product.name, product.price, product.quantity);
    }

    // 3. Получение всех заявок
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

    // 4. Получение детальной информации о заявке
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

    // 5. Создание новой заявки
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
        product_id: 2, // Мышь Logitech
        quantity: 3,
        unit_price: 7999.99,
    };

    item_repo.create(&new_item)?;
    println!("Добавлен товар в заявку");

    // 6. Получение заявок конкретного менеджера
    println!("\n=== Заявки менеджера #2 ===");
    let manager_requests = request_repo.get_requests_by_manager(2)?;
    for req in manager_requests {
        println!("Заявка #{}: {}", req.id.unwrap(), req.reason);
    }

    Ok(())
}