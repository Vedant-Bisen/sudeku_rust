fn empty_cell(sudeku: &[[u32; 9]; 9]) -> (usize, usize) {
    for i in 0..9 {
        for j in 0..9 {
            if sudeku[i][j] == 0 {
                // println!("Empty cell found at ({},{})", i, j);
                return (i, j);
            }
        }
    }
    (10, 10)
}

fn valid(sudeku: &[[u32; 9]; 9], num: u32, pos: (usize, usize)) -> bool {
    //Checking rows
    for i in 0..9 {
        if sudeku[pos.0][i] == num && pos.1 != i {
            return false;
        }
    }
    //Checking columns
    for i in 0..9 {
        if sudeku[i][pos.1] == num && pos.0 != i {
            return false;
        }
    }
    //Checking boxes
    let box_x = pos.1 / 3;
    let box_y = pos.0 / 3;
    for i in box_y * 3..box_y * 3 + 3 {
        for j in box_x * 3..box_x * 3 + 3 {
            if sudeku[i][j] == num && (i, j) != pos {
                return false;
            }
        }
    }
    return true;
}

fn solve(mut sudeku: [[u32; 9]; 9],mut count:i32) -> [[u32; 9]; 9] {
    let (i, j) = empty_cell(&sudeku);
    count += 1;
    if i == 10 && j == 10 {
        println!("{:?}", sudeku);
        println!("Solved");
        return sudeku;
    }
    // println!("Empty cell found at ({},{})",i,j);
    for num in 1..10 {
        if valid(&sudeku, num, (i, j)) {
            sudeku[i][j] = num;
            solve(sudeku,count);
        } 
    }
    return sudeku;
}
fn main() {
    let sudeku: [[u32; 9]; 9] = [
        [4, 0, 0, 0, 5, 6, 0, 9, 0],
        [0, 5, 0, 4, 0, 0, 0, 0, 0],
        [0, 6, 0, 3, 8, 0, 0, 4, 5],
        [0, 0, 6, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 2, 0],
        [3, 0, 0, 0, 1, 0, 8, 0, 0],
        [0, 0, 0, 0, 4, 0, 7, 0, 8],
        [8, 0, 0, 7, 0, 3, 2, 6, 0],
        [0, 0, 7, 8, 0, 5, 0, 3, 4],
    ];
    let count = 0;
    solve(sudeku,0);
}
