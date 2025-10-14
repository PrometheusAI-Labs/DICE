use axum::{http::StatusCode, response::Html, routing::get, Router};
use log::{error, info};
use std::net::SocketAddr;
use teloxide::prelude::*;

mod bot;
mod game;
mod state;

use bot::BotHandler;

#[tokio::main]
async fn main() {
    // Инициализация логгера
    env_logger::init();
    
    // Автозагрузка переменных окружения из .env в dev-режиме
    // Не паникуем, если файла нет (prod окружение)
    let _ = dotenvy::dotenv();

    info!("Запуск Telegram бота для игры в кубики");

    // Получение токена бота из переменных окружения
    let bot_token = std::env::var("BOT_TOKEN")
        .expect("BOT_TOKEN должен быть установлен в переменных окружения");

    info!("Подключение к Telegram API...");
    let bot = Bot::new(bot_token);
    
    // Создание обработчика бота
    let handler = BotHandler::new();
    
    // Получение порта из переменных окружения (по умолчанию 5000)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse::<u16>()
        .expect("PORT должен быть валидным номером порта");
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("HTTP сервер запускается на порту {}", port);

    // Создание роутера с health check endpoint
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check));

    // Запуск HTTP сервера для health check в отдельной задаче
    let server_handle = tokio::spawn(async move {
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                info!("Health check сервер запущен на {}", addr);
                if let Err(e) = axum::serve(listener, app).await {
                    error!("Ошибка HTTP сервера: {}", e);
                }
            }
            Err(e) => {
                error!("Не удалось привязать адрес {}: {}", addr, e);
            }
        }
    });

    // Запуск Telegram бота в основной задаче
    let bot_handle = tokio::spawn(async move {
        info!("Запуск Telegram бота...");
        Dispatcher::builder(bot, handler.schema())
            .dependencies(dptree::deps![])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        info!("Telegram бот завершился");
    });

    // Даем время серверу запуститься
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    info!("Приложение запущено. HTTP сервер работает на порту {}", port);

    // Ожидание завершения любой из задач
    tokio::select! {
        result = server_handle => {
            match result {
                Ok(_) => info!("HTTP сервер завершился"),
                Err(e) => error!("Ошибка в HTTP сервере: {}", e),
            }
        },
        result = bot_handle => {
            match result {
                Ok(_) => info!("Telegram бот завершился"),
                Err(e) => error!("Ошибка в Telegram боте: {}", e),
            }
        }
    }
}

// Health check endpoint для Autoscale deployment
async fn health_check() -> Result<Html<&'static str>, StatusCode> {
    Ok(Html("<h1>Telegram Dice Bot is running</h1><p>Status: OK</p>"))
}
