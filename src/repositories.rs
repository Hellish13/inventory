use rusqlite::params;
use anyhow::{anyhow, Context, Result};
use crate::models::*;
use crate::database::Database;

pub struct ManagerRepository<'a> {
    db: &'a Database,
}

impl<'a> ManagerRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        ManagerRepository { db }
    }

    pub fn create(&self, manager: &Manager) -> Result<i64> {
        let conn = self.db.get_connection();
        let sql = "INSERT INTO manager (name, email, phone, is_active) VALUES (?, ?, ?, ?)";
        
        conn.execute(
            sql,
            params![manager.name, manager.email, manager.phone, manager.is_active],
        )
        .context("Failed to create manager")?;
        
        Ok(conn.last_insert_rowid())
    }

    pub fn get_all(&self) -> Result<Vec<Manager>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare("SELECT id, name, email, phone, is_active FROM manager")?;
        
        let managers = stmt.query_map([], |row| {
            Ok(Manager {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                email: row.get(2)?,
                phone: row.get(3)?,
                is_active: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(managers)
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<Manager>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare("SELECT id, name, email, phone, is_active FROM manager WHERE id = ?")?;
        
        let manager = stmt.query_row(params![id], |row| {
            Ok(Manager {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                email: row.get(2)?,
                phone: row.get(3)?,
                is_active: row.get(4)?,
            })
        });
        
        match manager {
            Ok(m) => Ok(Some(m)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

pub struct ProductRepository<'a> {
    db: &'a Database,
}

impl<'a> ProductRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        ProductRepository { db }
    }

    pub fn create(&self, product: &Product) -> Result<i64> {
        let conn = self.db.get_connection();
        let sql = "INSERT INTO product (name, description, category, price, quantity, sku) VALUES (?, ?, ?, ?, ?, ?)";
        
        conn.execute(
            sql,
            params![
                product.name,
                product.description,
                product.category,
                product.price,
                product.quantity,
                product.sku
            ],
        )
        .context("Failed to create product")?;
        
        Ok(conn.last_insert_rowid())
    }

    pub fn get_all(&self) -> Result<Vec<Product>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare("SELECT id, name, description, category, price, quantity, sku FROM product")?;
        
        let products = stmt.query_map([], |row| {
            Ok(Product {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                category: row.get(3)?,
                price: row.get(4)?,
                quantity: row.get(5)?,
                sku: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(products)
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<Product>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare("SELECT id, name, description, category, price, quantity, sku FROM product WHERE id = ?")?;
        
        let product = stmt.query_row(params![id], |row| {
            Ok(Product {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                description: row.get(2)?,
                category: row.get(3)?,
                price: row.get(4)?,
                quantity: row.get(5)?,
                sku: row.get(6)?,
            })
        });
        
        match product {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn update_quantity(&self, id: i64, new_quantity: i32) -> Result<()> {
        let conn = self.db.get_connection();
        conn.execute(
            "UPDATE product SET quantity = ? WHERE id = ?",
            params![new_quantity, id],
        )
        .context("Failed to update product quantity")?;
        
        Ok(())
    }
}

pub struct WriteOffRequestRepository<'a> {
    db: &'a Database,
}

impl<'a> WriteOffRequestRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        WriteOffRequestRepository { db }
    }

    pub fn create(&self, request: &WriteOffRequest) -> Result<i64> {
        let conn = self.db.get_connection();
        let sql = "INSERT INTO write_off_request (manager_id, admin_id, request_date, approval_date, status, reason, notes) VALUES (?, ?, ?, ?, ?, ?, ?)";
        
        conn.execute(
            sql,
            params![
                request.manager_id,
                request.admin_id,
                request.request_date,
                request.approval_date,
                request.status.to_string(),
                request.reason,
                request.notes
            ],
        )
        .context("Failed to create write-off request")?;
        
        Ok(conn.last_insert_rowid())
    }

    pub fn get_all(&self) -> Result<Vec<WriteOffRequest>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, manager_id, admin_id, request_date, approval_date, status, reason, notes 
             FROM write_off_request"
        )?;
        
        let requests = stmt.query_map([], |row| {
            Ok(WriteOffRequest {
                id: Some(row.get(0)?),
                manager_id: row.get(1)?,
                admin_id: row.get(2)?,
                request_date: row.get(3)?,
                approval_date: row.get(4)?,
                status: row.get::<_, String>(5)?.into(),
                reason: row.get(6)?,
                notes: row.get(7)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(requests)
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<WriteOffRequest>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, manager_id, admin_id, request_date, approval_date, status, reason, notes 
             FROM write_off_request WHERE id = ?"
        )?;
        
        let request = stmt.query_row(params![id], |row| {
            Ok(WriteOffRequest {
                id: Some(row.get(0)?),
                manager_id: row.get(1)?,
                admin_id: row.get(2)?,
                request_date: row.get(3)?,
                approval_date: row.get(4)?,
                status: row.get::<_, String>(5)?.into(),
                reason: row.get(6)?,
                notes: row.get(7)?,
            })
        });
        
        match request {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn update_status(&self, id: i64, status: RequestStatus, admin_id: i64, approval_date: &str) -> Result<()> {
        let conn = self.db.get_connection();
        conn.execute(
            "UPDATE write_off_request SET status = ?, admin_id = ?, approval_date = ? WHERE id = ?",
            params![status.to_string(), admin_id, approval_date, id],
        )
        .context("Failed to update request status")?;
        
        Ok(())
    }

    // Сложные запросы для связанных данных
    pub fn get_requests_by_manager(&self, manager_id: i64) -> Result<Vec<WriteOffRequest>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, manager_id, admin_id, request_date, approval_date, status, reason, notes 
             FROM write_off_request WHERE manager_id = ?"
        )?;
        
        let requests = stmt.query_map(params![manager_id], |row| {
            Ok(WriteOffRequest {
                id: Some(row.get(0)?),
                manager_id: row.get(1)?,
                admin_id: row.get(2)?,
                request_date: row.get(3)?,
                approval_date: row.get(4)?,
                status: row.get::<_, String>(5)?.into(),
                reason: row.get(6)?,
                notes: row.get(7)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(requests)
    }

    pub fn get_request_with_details(&self, request_id: i64) -> Result<Option<WriteOffRequestWithDetails>> {
        let conn = self.db.get_connection();
        
        // Получаем заявку
        let request = match self.get_by_id(request_id)? {
            Some(r) => r,
            None => return Ok(None),
        };

        // Получаем менеджера
        let manager_repo = ManagerRepository::new(self.db);
        let manager = manager_repo.get_by_id(request.manager_id)?
            .ok_or_else(|| anyhow!("Manager not found"))?;

        // Получаем администратора (если есть)
        let admin = if let Some(admin_id) = request.admin_id {
            let admin_repo = AdminRepository::new(self.db);
            admin_repo.get_by_id(admin_id).ok().flatten()
        } else {
            None
        };

        // Получаем позиции списания с товарами
        let items_repo = WriteOffItemRepository::new(self.db);
        let items = items_repo.get_items_with_products(request_id)?;

        Ok(Some(WriteOffRequestWithDetails {
            request,
            manager,
            admin,
            items,
        }))
    }
}

pub struct WriteOffItemRepository<'a> {
    db: &'a Database,
}

impl<'a> WriteOffItemRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        WriteOffItemRepository { db }
    }

    pub fn create(&self, item: &WriteOffItem) -> Result<i64> {
        let conn = self.db.get_connection();
        let sql = "INSERT INTO write_off_item (request_id, product_id, quantity, unit_price) VALUES (?, ?, ?, ?)";
        
        conn.execute(
            sql,
            params![item.request_id, item.product_id, item.quantity, item.unit_price],
        )
        .context("Failed to create write-off item")?;
        
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_request_id(&self, request_id: i64) -> Result<Vec<WriteOffItem>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT id, request_id, product_id, quantity, unit_price 
             FROM write_off_item WHERE request_id = ?"
        )?;
        
        let items = stmt.query_map(params![request_id], |row| {
            Ok(WriteOffItem {
                id: Some(row.get(0)?),
                request_id: row.get(1)?,
                product_id: row.get(2)?,
                quantity: row.get(3)?,
                unit_price: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(items)
    }

    pub fn get_items_with_products(&self, request_id: i64) -> Result<Vec<WriteOffItemWithProduct>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare(
            "SELECT wi.id, wi.request_id, wi.product_id, wi.quantity, wi.unit_price,
                    p.id, p.name, p.description, p.category, p.price, p.quantity, p.sku
             FROM write_off_item wi
             JOIN product p ON wi.product_id = p.id
             WHERE wi.request_id = ?"
        )?;
        
        let items = stmt.query_map(params![request_id], |row| {
            let item = WriteOffItem {
                id: Some(row.get(0)?),
                request_id: row.get(1)?,
                product_id: row.get(2)?,
                quantity: row.get(3)?,
                unit_price: row.get(4)?,
            };
            
            let product = Product {
                id: Some(row.get(5)?),
                name: row.get(6)?,
                description: row.get(7)?,
                category: row.get(8)?,
                price: row.get(9)?,
                quantity: row.get(10)?,
                sku: row.get(11)?,
            };
            
            Ok(WriteOffItemWithProduct { item, product })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
        
        Ok(items)
    }
}

pub struct AdminRepository<'a> {
    db: &'a Database,
}

impl<'a> AdminRepository<'a> {
    pub fn new(db: &'a Database) -> Self {
        AdminRepository { db }
    }

    pub fn get_by_id(&self, id: i64) -> Result<Option<Admin>> {
        let conn = self.db.get_connection();
        let mut stmt = conn.prepare("SELECT id, name, email, phone FROM admin WHERE id = ?")?;
        
        let admin = stmt.query_row(params![id], |row| {
            Ok(Admin {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                email: row.get(2)?,
                phone: row.get(3)?,
            })
        });
        
        match admin {
            Ok(a) => Ok(Some(a)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn get_all(&self) -> Result<Vec<Admin>> {
        let _conn = self.db.get_connection();
        // Реализация получения всех администраторов
        // Пока возвращаем пустой вектор для демонстрации
        Ok(Vec::new())
    }
}