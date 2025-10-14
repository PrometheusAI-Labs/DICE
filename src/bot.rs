use log::{error, info};
use teloxide::{
    dispatching::UpdateHandler,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
    utils::command::BotCommands,
    RequestError,
};

use crate::game::DiceGame;
use crate::state::{EvenOddChoice, HighLowChoice, GuessOneChoice};

/// Команды бота
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Начать игру")]
    Start,
    #[command(description = "Помощь")]
    Help,
    #[command(description = "Играть в кубики")]
    Play,
}

pub struct BotHandler;

impl BotHandler {
    pub fn new() -> Self {
        Self
    }

    /// Создание схемы обработчика
    pub fn schema(&self) -> UpdateHandler<RequestError> {
        use dptree::case;

        let command_handler = teloxide::filter_command::<Command, _>()
            .branch(case![Command::Start].endpoint(Self::start_command))
            .branch(case![Command::Help].endpoint(Self::help_command))
            .branch(case![Command::Play].endpoint(Self::play_command));

        let callback_handler = Update::filter_callback_query()
            .endpoint(Self::handle_callback);

        let message_handler = Update::filter_message()
            .branch(command_handler)
            .branch(dptree::endpoint(Self::handle_message));

        dptree::entry()
            .branch(message_handler)
            .branch(callback_handler)
    }

    /// Обработчик команды /start
    async fn start_command(bot: Bot, msg: Message) -> ResponseResult<()> {
        info!("Пользователь {} начал работу с ботом", msg.chat.id);
        
        let text = "🎲 Привет! Добро пожаловать в игру с кубиками!\n\n\
                   Я предлагаю вам угадать результат броска кубика.\n\
                   Доступны четыре варианта игры:\n\n\
                   🔵 Четное/Нечетное - угадайте четность числа\n\
                   🔴 Больше/Меньше 3.5 - угадайте диапазон\n\
                   🎯 Точное число - угадайте конкретное число\n\
                   🎱 Угадать единицу - выпадет ли единица\n\n\
                   Используйте /play чтобы начать игру!";

        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;
        
        Ok(())
    }

    /// Обработчик команды /help
    async fn help_command(bot: Bot, msg: Message) -> ResponseResult<()> {
        let text = "🎮 <b>Помощь по игре в кубики</b>\n\n\
                   <b>Доступные команды:</b>\n\
                   /start - начать работу с ботом\n\
                   /play - начать новую игру\n\
                   /help - показать эту справку\n\n\
                   <b>Варианты игры:</b>\n\n\
                   🔵 <b>Четное/Нечетное</b>\n\
                   Угадайте, будет ли результат четным или нечетным числом\n\n\
                   🔴 <b>Больше/Меньше 3.5</b>\n\
                   Угадайте, будет ли результат больше 3.5 (4-6) или меньше 3.5 (1-3)\n\n\
                   🎯 <b>Точное число</b>\n\
                   Угадайте конкретное число от 1 до 6\n\n\
                   ⚀ <b>Угадать единицу</b>\n\
                   Угадайте, выпадет ли на кубике единица";

        bot.send_message(msg.chat.id, text)
            .parse_mode(ParseMode::Html)
            .await?;
        
        Ok(())
    }

    /// Обработчик команды /play
    async fn play_command(bot: Bot, msg: Message) -> ResponseResult<()> {
        info!("Пользователь {} начал новую игру", msg.chat.id);
        Self::show_game_selection(&bot, msg.chat.id).await
    }

    /// Отображение выбора типа игры
    async fn show_game_selection(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback("🔵 Четное/Нечетное", "game_even_odd")],
            vec![InlineKeyboardButton::callback("🔴 Больше/Меньше 3.5", "game_high_low")],
            vec![InlineKeyboardButton::callback("🎯 Точное число", "game_exact")],
            vec![InlineKeyboardButton::callback("⚀ Угадать единицу", "game_guess_one")],
        ]);

        let text = "🎲 Выберите вариант игры:";

        bot.send_message(chat_id, text)
            .reply_markup(keyboard)
            .await?;
        
        Ok(())
    }

    /// Обработчик callback запросов
    async fn handle_callback(bot: Bot, callback: CallbackQuery) -> ResponseResult<()> {
        if let Some(data) = &callback.data {
            if let Some(message) = &callback.message {
                let chat_id = message.chat.id;
                
                match data.as_str() {
                    "game_even_odd" => {
                        Self::start_even_odd_game(&bot, chat_id).await?;
                    }
                    "game_high_low" => {
                        Self::start_high_low_game(&bot, chat_id).await?;
                    }
                    "game_exact" => {
                        Self::start_exact_number_game(&bot, chat_id).await?;
                    }
                    "game_guess_one" => {
                        Self::start_guess_one_game(&bot, chat_id).await?;
                    }
                    "choice_even" => {
                        Self::play_even_odd_game(&bot, chat_id, EvenOddChoice::Even).await?;
                    }
                    "choice_odd" => {
                        Self::play_even_odd_game(&bot, chat_id, EvenOddChoice::Odd).await?;
                    }
                    "choice_high" => {
                        Self::play_high_low_game(&bot, chat_id, HighLowChoice::High).await?;
                    }
                    "choice_low" => {
                        Self::play_high_low_game(&bot, chat_id, HighLowChoice::Low).await?;
                    }
                    "guess_one_yes" => {
                        Self::play_guess_one_game(&bot, chat_id, GuessOneChoice::Yes).await?;
                    }
                    "guess_one_no" => {
                        Self::play_guess_one_game(&bot, chat_id, GuessOneChoice::No).await?;
                    }
                    data if data.starts_with("number_") => {
                        if let Ok(number) = data[7..].parse::<u8>() {
                            if (1..=6).contains(&number) {
                                Self::play_exact_number_game(&bot, chat_id, number).await?;
                            }
                        }
                    }
                    _ => {
                        error!("Неизвестный callback: {}", data);
                    }
                }
            }
        }

        // Подтверждение callback запроса
        bot.answer_callback_query(callback.id).await?;
        Ok(())
    }

    /// Начало игры "Четное/Нечетное"
    async fn start_even_odd_game(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("🔵 Четное", "choice_even"),
                InlineKeyboardButton::callback("🔴 Нечетное", "choice_odd"),
            ],
        ]);

        let text = "🔵 <b>Игра: Четное/Нечетное</b>\n\n\
                   Выберите, будет ли результат броска четным или нечетным числом:";

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
        
        Ok(())
    }

    /// Начало игры "Больше/Меньше 3.5"
    async fn start_high_low_game(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("⬆️ Больше 3.5 (4-6)", "choice_high"),
                InlineKeyboardButton::callback("⬇️ Меньше 3.5 (1-3)", "choice_low"),
            ],
        ]);

        let text = "🔴 <b>Игра: Больше/Меньше 3.5</b>\n\n\
                   Выберите, будет ли результат больше или меньше 3.5:";

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
        
        Ok(())
    }

    /// Начало игры "Точное число"
    async fn start_exact_number_game(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("1️⃣", "number_1"),
                InlineKeyboardButton::callback("2️⃣", "number_2"),
                InlineKeyboardButton::callback("3️⃣", "number_3"),
            ],
            vec![
                InlineKeyboardButton::callback("4️⃣", "number_4"),
                InlineKeyboardButton::callback("5️⃣", "number_5"),
                InlineKeyboardButton::callback("6️⃣", "number_6"),
            ],
        ]);

        let text = "🎯 <b>Игра: Точное число</b>\n\n\
                   Выберите число от 1 до 6, которое выпадет на кубике:";

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
        
        Ok(())
    }

    /// Начало игры "Угадать единицу"
    async fn start_guess_one_game(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        let keyboard = InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback("✅ Да, выпадет единица", "guess_one_yes"),
                InlineKeyboardButton::callback("❌ Нет, не выпадет", "guess_one_no"),
            ],
        ]);

        let text = "⚀ <b>Игра: Угадать единицу</b>\n\n\
                   Выпадет ли на кубике единица?";

        bot.send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;
        
        Ok(())
    }

    /// Игра "Четное/Нечетное"
    async fn play_even_odd_game(bot: &Bot, chat_id: ChatId, choice: EvenOddChoice) -> ResponseResult<()> {
        let choice_text = match choice {
            EvenOddChoice::Even => "четное",
            EvenOddChoice::Odd => "нечетное",
        };

        // Отправляем сообщение о выборе пользователя
        bot.send_message(chat_id, format!("🎯 Вы выбрали: {}\n🎲 Бросаю кубик...", choice_text))
            .await?;

        // Отправляем анимированный кубик
        let dice_message = bot.send_dice(chat_id).await?;
        
        // Получаем результат кубика
        if let Some(dice) = dice_message.dice() {
            let dice_result = dice.value as u8;
            let is_win = DiceGame::check_even_odd(dice_result, choice.clone());
            let result_text = if dice_result % 2 == 0 { "четное" } else { "нечетное" };
            
            // Даем время для анимации кубика
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            
            let message = if is_win {
                format!(
                    "🎉 Число {}: {}\n\n{}",
                    dice_result,
                    result_text,
                    DiceGame::win_message()
                )
            } else {
                format!(
                    "😔 Число {}: {}\n\n{}",
                    dice_result,
                    result_text,
                    DiceGame::lose_message()
                )
            };

            bot.send_message(chat_id, message).await?;
        }
        
        // Предложение новой игры
        Self::offer_new_game(bot, chat_id).await
    }

    /// Игра "Больше/Меньше 3.5"
    async fn play_high_low_game(bot: &Bot, chat_id: ChatId, choice: HighLowChoice) -> ResponseResult<()> {
        let choice_text = match choice {
            HighLowChoice::High => "больше 3.5 (4-6)",
            HighLowChoice::Low => "меньше 3.5 (1-3)",
        };

        // Отправляем сообщение о выборе пользователя
        bot.send_message(chat_id, format!("📊 Вы выбрали: {}\n🎲 Бросаю кубик...", choice_text))
            .await?;

        // Отправляем анимированный кубик
        let dice_message = bot.send_dice(chat_id).await?;
        
        // Получаем результат кубика
        if let Some(dice) = dice_message.dice() {
            let dice_result = dice.value as u8;
            let is_win = DiceGame::check_high_low(dice_result, choice.clone());
            let result_text = if dice_result >= 4 { "больше 3.5" } else { "меньше 3.5" };
            
            // Даем время для анимации кубика
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            
            let message = if is_win {
                format!(
                    "🎉 Число {} - {}\n\n{}",
                    dice_result,
                    result_text,
                    DiceGame::win_message()
                )
            } else {
                format!(
                    "😔 Число {} - {}\n\n{}",
                    dice_result,
                    result_text,
                    DiceGame::lose_message()
                )
            };

            bot.send_message(chat_id, message).await?;
        }
        
        // Предложение новой игры
        Self::offer_new_game(bot, chat_id).await
    }

    /// Игра "Точное число"
    async fn play_exact_number_game(bot: &Bot, chat_id: ChatId, guess: u8) -> ResponseResult<()> {
        // Отправляем сообщение о выборе пользователя
        bot.send_message(chat_id, format!("🎯 Вы выбрали число: {}\n🎲 Бросаю кубик...", guess))
            .await?;

        // Отправляем анимированный кубик
        let dice_message = bot.send_dice(chat_id).await?;
        
        // Получаем результат кубика
        if let Some(dice) = dice_message.dice() {
            let dice_result = dice.value as u8;
            let is_win = DiceGame::check_exact_number(dice_result, guess);
            
            // Даем время для анимации кубика
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            
            let message = if is_win {
                format!(
                    "🎉 Выпало число: {}\nВы угадали!\n\n{}",
                    dice_result,
                    DiceGame::win_message()
                )
            } else {
                format!(
                    "😔 Выпало число: {}\nВы выбрали: {}\n\n{}",
                    dice_result,
                    guess,
                    DiceGame::lose_message()
                )
            };

            bot.send_message(chat_id, message).await?;
        }
        
        // Предложение новой игры
        Self::offer_new_game(bot, chat_id).await
    }

    /// Игра "Угадать единицу"
    async fn play_guess_one_game(bot: &Bot, chat_id: ChatId, choice: GuessOneChoice) -> ResponseResult<()> {
        let choice_text = match choice {
            GuessOneChoice::Yes => "да, выпадет единица",
            GuessOneChoice::No => "нет, не выпадет единица",
        };

        // Отправляем сообщение о выборе пользователя
        bot.send_message(chat_id, format!("⚀ Вы выбрали: {}\n🎲 Бросаю кубик...", choice_text))
            .await?;

        // Отправляем анимированный кубик
        let dice_message = bot.send_dice(chat_id).await?;
        
        // Получаем результат кубика
        if let Some(dice) = dice_message.dice() {
            let dice_result = dice.value as u8;
            let is_win = DiceGame::check_guess_one(dice_result, choice.clone());
            let result_text = if dice_result == 1 { "выпала единица" } else { "единица не выпала" };
            
            // Даем время для анимации кубика
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            
            let message = if is_win {
                format!(
                    "🎉 Число {}: {}\n\n{}",
                    dice_result,
                    result_text,
                    DiceGame::win_message()
                )
            } else {
                format!(
                    "😔 Число {}: {}\n\n{}",
                    dice_result,
                    result_text,
                    DiceGame::lose_message()
                )
            };

            bot.send_message(chat_id, message).await?;
        }
        
        // Предложение новой игры
        Self::offer_new_game(bot, chat_id).await
    }

    /// Предложение новой игры
    async fn offer_new_game(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        let text = "🎮 Хотите сыграть еще раз? Используйте /play для новой игры!";
        bot.send_message(chat_id, text).await?;
        
        Ok(())
    }

    /// Обработчик обычных сообщений
    async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
        if let Some(text) = msg.text() {
            match text.to_lowercase().as_str() {
                "играть" | "игра" | "кубик" | "кубики" => {
                    Self::show_game_selection(&bot, msg.chat.id).await?;
                }
                "привет" | "hello" | "hi" => {
                    bot.send_message(msg.chat.id, "🎲 Привет! Хотите сыграть в кубики? Используйте /play")
                        .await?;
                }
                _ => {
                    bot.send_message(
                        msg.chat.id,
                        "🤔 Не понимаю команду. Используйте /help для получения справки или /play для игры."
                    ).await?;
                }
            }
        }
        
        Ok(())
    }
}
