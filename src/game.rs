use crate::state::{EvenOddChoice, GuessOneChoice, HighLowChoice};
use rand::Rng;

/// Структура для управления игровой логикой
pub struct DiceGame;

impl DiceGame {
    /// Проверка результата для игры "Четное/Нечетное"
    pub fn check_even_odd(dice_result: u8, user_choice: EvenOddChoice) -> bool {
        let is_even = dice_result % 2 == 0;
        match user_choice {
            EvenOddChoice::Even => is_even,
            EvenOddChoice::Odd => !is_even,
        }
    }

    /// Проверка результата для игры "Больше/Меньше 3.5"
    pub fn check_high_low(dice_result: u8, user_choice: HighLowChoice) -> bool {
        match user_choice {
            HighLowChoice::High => dice_result >= 4,
            HighLowChoice::Low => dice_result <= 3,
        }
    }

    /// Проверка результата для игры "Точное число"
    pub fn check_exact_number(dice_result: u8, user_guess: u8) -> bool {
        dice_result == user_guess
    }

    /// Проверка результата для игры "Угадать единицу"
    pub fn check_guess_one(dice_result: u8, user_choice: GuessOneChoice) -> bool {
        let is_one = dice_result == 1;
        match user_choice {
            GuessOneChoice::Yes => is_one,
            GuessOneChoice::No => !is_one,
        }
    }

    /// Получение сообщения о выигрыше
    pub fn win_message() -> &'static str {
        let messages = [
            "🎉 Поздравляю! Вы угадали!",
            "🎊 Отлично! Правильный ответ!",
            "✨ Великолепно! Вы победили!",
            "🏆 Браво! Точное попадание!",
            "🎯 Превосходно! Вы угадали!",
        ];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..messages.len());
        messages[index]
    }

    /// Получение ободряющего сообщения при проигрыше
    pub fn lose_message() -> &'static str {
        let messages = [
            "😔 Не угадали, но не расстраивайтесь!",
            "🎲 В этот раз не повезло, попробуйте еще!",
            "💪 Ничего страшного, удача улыбнется в следующий раз!",
            "🌟 Не переживайте, у вас все получится!",
            "🎮 Попытка не пытка, играем еще!",
        ];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..messages.len());
        messages[index]
    }

    /// Сравнение результатов бросков кубиков
    pub fn compare_dices(bot_dice: u8, user_dice: u8) -> &'static str {
        if bot_dice > user_dice {
            "🤖 Компьютер победил!"
        } else if user_dice > bot_dice {
            "🎉 Пользователь победил!"
        } else {
            "🤝 Ничья!"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{EvenOddChoice, GuessOneChoice, HighLowChoice};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_check_even_odd_basic() {
        assert!(DiceGame::check_even_odd(2, EvenOddChoice::Even));
        assert!(!DiceGame::check_even_odd(2, EvenOddChoice::Odd));
        assert!(DiceGame::check_even_odd(5, EvenOddChoice::Odd));
        assert!(!DiceGame::check_even_odd(5, EvenOddChoice::Even));
    }

    #[test]
    fn test_check_high_low_basic() {
        assert!(DiceGame::check_high_low(1, HighLowChoice::Low));
        assert!(DiceGame::check_high_low(3, HighLowChoice::Low));
        assert!(!DiceGame::check_high_low(3, HighLowChoice::High));
        assert!(DiceGame::check_high_low(4, HighLowChoice::High));
        assert!(DiceGame::check_high_low(6, HighLowChoice::High));
        assert!(!DiceGame::check_high_low(4, HighLowChoice::Low));
    }

    #[test]
    fn test_check_exact_number_basic() {
        assert!(DiceGame::check_exact_number(4, 4));
        assert!(!DiceGame::check_exact_number(1, 6));
    }

    #[test]
    fn test_check_guess_one_basic() {
        assert!(DiceGame::check_guess_one(1, GuessOneChoice::Yes));
        assert!(!DiceGame::check_guess_one(1, GuessOneChoice::No));
        assert!(DiceGame::check_guess_one(3, GuessOneChoice::No));
        assert!(!DiceGame::check_guess_one(3, GuessOneChoice::Yes));
    }

    #[test]
    fn test_compare_dices_bot_wins() {
        assert_eq!(DiceGame::compare_dices(5, 3), "🤖 Компьютер победил!");
    }

    #[test]
    fn test_compare_dices_user_wins() {
        assert_eq!(DiceGame::compare_dices(2, 4), "🎉 Пользователь победил!");
    }

    #[test]
    fn test_compare_dices_tie() {
        assert_eq!(DiceGame::compare_dices(3, 3), "🤝 Ничья!");
    }

    mod properties {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn even_odd_property(dice_result in 1u8..=6u8) {
                let is_even = dice_result % 2 == 0;
                prop_assert_eq!(DiceGame::check_even_odd(dice_result, EvenOddChoice::Even), is_even);
                prop_assert_eq!(DiceGame::check_even_odd(dice_result, EvenOddChoice::Odd), !is_even);
            }

            #[test]
            fn high_low_property(dice_result in 1u8..=6u8) {
                let is_high = dice_result >= 4;
                prop_assert_eq!(DiceGame::check_high_low(dice_result, HighLowChoice::High), is_high);
                prop_assert_eq!(DiceGame::check_high_low(dice_result, HighLowChoice::Low), !is_high);
            }

            #[test]
            fn exact_number_property(dice_result in 1u8..=6u8) {
                prop_assert!(DiceGame::check_exact_number(dice_result, dice_result));
                let other = if dice_result == 6 { 1 } else { dice_result + 1 };
                prop_assert!(!DiceGame::check_exact_number(dice_result, other));
            }
        }
    }
}
