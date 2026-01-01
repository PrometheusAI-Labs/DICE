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
