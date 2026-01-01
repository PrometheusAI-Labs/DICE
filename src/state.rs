use serde::{Deserialize, Serialize};

/// Состояния диалога с пользователем
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DialogueState {
    /// Начальное состояние
    Start,
    /// Пользователь выбирает тип игры
    SelectingGameType,
    /// Игра "Четное/Нечетное"
    EvenOddGame,
    /// Игра "Больше/Меньше 3.5"
    HighLowGame,
    /// Игра "Точное число"
    ExactNumberGame,
    /// Ожидание выбора числа (1-6)
    WaitingForNumber,
}

impl Default for DialogueState {
    fn default() -> Self {
        DialogueState::Start
    }
}

/// Типы игр
#[derive(Clone, Debug, PartialEq)]
pub enum GameType {
    EvenOdd,     // Четное/Нечетное
    HighLow,     // Больше/Меньше 3.5
    ExactNumber, // Точное число
    GuessOne,    // Угадать единицу
}

/// Выбор пользователя в игре "Четное/Нечетное"
#[derive(Clone, Debug, PartialEq)]
pub enum EvenOddChoice {
    Even, // Четное
    Odd,  // Нечетное
}

/// Выбор пользователя в игре "Больше/Меньше 3.5"
#[derive(Clone, Debug, PartialEq)]
pub enum HighLowChoice {
    High, // Больше 3.5 (4-6)
    Low,  // Меньше 3.5 (1-3)
}

/// Выбор пользователя в игре "Угадать единицу"
#[derive(Clone, Debug, PartialEq)]
pub enum GuessOneChoice {
    Yes, // Выпадет единица
    No,  // Не выпадет единица
}
