use std::io;

/* This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

/*          ---------
            |       |
            |   1   |
    -------------------------
    |       |       |       |
    |   2   |   5   |   4   |
    -------------------------
            |       |
            |   3   |
            ---------
*/

// 1 = 5
// 2 = 5
// 3 = 5
// 4 = 5
// 5 = 1, 2, 3, 4

fn check_cells(number: i32, grid: &mut Vec<Vec<i32>>) -> bool {         // Проверка клетки на сопоставимость с соседними клетками
    if number >= 1 && number <= 4 {
        if grid[(number - 1) as usize] == grid[4] {return false} 
        else {return true}
    }
    else if (number - 1) == 5 {
        for n in 0..4 {
            if grid[4] == grid[n] {return false} else {return true};
        }
    }
    return false;
}

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![vec![0;8]; 5];                   // Задаём сетку в вектор
    let mut i = 0;                                                      // Определяем итераторы     
    let mut j = 0;
    loop {                                                              // Создаём главный бесконечный массив
        let mut grid_cells_remaining = vec![1, 2, 3, 4, 5];             // Вектор оставшихся значений
        println!("Enter grid number [or exit]: ");                      // Запрос на номер клетки
        let mut number_of_cell = String::new();                         // Инициализируем переменную типа String
        io::stdin()                                                     // Запрашиваем ввод из клавиатуры
            .read_line(&mut number_of_cell)                             // Указываем мутабельную переменную
            .expect("Failed to read line!");                            // Выполняем если условие не удовлетворено
        if number_of_cell == "exit" {break;};                           // Выход из программы если введена команда "exit"
        let number_of_cell: i32 = number_of_cell.parse().unwrap();      // Приводим переменную number_of_cell к типу i32
        while j < 8 {                                                   // Создаём цикл заполнения одной клетки
            println!("Enter value: ");                                  // Выводим на экран запрос о вводе числа
            let mut value_of_cell = String::new();
            io::stdin()
                .read_line(&mut value_of_cell)
                .expect("Failed to read line!");
            let value_of_cell = value_of_cell.parse::<i32>().unwrap();  //

            grid[number_of_cell as usize][i] = value_of_cell;           // Присваиваем значение к соответствующей ячейке в векторе

            if check_cells(number_of_cell, &mut grid) {continue}        // Проверяем число на соблюдение условия      
            else {println!("Число не удовлетворяет условию!")}                    
            j = j + 1;
        }
        grid_cells_remaining.remove((number_of_cell - 1) as usize);     // Удаляем заполненные клетки
        println!("Cells remain: {:?}", grid_cells_remaining);           // Выводим оставшиеся на экран
        i = i + 1;
    }

} 