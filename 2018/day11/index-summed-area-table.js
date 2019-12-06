
/* Advent of Code 2018 Day 11 (https://adventofcode.com/2018/day/11)

All of the puzzle flavour text aside, what we are looking for here is the
largest square sub-matrix (ie., the sum of the elements in the sub-matrix) in a
300x300 matrix. Q1 wanted the largest 3x3 matrix and I just wrote a brute force
loop. Q2 wanted the square sub-matrix with the largest sum.

I knew there had to be a more efficienct way to sovle this beyond brute force
and after a bit of research I found Summed-area Tables
(https://en.wikipedia.org/wiki/Summed-area_table). I liked this explanation:
https://www.codeproject.com/Articles/441226/Haar-feature-Object-Detection-in-Csharp

*/

const calc_power_lvl = (serial_num, x , y) => {
    const rack_id = x + 10;
    var pwr_lvl = (rack_id * y + serial_num) * rack_id;
    if (Math.abs(pwr_lvl) < 100)
        return -5;
    else
        return Math.floor((pwr_lvl % 1000) / 100) - 5;
};

/* So what we are doing is this: y, x (y is row, x is column) form the bottom
    right corner of the sub-matrix we are checking and its length/width is size.

    So:
        4  -1   7   0
        3  -2  -4   5
        5   0   0   1
        1  -1  -1   2

    If we want the sum of the 2x2 matrix comprising the bottom right corner,
    we take m[y, x] which is 2. We want to subtract m[y - 2, x], which -1 and
    m[y, x - 2], which is 5, THEN add m[y - 2, x - 2]. Basically, we add
    the two matrix sub-totals comprising the cells we want to ignore. However,
    we've subtracted their intersection twice, so we need to add them back. Ie:

        C   C    A   A
        C   C    A   A
        B   B    0   1
        B   B   -1   2

    The cells marked C are where the cells overlap and were double counted.

    Of course, if we have a situation were we want the 2x2 matrix whose lower
    right corner is 1, 3, we only need to subtract m[y, x - 2] from its total.
*/
const sum_of_matrix = (sub_sums, x, y, size) => {
    var sum = sub_sums[y][x];
    const ya = y - size;
    const xb = x - size;
    if (ya >= 0) sum -= sub_sums[ya][x];
    if (xb >= 0) sum -= sub_sums[y][xb];
    if (ya >= 0 && xb >= 0) sum += sub_sums[ya][xb];

    return sum;
}

const max_sum_of_size_n = (sub_sums, matrix_size, size) => {
    var highest_sum = 0;
    var X = -1, Y = -1;
    const start = size - 1;
    for (let r = start; r < matrix_size; r++) {
        for (let c = start; c < matrix_size; c++) {
            s = sum_of_matrix(sub_sums, c, r, size);
            if (s > highest_sum) {
                highest_sum = s;
                X = c;
                Y = r;
            }
        }
    }

    return { max:highest_sum, X:X - size + 1, Y:Y - size + 1, size:size };
}

const serial_num = 6042;
const matrix_size = 300;

const sub_sums = [];
for (let r = 0; r < matrix_size; r++) {
    let row = [];
    let row_sum = 0;
    for (let c = 0; c < matrix_size; c++) {
        row_sum += calc_power_lvl(serial_num, c, r);
        row.push(r == 0 ? row_sum : row_sum + sub_sums[r-1][c]);
    }
    sub_sums.push(row);
}

var res = max_sum_of_size_n(sub_sums, matrix_size, 3)
console.log(`Q1: ${res.max} at X:${res.X} Y:${res.Y}`);

var max_res = res;
for (let s = 4; s <= matrix_size; s++) {
    res = max_sum_of_size_n(sub_sums, matrix_size, s);
    if (res.max > max_res.max)
        max_res = res;
}
console.log(`Q2: ${max_res.X},${max_res.Y},${max_res.size}`);
