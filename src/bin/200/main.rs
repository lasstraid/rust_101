fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn bfs(row: usize, column: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;

        visited[row][column] = true;

        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push((row, column));

        let mut index: usize = 0;

        let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while index < queue.len() {
            let (r0, c0) = queue[index];
            index += 1;

            for (dr, dc) in directions {
                let r1: i32 = r0 as i32 + dr;
                let c1: i32 = c0 as i32 + dc;

                if r1 >= 0
                    && r1 < n
                    && c1 >= 0
                    && c1 < m
                    && grid[r1 as usize][c1 as usize] == '1'
                    && !visited[r1 as usize][c1 as usize]
                {
                    visited[r1 as usize][c1 as usize] = true;
                    queue.push((r1 as usize, c1 as usize));
                }
            }
        }
    }

    let mut answer: i32 = 0;
    let mut visited: Vec<Vec<bool>> = (0..grid.len())
        .map(|_| vec![false; grid[0].len()])
        .collect();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                bfs(i, j, &grid, &mut visited);
                answer += 1;
            }
        }
    }

    return answer;
}

fn main() {}
