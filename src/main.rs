//----------------------------------------------------//
//----     Перечисляем основные определения     ------//
//----------------------------------------------------//

fn main() {

    let is_rust_cool: bool   = true;    // Логическое значение true
    let is_open_source: bool = false;   // Логическое значение false

    let char_type: char      = 'A';     // Символ
    let string_type: &str    = "Hello, Rust!"; // Строка (неизменяемая)

    let     a  = 10;               // Неизменяемая переменная
    let mut b  = 3;                // Изменяемая переменная
    let float: f32  = 3.14159265359;    // 32-битное число с плавающей точкой
    let double: f64 = 2.71828182845;    // 64-битное число с плавающей точкой (по умолчанию)

    let sum        = a + b;        // Сложение
    let difference = a - b;        // Вычитание
    let product    = a * b;        // Умножение
    let quotient   = a / b;        // Деление
    let remainder  = a % b;        // Остаток от деления




    // Создание массива строк
    let colors: [&str; 3] = ["Red", "Green", "Blue"];

    // Доступ к элементам массива
    let first_color = colors[0];

    // Итерация по массиву
    for color in &colors {
        println!("Цвет: {}", color);
    }

    // Размер массива (известен на этапе компиляции)
    let length = colors.len();

    let age = 25;

    if age < 18 {
        println!("Вы несовершеннолетний");
    } else if age >= 18 && age < 65 {
        println!("Вы взрослый");
    } else {
        println!("Вы пенсионер");
    }

        // Цикл for
    for i in 1..=5 {
        println!("Итерация {}", i);
    }

    // Цикл while
    let mut counter = 0;
    
    while counter < 3 {
        println!("Повторение {}", counter);
        counter += 1;
    }

    let language = "Rust";

    match language {
        "Rust" => println!("Вы выбрали Rust - отличный выбор!"),
        "Python" | "JavaScript" => println!("Тоже хорошие языки!"),
        _ => println!("Неизвестный язык программирования"),
    }

    // Вызов функции с возвращаемым значением
    let result = add(3, 5);

    greet(result);

}

// Объявление функции без возвращаемого значения
fn greet(result: i32) {
    println!("Сумма, {}!", result);
}

// Объявление функции с возвращаемым значением
fn add(a: i32, b: i32) -> i32 {
    a + b
}